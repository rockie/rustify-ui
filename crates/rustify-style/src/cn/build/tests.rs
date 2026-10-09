use super::*;

#[test]
fn project_build_reads_sources_and_writes_unchanged_output() {
    let tree = BuildSourceTree::virtual_files([(
        "/project/src/view.html",
        "<div class=\"p-2 p-4\"></div>",
    )]);
    let options = ProjectOptions {
        cwd: Some("/project".into()),
        css: CssInput::Disabled,
        ..Default::default()
    };
    let first = build(&tree, &options, None).unwrap();
    assert_eq!(
        first.artifact.tokens,
        ["div", "class=", "p-2", "p-4", "/div"]
    );
    assert_eq!(first.scanned_files, 1);
    assert!(!build(&tree, &options, None).unwrap().artifact.changed);
}

use super::super::{merge_configs, validators, ClassGroupDef, Cn, Config, ConfigExtension};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::rc::Rc;
fn digest(text: &str) -> String {
    format!("{:x}", Sha256::digest(text.as_bytes()))
}
fn definitions(defs: &[ClassGroupDef]) -> Value {
    Value::Array(
        defs.iter()
            .map(|def| match def {
                ClassGroupDef::Literal(s) => json!(s),
                ClassGroupDef::Builtin(op) => json!({"$v":validators::NAMES[*op]}),
                ClassGroupDef::Theme(s) => json!({"$t":s}),
                ClassGroupDef::Object(map) => group_map(map),
                ClassGroupDef::Validator(_) | ClassGroupDef::ThemeGetter(_) => {
                    panic!("callback is not a frozen marker")
                }
            })
            .collect(),
    )
}
fn group_map(map: &[(String, Vec<ClassGroupDef>)]) -> Value {
    Value::Object(
        map.iter()
            .map(|(key, defs)| (key.clone(), definitions(defs)))
            .collect(),
    )
}
fn string_map(map: &[(String, Vec<String>)]) -> Value {
    Value::Object(
        map.iter()
            .map(|(key, values)| (key.clone(), json!(values)))
            .collect(),
    )
}
fn config_json(config: &Config) -> Value {
    let mut obj = serde_json::Map::new();
    obj.insert("theme".into(), group_map(&config.theme));
    obj.insert("classGroups".into(), group_map(&config.class_groups));
    obj.insert(
        "conflictingClassGroups".into(),
        string_map(&config.conflicting_class_groups),
    );
    obj.insert(
        "conflictingClassGroupModifiers".into(),
        string_map(&config.conflicting_class_group_modifiers),
    );
    obj.insert(
        "orderSensitiveModifiers".into(),
        json!(config.order_sensitive_modifiers),
    );
    obj.insert(
        "postfixLookupClassGroups".into(),
        json!(config.postfix_lookup_class_groups),
    );
    if let Some(prefix) = &config.prefix {
        obj.insert("prefix".into(), json!(prefix));
    }
    Value::Object(obj)
}
fn config_hash(config: &Config) -> String {
    digest(&(serde_json::to_string_pretty(&config_json(config)).unwrap() + "\n"))
}
fn fixture_tree() -> BuildSourceTree {
    BuildSourceTree::virtual_files([
        (
            "/fixture/src/view.html",
            "<div class=\"p-2 p-4 text-lg/7 leading-6 custom\"></div>",
        ),
        ("/fixture/tokens.txt", "p-2 p-4 hover:p-2 hover:p-4"),
        ("/fixture/safelist.txt", "shadow-card shadow-lg"),
        (
            "/fixture/theme.css",
            "@import \"./nested.css\"; @theme { --radius-*: initial; --radius-card: 3px; --text-hero: 24px; }",
        ),
        (
            "/fixture/nested.css",
            "@theme { --shadow-card: 0 1px black; }",
        ),
        (
            "/fixture/config.mjs",
            "export default {extend:{classGroups:{display:[\"custom-display\"]}}}",
        ),
        (
            "/fixture/bad-config.mjs",
            "invalid validator fixture module",
        ),
    ])
}
fn fixture_loader() -> ConfigLoader {
    Rc::new(|_, path, base| {
        let ext = if path.ends_with("bad-config.mjs") {
            json!({"extend":{"classGroups":{"probe":[{"probe":[{"$v":"unknown"}]}]}}})
        } else {
            json!({"extend":{"classGroups":{"display":["custom-display"]}}})
        };
        Ok(merge_configs(&base, &ConfigExtension::from_json(&ext)?))
    })
}
fn fixture_options(case: &Value) -> ProjectOptions {
    let input = &case["options"];
    let id = case["id"].as_str().unwrap();
    ProjectOptions {
        cwd: Some("/fixture".into()),
        out: input["out"]
            .as_str()
            .map(str::to_owned)
            .unwrap_or_else(|| format!("tables-{id}.mjs")),
        content: input["content"]
            .as_array()
            .map(|array| array.iter().map(|p| p.as_str().unwrap().into()).collect())
            .unwrap_or_default(),
        tokens: input["tokens"].as_str().map(str::to_owned),
        config: input["config"].as_str().map(str::to_owned),
        safelist: input["safelist"].as_str().map(str::to_owned),
        full: input["full"].as_bool().unwrap_or(false),
        css: if input["css"].as_bool() == Some(false)
            || ["noFiles", "missingTokens", "unknownValidatorConfig"].contains(&id)
        {
            CssInput::Disabled
        } else if let Some(file) = input["css"].as_str() {
            CssInput::File(file.into())
        } else {
            CssInput::Discover
        },
    }
}
#[test]
fn frozen_project_build_reports_and_error_contracts() {
    let fixture: Value = serde_json::from_str(include_str!(
        "../../../../../tests/utility/cn/build-fixtures.json"
    ))
    .unwrap();
    let tree = fixture_tree();
    let loader = fixture_loader();
    let mut checked = 0;
    for case in fixture["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        if [
            "builder-dropped-group-edit",
            "content-matcher",
            "token-length-cap",
        ]
        .contains(&id)
        {
            continue;
        }
        let expected = &case["expected"];
        let options = fixture_options(case);
        let result = build(&tree, &options, Some(&loader));
        if expected.get("message").is_some() {
            assert_eq!(
                result
                    .err()
                    .unwrap()
                    .message
                    .replace("/fixture", "<fixture>"),
                expected["message"].as_str().unwrap(),
                "{id}"
            );
            checked += 1;
            continue;
        }
        let report = result.unwrap();
        let artifact = &report.artifact;
        let mut tokens = artifact.tokens.clone();
        tokens.sort();
        assert_eq!(json!(tokens), expected["tokens"], "{id} tokens");
        assert_eq!(
            config_hash(&artifact.config),
            expected["configSha256"].as_str().unwrap(),
            "{id} config"
        );
        assert_eq!(
            config_hash(&artifact.full_config),
            expected["fullConfigSha256"].as_str().unwrap(),
            "{id} full config"
        );
        assert_eq!(
            group_map(&artifact.full_config.theme),
            expected["theme"],
            "{id} theme"
        );
        assert_eq!(
            json!(artifact
                .config
                .class_groups
                .iter()
                .map(|(key, _)| key)
                .collect::<Vec<_>>()),
            expected["keptClassGroups"],
            "{id} groups"
        );
        assert_eq!(
            artifact.full_config.class_groups.len(),
            expected["fullClassGroupCount"].as_u64().unwrap() as usize
        );
        assert_eq!(
            report.scanned_files,
            expected["scannedFiles"].as_u64().unwrap() as usize
        );
        assert_eq!(
            json!(report
                .css_files
                .iter()
                .map(|path| path.trim_start_matches("/fixture/"))
                .collect::<Vec<_>>()),
            expected["cssFiles"]
        );
        assert_eq!(json!(report.warnings), expected["warnings"]);
        assert_eq!(json!(artifact.used_groups), expected["usedGroups"]);
        assert_eq!(json!(artifact.total_groups), expected["totalGroups"]);
        assert_eq!(
            digest(&artifact.source),
            expected["sourceSha256"].as_str().unwrap(),
            "{id} source"
        );
        assert_eq!(
            artifact.source.len(),
            expected["sourceBytes"].as_u64().unwrap() as usize
        );
        if let Some(merges) = expected["merges"].as_array() {
            let cn = Cn::new(artifact.config.clone()).unwrap();
            for merge in merges {
                assert_eq!(
                    cn.merge_str(merge["input"].as_str().unwrap()).unwrap(),
                    merge["expected"].as_str().unwrap(),
                    "{id}"
                );
            }
        }
        assert!(artifact.changed);
        let second = build(&tree, &options, Some(&loader)).unwrap();
        assert!(!second.artifact.changed);
        assert_eq!(second.artifact.source, artifact.source);
        checked += 1;
    }
    assert_eq!(checked, 9);
}
#[test]
fn frozen_incremental_builder_and_matcher_and_cap() {
    let fixture: Value = serde_json::from_str(include_str!(
        "../../../../../tests/utility/cn/build-fixtures.json"
    ))
    .unwrap();
    let cases = fixture["cases"].as_array().unwrap();
    let expected = &cases
        .iter()
        .find(|c| c["id"] == "builder-dropped-group-edit")
        .unwrap()["expected"];
    let tree = Rc::new(fixture_tree());
    let options = ProjectOptions {
        cwd: Some("/fixture".into()),
        content: vec!["src/**/*.html".into()],
        out: "builder.mjs".into(),
        css: CssInput::Disabled,
        ..Default::default()
    };
    let builder = create_builder(tree.clone(), options, None).unwrap();
    assert!(builder.last().is_none());
    let first = builder.run().unwrap();
    tree.put_file("/fixture/src/view.html", "<div class=\"p-6\"></div>");
    let known = builder.changed("src/view.html").unwrap();
    tree.put_file("/fixture/src/view.html", "<div class=\"m-4\"></div>");
    let fresh = builder.changed("src/view.html").unwrap();
    let result = json!({"firstGroups":first.artifact.config.class_groups.iter().map(|(key,_)|key).collect::<Vec<_>>(),"reusedKnownResult":Rc::ptr_eq(&known,&first),"addedGroupChanged":fresh.artifact.changed,"lastIsNewResult":Rc::ptr_eq(&builder.last().unwrap(),&fresh),"freshGroups":fresh.artifact.config.class_groups.iter().map(|(key,_)|key).collect::<Vec<_>>()});
    assert_eq!(&result, expected);
    let matcher = create_content_matcher(
        &["**/*.html".into(), "node_modules/allowed/*.html".into()],
        "/fixture",
    )
    .unwrap();
    let expected = &cases.iter().find(|c| c["id"] == "content-matcher").unwrap()["expected"];
    for item in expected.as_array().unwrap() {
        assert_eq!(
            matcher.matches(item["path"].as_str().unwrap()),
            item["matched"].as_bool().unwrap(),
            "{}",
            item["path"]
        );
    }
    let mut tokens = Vec::new();
    let skipped = extract_tokens(
        &("class=\"p-2 hover:p-4\" <x> ".to_owned() + &"x".repeat(8193)),
        &mut tokens,
    );
    let expected = &cases
        .iter()
        .find(|c| c["id"] == "token-length-cap")
        .unwrap()["expected"];
    assert_eq!(json!({"skippedLong":skipped,"tokens":tokens}), *expected);
}

