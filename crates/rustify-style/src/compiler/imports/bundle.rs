//! Owned native fragments, prepared by the same reaction machine used for replay.
use super::super::public_api::{
    InputMutation, PublicAst, PublicValue, Source, Stylesheet, StylesheetLoad,
};
use super::{
    directory, normalize, parameters, schedule::*, wrapper, AstNode, CompileError, Limits,
    PublicSetup,
};
use std::{collections::BTreeMap, sync::Arc};

struct Fragment {
    nodes: Vec<AstNode>,
    base: String,
    recursion: usize,
    parent: Option<PublicAst>,
}
struct Site<'a> {
    fragment: usize,
    path: Vec<usize>,
    parent: Option<PublicAst>,
    index: usize,
    original: AstNode,
    import: super::Import,
    asset: Option<&'a Stylesheet>,
    context: Option<PublicValue>,
    loaded: Option<usize>,
}
struct Bundle<'a, 'b> {
    assets: BTreeMap<String, &'a Stylesheet>,
    parsed: BTreeMap<String, Vec<AstNode>>,
    limits: &'a Limits,
    expanded_bytes: usize,
    has_import: bool,
    calls: Vec<StylesheetLoad>,
    track: bool,
    public: Option<&'b mut dyn PublicSetup>,
    fragments: Vec<Fragment>,
    sites: Vec<Site<'a>>,
    effects: Vec<InputMutation>,
}
impl Bundle<'_, '_> {
    fn scope(
        &mut self,
        program: &mut Program,
        machine: &mut Machine,
        fragment: Fragment,
    ) -> Result<usize, CompileError> {
        let scope = program.scopes.len();
        program.scopes.push(Scope {
            returned: machine.promise()?,
            all: machine.promise()?,
            members: vec![],
            error: None,
        });
        self.fragments.push(fragment);
        let nodes = std::mem::take(&mut self.fragments[scope].nodes);
        let parent = self.fragments[scope].parent.clone();
        let result = self.visit(program, machine, scope, &nodes, parent, &mut vec![]);
        self.fragments[scope].nodes = nodes;
        match result {
            Err(error @ CompileError::Limit { .. }) => return Err(error),
            Err(error) => program.scopes[scope].error = Some(error),
            Ok(()) => {}
        }
        Ok(scope)
    }
    fn visit(
        &mut self,
        program: &mut Program,
        machine: &mut Machine,
        scope: usize,
        nodes: &[AstNode],
        parent: Option<PublicAst>,
        path: &mut Vec<usize>,
    ) -> Result<(), CompileError> {
        let mut public_index = 0;
        for (native_index, node) in nodes.iter().enumerate() {
            if let (Some(parent), Some(origin)) = (&parent, node.origin()) {
                while public_index < parent.len()
                    && parent[public_index]
                        .as_object()
                        .map(|object| Arc::as_ptr(object.identity()) as usize)
                        != origin.object_id()
                {
                    public_index += 1;
                }
            }
            let index = public_index;
            public_index += 1;
            path.push(native_index);
            if let AstNode::AtRule { name, params, .. } = node {
                if name == "@import" || name == "@reference" {
                    if let Some(mut import) = parameters(params, self.limits.ast_depth)? {
                        self.has_import = true;
                        if name == "@reference" {
                            import.media = Some("reference".into());
                        }
                        if !import.uri.starts_with("data:")
                            && !import.uri.starts_with("http://")
                            && !import.uri.starts_with("https://")
                        {
                            let fragment = &self.fragments[scope];
                            super::super::check_limit(
                                fragment.recursion,
                                self.limits.ast_depth,
                                "ast_depth",
                            )?;
                            let launch_error = (fragment.recursion > 100).then(|| CompileError::Syntax {
                                offset: 0,
                                message: format!("Exceeded maximum recursion depth while resolving `{}` in `{}`)", import.uri, fragment.base).into(),
                            });
                            if launch_error.is_none() {
                                self.calls.push(StylesheetLoad {
                                    id: import.uri.clone(),
                                    base: fragment.base.clone(),
                                });
                            }
                            let joined = if import.uri.starts_with('/') {
                                normalize(&import.uri)
                            } else {
                                normalize(&format!("{}/{}", fragment.base, import.uri))
                            };
                            let asset = self
                                .assets
                                .get(&joined)
                                .or_else(|| self.assets.get(&normalize(&import.uri)))
                                .copied();
                            let loader_error =
                                asset.is_none().then(|| CompileError::MissingAsset {
                                    id: import.uri.clone(),
                                    base: fragment.base.clone(),
                                });
                            let task = program.tasks.len();
                            super::super::check_limit(
                                (task + 1).saturating_mul(
                                    2 * (std::mem::size_of::<Task>()
                                        + std::mem::size_of::<Site<'_>>()),
                                ),
                                self.limits.cache_bytes,
                                "cache_bytes",
                            )?;
                            program.tasks.push(Task {
                                returned: machine.promise()?,
                                launch_error,
                                loader_error,
                                loaded: None,
                                begin: None,
                                complete: None,
                            });
                            program.scopes[scope].members.push(task);
                            self.sites.push(Site {
                                fragment: scope,
                                path: path.clone(),
                                parent: parent.clone(),
                                index,
                                original: node.clone(),
                                import,
                                asset,
                                context: None,
                                loaded: None,
                            });
                            path.pop();
                            continue;
                        }
                    }
                }
            }
            let child_parent = node
                .origin()
                .and_then(|origin| origin.fields().get("nodes"))
                .and_then(|value| match value {
                    PublicValue::Array(nodes) => Some(nodes.clone()),
                    _ => None,
                });
            if let Some(children) = node.children() {
                self.visit(program, machine, scope, children, child_parent, path)?;
            }
            path.pop();
        }
        Ok(())
    }
}
impl Host for Bundle<'_, '_> {
    fn begin(
        &mut self,
        program: &mut Program,
        _machine: &mut Machine,
        task: usize,
    ) -> Result<Settlement, CompileError> {
        let site = &mut self.sites[task];
        if let (Some(public), Some(parent)) = (&mut self.public, &site.parent) {
            let (context, effect) = public.begin(parent, site.index)?;
            site.context = Some(context);
            program.tasks[task].begin = Some(effect.clone());
            self.effects.push(effect);
        }
        Ok(Ok(()))
    }
    fn loaded(
        &mut self,
        program: &mut Program,
        machine: &mut Machine,
        task: usize,
    ) -> Result<Result<usize, Fault>, CompileError> {
        let site = &self.sites[task];
        // Only successful loader settlements reach this callback.
        let Some(asset) = site.asset else {
            return Ok(Err(CompileError::MissingAsset {
                id: site.import.uri.clone(),
                base: self.fragments[site.fragment].base.clone(),
            }
            .into()));
        };
        let css = &asset.content;
        self.expanded_bytes = self.expanded_bytes.saturating_add(css.len());
        super::super::check_limit(
            self.expanded_bytes,
            self.limits
                .input_bytes
                .saturating_add(self.limits.asset_bytes),
            "expanded_bytes",
        )?;
        let path = normalize(&asset.id);
        let parsed = if let Some(ast) = self.parsed.get(&path).filter(|_| self.public.is_none()) {
            Ok(ast.clone())
        } else {
            let file = asset.path.as_ref().unwrap_or(&asset.id);
            let source = (self.track && !file.is_empty()).then(|| {
                Arc::new(Source {
                    file: Some(file.clone()),
                    code: css.clone(),
                })
            });
            program.sources.extend(source.iter().cloned());
            super::parser::parse_with_source(css, self.limits.ast_depth, source)
        };
        let mut ast = match parsed {
            Ok(ast) => ast,
            Err(error @ CompileError::Limit { .. }) => return Err(error),
            Err(error) => {
                program.tasks[task].loaded = Some(Err(error.clone()));
                return Ok(Err(error.into()));
            }
        };
        if self.public.is_none() {
            self.parsed.entry(path).or_insert_with(|| ast.clone());
        }
        let parent = self
            .public
            .as_mut()
            .map(|public| public.parsed(&mut ast))
            .transpose()?;
        let fragment = Fragment {
            nodes: ast,
            base: asset
                .base
                .as_deref()
                .unwrap_or_else(|| directory(&asset.id))
                .into(),
            recursion: self.fragments[site.fragment].recursion + 1,
            parent,
        };
        let scope = self.scope(program, machine, fragment)?;
        self.sites[task].loaded = Some(scope);
        program.tasks[task].loaded = Some(Ok(scope));
        Ok(Ok(scope))
    }
    fn complete(&mut self, program: &mut Program, task: usize) -> Result<Settlement, CompileError> {
        let site = &mut self.sites[task];
        let Some(loaded) = site.loaded else {
            return Ok(Ok(()));
        };
        let fragment = &mut self.fragments[loaded];
        let mut imported = vec![AstNode::Context {
            origin: None,
            base: fragment.base.as_str().into(),
            source_base: None,
            reference: false,
            nodes: std::mem::take(&mut fragment.nodes),
        }];
        let source = match &site.original {
            AstNode::AtRule { source, .. } => source.clone(),
            _ => None,
        };
        if let Some(layer) = site.import.layer.take() {
            imported = wrapper("@layer", layer, imported, source.clone());
        }
        if let Some(media) = site.import.media.take() {
            imported = wrapper("@media", media, imported, source.clone());
        }
        if let Some(supports) = site.import.supports.take() {
            let supports = if supports.starts_with("(") {
                supports
            } else {
                let mut wrapped = crate::text::Text::from("(");
                wrapped.push_text(&supports);
                wrapped.push_str(")");
                wrapped
            };
            imported = wrapper("@supports", supports, imported, source);
        }
        let mut replacement = if let (Some(public), Some(parent), Some(loaded), Some(context)) = (
            &mut self.public,
            &site.parent,
            &fragment.parent,
            site.context.take(),
        ) {
            let mut outer = AstNode::Context {
                origin: None,
                base: crate::text::Text::default(),
                source_base: None,
                reference: false,
                nodes: imported,
            };
            let effect = public.complete(
                parent,
                site.index,
                &site.original,
                &mut outer,
                loaded,
                context,
            )?;
            program.tasks[task].complete = Some(effect.clone());
            self.effects.push(effect);
            outer
        } else {
            imported.remove(0)
        };
        let mut nodes = &mut self.fragments[site.fragment].nodes;
        for (depth, index) in site.path.iter().copied().enumerate() {
            if depth + 1 == site.path.len() {
                std::mem::swap(&mut nodes[index], &mut replacement);
                break;
            }
            if let Some(children) = nodes[index].children_mut() {
                nodes = children;
            } else {
                break;
            }
        }
        Ok(Ok(()))
    }
}
pub(super) fn expand(
    nodes: &mut Vec<AstNode>,
    base: &str,
    assets: &[Stylesheet],
    limits: &Limits,
    track: bool,
    public: Option<&mut dyn PublicSetup>,
) -> Result<super::Expansion, CompileError> {
    let parent = public.as_ref().map(|public| public.root());
    let mut bundle = Bundle {
        assets: assets
            .iter()
            .map(|asset| (normalize(&asset.id), asset))
            .collect(),
        parsed: BTreeMap::new(),
        limits,
        expanded_bytes: 0,
        has_import: false,
        calls: vec![],
        track,
        public,
        fragments: vec![],
        sites: vec![],
        effects: vec![],
    };
    let mut machine = Machine::new(limits);
    let mut program = Program::new(&mut machine)?;
    bundle.scope(
        &mut program,
        &mut machine,
        Fragment {
            nodes: std::mem::take(nodes),
            base: base.into(),
            recursion: 0,
            parent,
        },
    )?;
    if program.tasks.is_empty() && program.scopes[0].error.is_none() {
        *nodes = std::mem::take(&mut bundle.fragments[0].nodes);
        return Ok(super::Expansion {
            has_import: bundle.has_import,
            calls: bundle.calls,
        });
    }
    machine.start(&mut program, &mut bundle, Boundary::Observed)?;
    let mut observed = None;
    let mut prefix_length = 0;
    while let Some(job) = machine.pop() {
        if let Some(outcome) = machine.drive(&mut program, &mut bundle, job)? {
            prefix_length = bundle.effects.len();
            observed = Some(outcome);
        }
    }
    program.promise_count = machine.promise_count();
    program.prefix_length = prefix_length;
    super::super::check_limit(program.owned_bytes(), limits.cache_bytes, "cache_bytes")?;
    for fragment in &bundle.fragments {
        super::super::ast::validate_depth(&fragment.nodes, limits.ast_depth)?;
    }
    *nodes = std::mem::take(&mut bundle.fragments[0].nodes);
    if let Some(public) = &mut bundle.public {
        let prefix = bundle.effects.drain(..prefix_length).collect();
        public.scheduled(program, prefix);
    }
    if let Some(Err(Fault::Native(error))) = observed {
        return Err((*error).clone());
    }
    Ok(super::Expansion {
        has_import: bundle.has_import,
        calls: bundle.calls,
    })
}