fn contract() -> Value {
    serde_json::from_str(include_str!(
        "../../../../../tests/utility/cn/build-contract-fixtures.json"
    ))
    .unwrap()
}
fn contract_tree(fixture: &Value) -> BuildSourceTree {
    let tree = BuildSourceTree::virtual_files([]);
    for (key, value) in fixture["files"].as_object().unwrap() {
        tree.put_file(&format!("/project/{key}"), value.as_str().unwrap());
    }
    for (path, target) in fixture["links"].as_object().unwrap() {
        tree.put_symlink(&format!("/project/{path}"), target.as_str().unwrap());
    }
    tree
}
#[test]
fn fixed_glob_matcher_and_css_contracts() {
    let fixture = contract();
    let tree = contract_tree(&fixture);
    for case in fixture["globs"].as_array().unwrap() {
        let patterns: Vec<_> = case["patterns"]
            .as_array()
            .unwrap()
            .iter()
            .map(|p| p.as_str().unwrap().into())
            .collect();
        let mut state = ScanState::default();
        let result = expand_globs(&tree, &patterns, "/project", &mut state);
        let got = match result {
            Ok(files) => {
                json!({"files":files.iter().map(|f|f.strip_prefix("/project/").map(str::to_owned).unwrap_or_else(||"../outside.html".into())).collect::<Vec<_>>(),"skippedUnreadable":state.skipped_unreadable})
            }
            Err(err) => json!({"error":err.message}),
        };
        assert_eq!(got, case["expected"], "glob {patterns:?}");
    }
    for case in fixture["matchers"].as_array().unwrap() {
        let patterns: Vec<_> = case["patterns"]
            .as_array()
            .unwrap()
            .iter()
            .map(|p| p.as_str().unwrap().into())
            .collect();
        let result = create_content_matcher(&patterns, "/project");
        if let Some(error) = case["expected"].get("error") {
            assert_eq!(result.err().unwrap().message, error.as_str().unwrap());
        } else {
            let matcher = result.unwrap();
            for probe in case["expected"].as_array().unwrap() {
                assert_eq!(
                    matcher.matches(probe["path"].as_str().unwrap()),
                    probe["matched"].as_bool().unwrap(),
                    "matcher {patterns:?} {}",
                    probe["path"]
                );
            }
        }
    }
    for probe in fixture["discovery"].as_array().unwrap() {
        assert_eq!(
            is_tailwind_css(probe["css"].as_str().unwrap()),
            probe["expected"].as_bool().unwrap()
        );
    }
    for case in fixture["themes"].as_array().unwrap() {
        let entries: Vec<_> = case["entries"]
            .as_array()
            .unwrap()
            .iter()
            .map(|p| format!("/project/{}", p.as_str().unwrap()))
            .collect();
        let got = match theme_from_css(&tree, &entries) {
            Ok(theme) => {
                let mut extension = serde_json::Map::new();
                if !theme.extension.extend.theme.is_empty() {
                    extension.insert(
                        "extend".into(),
                        json!({"theme":group_map(&theme.extension.extend.theme)}),
                    );
                }
                if !theme.extension.override_groups.theme.is_empty() {
                    extension.insert(
                        "override".into(),
                        json!({"theme":group_map(&theme.extension.override_groups.theme)}),
                    );
                }
                json!({"extension":extension,"files":theme.files.iter().map(|f|f.trim_start_matches("/project/")).collect::<Vec<_>>()})
            }
            Err(err) => json!({"error":err.message.replace("/project","<project>")}),
        };
        assert_eq!(got, case["expected"], "theme {entries:?}");
    }
}
#[test]
fn fixed_extended_build_contracts() {
    let fixture = contract();
    let tree = contract_tree(&fixture);
    let loader: ConfigLoader = Rc::new(|_, _, mut base| {
        base.prefix = Some("tw".into());
        Ok(base)
    });
    for case in fixture["builds"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        let input = &case["options"];
        let mut options = fixture_options(&json!({"id":id,"options":input}));
        options.cwd = Some("/project".into());
        options.out = input["out"]
            .as_str()
            .map(str::to_owned)
            .unwrap_or_else(|| format!("contract-{id}.mjs"));
        tree.put_file(
            "/project/long.html",
            format!("class=\"p-2\" {}", "😀".repeat(4097)),
        );
        tree.put_file("/project/self/tables.html", "p-4");
        tree.put_file("/project/outputs/real.html", "m-4");
        tree.put_file("/project/outputs/tables.html", "p-4");
        tree.put_symlink("/project/outputs/alias.html", "tables.html");
        if id == "missingImport" {
            tree.put_file(
                "/project/themes/broken.css",
                "@import \"./missing.css\"; @theme {}",
            );
        }
        let got = match build(&tree, &options, Some(&loader)) {
            Ok(report) => {
                let mut tokens: Vec<_> = report
                    .artifact
                    .tokens
                    .iter()
                    .filter(|s| s.encode_utf16().count() < 100)
                    .cloned()
                    .collect();
                tokens.sort();
                json!({"tokens":tokens,"longTokens":report.artifact.tokens.iter().filter(|s|s.encode_utf16().count()>=100).map(|s|s.encode_utf16().count()).collect::<Vec<_>>(),"scannedFiles":report.scanned_files,"cssFiles":report.css_files.iter().map(|f|f.trim_start_matches("/project/")).collect::<Vec<_>>(),"groups":report.artifact.config.class_groups.iter().map(|(key,_)|key).collect::<Vec<_>>(),"warnings":report.warnings,"sourceSha256":digest(&report.artifact.source)})
            }
            Err(err) => json!({"error":err.message.replace("/project","<project>")}),
        };
        assert_eq!(got, case["expected"], "build {id}");
    }
}
#[test]
fn builder_coalesces_concurrent_and_reentrant_requests() {
    use std::cell::{Cell, RefCell};
    use std::future::Future;
    use std::task::{Context, Poll, Waker};
    let fixture = contract();
    let tree = Rc::new(contract_tree(&fixture));
    let count = Rc::new(Cell::new(0));
    let calls = count.clone();
    let loader: ConfigLoader = Rc::new(move |_, _, base| {
        calls.set(calls.get() + 1);
        Ok(base)
    });
    let options = ProjectOptions {
        cwd: Some("/project".into()),
        content: vec!["src/**/*.html".into()],
        css: CssInput::Disabled,
        config: Some("coalesce.mjs".into()),
        out: "coalesce-tables.mjs".into(),
        ..Default::default()
    };
    let builder = create_builder(tree.clone(), options.clone(), Some(loader)).unwrap();
    let mut one = std::pin::pin!(builder.run_async());
    let mut two = std::pin::pin!(builder.run_async());
    let mut cx = Context::from_waker(Waker::noop());
    let Poll::Ready(Ok(one)) = one.as_mut().poll(&mut cx) else {
        panic!("first run must finish");
    };
    let Poll::Ready(Ok(two)) = two.as_mut().poll(&mut cx) else {
        panic!("shared run must finish");
    };
    assert_eq!(
        json!({"builds":count.get(),"same":Rc::ptr_eq(&one,&two),"lastSame":Rc::ptr_eq(&builder.last().unwrap(),&one),"changed":one.artifact.changed}),
        fixture["coalescing"]
    );
    let slot: Rc<RefCell<Option<Builder>>> = Rc::new(RefCell::new(None));
    let queued: Rc<RefCell<Option<BuildFuture>>> = Rc::new(RefCell::new(None));
    let count = Rc::new(Cell::new(0));
    let slot_in = slot.clone();
    let queue_in = queued.clone();
    let count_in = count.clone();
    let loader: ConfigLoader = Rc::new(move |_, _, base| {
        count_in.set(count_in.get() + 1);
        if count_in.get() == 1 {
            let mut future = slot_in.borrow().as_ref().unwrap().run_async();
            assert!(std::pin::Pin::new(&mut future)
                .poll(&mut Context::from_waker(Waker::noop()))
                .is_pending());
            *queue_in.borrow_mut() = Some(future);
        }
        Ok(base)
    });
    let builder = create_builder(tree, options, Some(loader)).unwrap();
    *slot.borrow_mut() = Some(builder.clone());
    let result = builder.run().unwrap();
    assert_eq!(count.get(), 2);
    let mut pending = queued.borrow_mut().take().unwrap();
    let Poll::Ready(Ok(followup)) = std::pin::Pin::new(&mut pending).poll(&mut cx) else {
        panic!("reentrant flight must resolve");
    };
    assert!(Rc::ptr_eq(&result, &followup));
    *slot.borrow_mut() = None;
}

#[test]
fn fixed_cli_and_bundler_lifecycle_contracts() {
    let fixture = contract();
    let tree = Rc::new(contract_tree(&fixture));
    // The oracle runs its plugin tests after editing the source to m-4.
    tree.put_file("/project/src/view.html", "m-4");
    for case in fixture["cli"].as_array().unwrap() {
        let args: Vec<_> = case["args"]
            .as_array()
            .unwrap()
            .iter()
            .map(|arg| arg.as_str().unwrap().replace("<project>", "/project"))
            .collect();
        let output = run_cli(&tree, &args, None);
        assert_eq!(
            output.exit_code,
            case["expected"]["exitCode"].as_i64().unwrap() as i32,
            "CLI {}",
            case["id"]
        );
        assert_eq!(
            output.stderr,
            case["expected"]["stderr"].as_str().unwrap(),
            "CLI {}",
            case["id"]
        );
        assert_eq!(
            output.stdout,
            case["expected"]["stdout"].as_str().unwrap(),
            "CLI {}",
            case["id"]
        );
    }
    let options = ProjectOptions {
        css: CssInput::Disabled,
        content: vec!["src/**/*.html".into()],
        ..Default::default()
    };
    tree.put_file("/project/src/view.html", "<div class=\"p-2 p-4\"></div>");
    let mut vite = ViteAdapter::new(tree.clone(), options, None);
    vite.config_resolved("/project").unwrap();
    let first = vite.build_start().unwrap();
    tree.put_file("/project/src/view.html", "m-4");
    let skipped = vite
        .watch_change("/project/src/view.html", "delete")
        .unwrap();
    let delete_ignored = skipped.is_none()
        && tree.read_text("/project/cn-tables.mjs").unwrap() == first.artifact.source;
    let fresh = vite
        .watch_change("/project/src/view.html", "update")
        .unwrap()
        .unwrap();
    assert_eq!(
        json!({"name":ViteAdapter::NAME,"deleteIgnored":delete_ignored,"updateChanged":fresh.artifact.source!=first.artifact.source}),
        fixture["tools"]["vite"]
    );
    let options = ProjectOptions {
        cwd: Some("/project".into()),
        css: CssInput::Disabled,
        full: true,
        out: "next-tables.mjs".into(),
        ..Default::default()
    };
    let mut passed = Value::Null;
    let next=with_cn(tree.clone(),options,None,"phase-production-build",&json!({"probe":7}),|phase,context|{passed=json!({"phase":phase,"context":context,"outputExists":tree.read_text("/project/next-tables.mjs").is_ok()});Ok(json!({"test":true}))}).unwrap();
    assert_eq!(
        json!({"passed":passed,"result":next.config}),
        fixture["tools"]["next"]
    );
    assert!(next.watch_directory.is_none());
    assert!(next.watch_event(Some("src/view.html")).unwrap().is_none());
}
#[test]
fn gzip_output_is_a_real_level_nine_member() {
    use std::io::{Read, Write};
    let input = "cn source 😀".repeat(300);
    let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::new(9));
    encoder.write_all(input.as_bytes()).unwrap();
    let bytes = encoder.finish().unwrap();
    assert_eq!(gzip_size(&input).unwrap(), bytes.len());
    assert_eq!(&bytes[..3], &[31, 139, 8]);
    let mut decoded = String::new();
    flate2::read::GzDecoder::new(bytes.as_slice())
        .read_to_string(&mut decoded)
        .unwrap();
    assert_eq!(decoded, input);
}
#[test]
fn warnings_and_theme_watcher_and_failure_recovery() {
    use std::cell::Cell;
    let tree = Rc::new(BuildSourceTree::virtual_files([
        ("/project/src/view.html", "shadow-card p-2"),
        ("/project/theme.css", "@theme { --shadow-card: 1px; }"),
    ]));
    tree.put_unreadable("/project/blocked", true);
    tree.put_unreadable("/project/src/denied.html", false);
    tree.put_file("/project/src/long.html", "😀".repeat(4097));
    let options = ProjectOptions {
        cwd: Some("/project".into()),
        ..Default::default()
    };
    let builder = create_builder(tree.clone(), options, None).unwrap();
    let first = builder.run().unwrap();
    assert_eq!(
        first.warnings,
        [
            "skipped 2 unreadable path(s)",
            "skipped 1 candidate token(s) longer than 8192 chars"
        ]
    );
    tree.remove("/project/src/view.html");
    assert!(Rc::ptr_eq(
        &builder.changed("src/view.html").unwrap(),
        &first
    ));
    assert!(Rc::ptr_eq(
        &builder.changed("unrelated.txt").unwrap(),
        &first
    ));
    tree.put_file("/project/new.css", "@theme { --shadow-new: 1px; }");
    let added = builder.changed("new.css").unwrap();
    assert!(!Rc::ptr_eq(&first, &added));
    assert!(group_map(&added.artifact.full_config.theme)["shadow"]
        .as_array()
        .unwrap()
        .contains(&json!("new")));
    tree.remove("/project/new.css");
    let removed = builder.changed("new.css").unwrap();
    assert!(removed.css_files.iter().all(|f| !f.ends_with("new.css")));
    let count = Rc::new(Cell::new(0));
    let count_in = count.clone();
    let loader: ConfigLoader = Rc::new(move |_, _, base| {
        count_in.set(count_in.get() + 1);
        if count_in.get() == 1 {
            Err(super::super::CnError::new("module callback failure"))
        } else {
            Ok(base)
        }
    });
    let options = ProjectOptions {
        cwd: Some("/project".into()),
        config: Some("config.mjs".into()),
        full: true,
        css: CssInput::Disabled,
        ..Default::default()
    };
    let builder = create_builder(tree, options, Some(loader)).unwrap();
    assert_eq!(
        builder.run().err().unwrap().message,
        "module callback failure"
    );
    assert!(builder.last().is_none());
    assert!(builder.run().is_ok());
    assert_eq!(count.get(), 2);
}
#[test]
fn virtual_output_follows_directory_and_dangling_file_symlinks() {
    let tree = BuildSourceTree::virtual_files([]);
    tree.put_directory("/project/target");
    tree.put_symlink("/project/link", "target");
    tree.write_text("/project/link/new.mjs", "source").unwrap();
    assert_eq!(tree.read_text("/project/target/new.mjs").unwrap(), "source");
    tree.put_symlink("/project/output.mjs", "target/dangling.mjs");
    tree.write_text("/project/output.mjs", "updated").unwrap();
    assert_eq!(
        tree.read_text("/project/target/dangling.mjs").unwrap(),
        "updated"
    );
}
#[cfg(unix)]
#[test]
fn native_and_virtual_sources_agree_with_frozen_globs_and_builds() {
    use std::os::unix::fs::symlink;
    let fixture = contract();
    let temp = std::env::temp_dir().join(format!(
        "rust-cn-build-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    struct Cleanup(std::path::PathBuf);
    impl Drop for Cleanup {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }
    let _cleanup = Cleanup(temp.clone());
    let cwd = temp.join("project");
    std::fs::create_dir_all(&cwd).unwrap();
    let native = BuildSourceTree::Native;
    let virtual_tree = contract_tree(&fixture);
    for (path, text) in fixture["files"].as_object().unwrap() {
        let path = source::resolve(cwd.to_str().unwrap(), path);
        native.write_text(&path, text.as_str().unwrap()).unwrap();
    }
    for (path, target) in fixture["links"].as_object().unwrap() {
        let path = cwd.join(path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        symlink(target.as_str().unwrap(), path).unwrap();
    }
    for case in fixture["globs"].as_array().unwrap() {
        let patterns: Vec<_> = case["patterns"]
            .as_array()
            .unwrap()
            .iter()
            .map(|p| p.as_str().unwrap().into())
            .collect();
        let mut state = ScanState::default();
        let native_paths = expand_globs(&native, &patterns, cwd.to_str().unwrap(), &mut state);
        let mut other_state = ScanState::default();
        let virtual_paths = expand_globs(&virtual_tree, &patterns, "/project", &mut other_state);
        match (native_paths, virtual_paths) {
            (Ok(a), Ok(b)) => assert_eq!(
                a.iter()
                    .map(|p| p.replace(temp.to_str().unwrap(), ""))
                    .collect::<Vec<_>>(),
                b
            ),
            (Err(a), Err(b)) => assert_eq!(a.message, b.message),
            _ => panic!("native/virtual glob discrepancy"),
        };
        assert_eq!(state.skipped_unreadable, other_state.skipped_unreadable);
    }
    let options = ProjectOptions {
        cwd: Some(cwd.to_string_lossy().into_owned()),
        content: vec!["src/**/*.html".into()],
        css: CssInput::File("themes/main.css".into()),
        out: "native.mjs".into(),
        ..Default::default()
    };
    let report = build(&native, &options, None).unwrap();
    let mut virtual_options = options.clone();
    virtual_options.cwd = Some("/project".into());
    let other = build(&virtual_tree, &virtual_options, None).unwrap();
    assert_eq!(report.artifact.source, other.artifact.source);
    assert_eq!(report.artifact.tokens, other.artifact.tokens);
    assert_eq!(
        config_json(&report.artifact.full_config),
        config_json(&other.artifact.full_config)
    );
    assert!(!build(&native, &options, None).unwrap().artifact.changed);
}

#[test]
fn cli_rounding_and_json_config_and_async_next_factory() {
    use std::future::Future;
    use std::task::{Context, Poll, Waker};
    let tree = Rc::new(BuildSourceTree::virtual_files([(
        "/project/config.json",
        r#"{"prefix":"tw","extend":{"classGroups":{"display":["custom-display"]}}}"#,
    )]));
    let options = ProjectOptions {
        cwd: Some("/project".into()),
        full: true,
        config: Some("config.json".into()),
        css: CssInput::Disabled,
        ..Default::default()
    };
    let mut report = build(&tree, &options, None).unwrap();
    assert_eq!(report.artifact.full_config.prefix.as_deref(), Some("tw"));
    assert!(definitions(
        &report
            .artifact
            .full_config
            .class_groups
            .iter()
            .find(|(key, _)| key == "display")
            .unwrap()
            .1
    )
    .as_array()
    .unwrap()
    .contains(&json!("custom-display")));
    report.artifact.source = "x".repeat(256);
    assert!(cli_summary(&report, &options)
        .unwrap()
        .contains("0.3 KB source"));
    let mut future = std::pin::pin!(with_cn_async(
        tree.clone(),
        options,
        None,
        "phase-development-server",
        &17,
        |phase, context| {
            assert_eq!(phase, "phase-development-server");
            assert_eq!(*context, 17);
            assert!(tree.read_text("/project/cn-tables.mjs").is_ok());
            std::future::ready(Ok("factory returned"))
        }
    ));
    let Poll::Ready(Ok(next)) = future
        .as_mut()
        .poll(&mut Context::from_waker(Waker::noop()))
    else {
        panic!("ready config factory must complete");
    };
    assert_eq!(next.config, "factory returned");
    assert_eq!(next.watch_directory.as_deref(), Some("/project"));
    assert!(next.watch_event(None).unwrap().is_none());
}
#[test]
fn failed_coalesced_flight_does_not_build_followup_and_can_restart() {
    use std::cell::Cell;
    use std::future::Future;
    use std::task::{Context, Poll, Waker};
    let tree = Rc::new(BuildSourceTree::virtual_files([]));
    let count = Rc::new(Cell::new(0));
    let count_in = count.clone();
    let loader: ConfigLoader = Rc::new(move |_, _, base| {
        count_in.set(count_in.get() + 1);
        if count_in.get() == 1 {
            Err(super::super::CnError::new("failing module"))
        } else {
            Ok(base)
        }
    });
    let options = ProjectOptions {
        full: true,
        config: Some("module.mjs".into()),
        css: CssInput::Disabled,
        ..Default::default()
    };
    let builder = create_builder(tree, options, Some(loader)).unwrap();
    let mut one = std::pin::pin!(builder.run_async());
    let mut two = std::pin::pin!(builder.run_async());
    let mut cx = Context::from_waker(Waker::noop());
    let Poll::Ready(Err(one)) = one.as_mut().poll(&mut cx) else {
        panic!("first flight must reject");
    };
    let Poll::Ready(Err(two)) = two.as_mut().poll(&mut cx) else {
        panic!("second flight must share rejection");
    };
    assert_eq!(one, two);
    assert_eq!(count.get(), 1);
    assert!(builder.last().is_none());
    assert!(builder.run().is_ok());
    assert_eq!(count.get(), 2);
}
#[cfg(unix)]
#[test]
fn native_permission_warnings_and_non_file_entries_follow_host_semantics() {
    use std::os::unix::fs::PermissionsExt;
    use std::os::unix::net::UnixListener;
    let temp = std::env::temp_dir().join(format!(
        "cn-perms-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir_all(temp.join("blocked")).unwrap();
    struct Cleanup(std::path::PathBuf);
    impl Drop for Cleanup {
        fn drop(&mut self) {
            let _ = std::fs::set_permissions(
                self.0.join("blocked"),
                std::fs::Permissions::from_mode(0o700),
            );
            let _ = std::fs::set_permissions(
                self.0.join("denied.html"),
                std::fs::Permissions::from_mode(0o600),
            );
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }
    let _cleanup = Cleanup(temp.clone());
    std::fs::write(temp.join("view.html"), "p-2 p-4").unwrap();
    std::fs::write(temp.join("denied.html"), "m-4").unwrap();
    std::fs::write(temp.join("blocked/view.html"), "m-4").unwrap();
    let _socket = UnixListener::bind(temp.join("socket.html")).unwrap();
    std::fs::set_permissions(temp.join("blocked"), std::fs::Permissions::from_mode(0o0)).unwrap();
    std::fs::set_permissions(
        temp.join("denied.html"),
        std::fs::Permissions::from_mode(0o0),
    )
    .unwrap();
    let blocked = usize::from(std::fs::read_dir(temp.join("blocked")).is_err());
    let denied = usize::from(std::fs::read(temp.join("denied.html")).is_err());
    let options = ProjectOptions {
        cwd: Some(temp.to_string_lossy().into_owned()),
        css: CssInput::Disabled,
        ..Default::default()
    };
    let report = build(&BuildSourceTree::Native, &options, None).unwrap();
    assert_eq!(report.scanned_files, 3 - blocked - denied);
    assert_eq!(
        report.warnings,
        if blocked + denied > 0 {
            vec![format!("skipped {} unreadable path(s)", blocked + denied)]
        } else {
            Vec::new()
        }
    );
}

#[test]
fn virtual_growing_symlink_cycles_and_unreadable_ancestors_are_bounded() {
    let tree = BuildSourceTree::virtual_files([("/project/denied/tokens.txt", "p-2")]);
    tree.put_symlink("/project/loop", "loop/child");
    assert!(tree
        .read_text("/project/loop")
        .unwrap_err()
        .starts_with("ELOOP:"));
    tree.put_unreadable("/project/denied", true);
    assert!(tree
        .read_text("/project/denied/tokens.txt")
        .unwrap_err()
        .starts_with("EACCES:"));
    let options = ProjectOptions {
        cwd: Some("/project".into()),
        tokens: Some("denied/tokens.txt".into()),
        css: CssInput::Disabled,
        ..Default::default()
    };
    assert!(build(&tree, &options, None)
        .err()
        .unwrap()
        .message
        .starts_with("cannot read tokens file denied/tokens.txt: EACCES:"));
}
