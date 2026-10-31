//! Finish field label references after synchronous deferred children render.

use leptos::prelude::*;
use leptos::tachys::{
    html::attribute::{any_attribute::AnyAttribute, Attribute, AttributeValue},
    hydration::Cursor,
    ssr::{StreamBuilder, StreamChunk},
    view::{add_attr::AddAnyAttr, Position, PositionState},
};

pub(super) struct FieldHtml {
    pub view: AnyView,
    pub binding: super::field::FieldBinding,
}

impl Render for FieldHtml {
    type State = <AnyView as Render>::State;
    fn build(self) -> Self::State {
        self.view.build()
    }
    fn rebuild(self, state: &mut Self::State) {
        self.view.rebuild(state);
    }
}
impl AddAnyAttr for FieldHtml {
    type Output<A: Attribute> = Self;
    fn add_any_attr<A: Attribute>(self, attr: A) -> Self {
        Self {
            view: self.view.add_any_attr(attr).into_any(),
            ..self
        }
    }
}
impl RenderHtml for FieldHtml {
    type AsyncOutput = Self;
    type Owned = Self;
    const MIN_LENGTH: usize = 0;

    fn dry_resolve(&mut self) {
        self.view.dry_resolve();
    }
    async fn resolve(self) -> Self {
        Self {
            view: self.view.resolve().await,
            ..self
        }
    }
    fn to_html_with_buf(
        self,
        buf: &mut String,
        position: &mut Position,
        escape: bool,
        mark_branches: bool,
        extra_attrs: Vec<AnyAttribute>,
    ) {
        let mut html = String::new();
        self.view
            .to_html_with_buf(&mut html, position, escape, mark_branches, extra_attrs);
        // Show, For and reactive closures construct controls during serialization.
        // Field-local buffering lets labels preceding them use the final ID,
        // without evaluating children twice or changing hydration markers/IDs.
        let mut marker = String::new();
        AttributeValue::to_html(self.binding.label_control(), "for", &mut marker);
        let mut actual = String::new();
        AttributeValue::to_html(self.binding.id(), "for", &mut actual);
        buf.push_str(&html.replace(&marker, &actual));
    }
    fn to_html_async_with_buf<const OUT_OF_ORDER: bool>(
        self,
        buf: &mut StreamBuilder,
        position: &mut Position,
        escape: bool,
        mark_branches: bool,
        extra_attrs: Vec<AnyAttribute>,
    ) {
        let mut field_position = *position;
        *position = Position::NextChild;
        // A field is one stream chunk so even asynchronous controls register
        // before its labels are emitted. Sibling fields can still stream.
        buf.push_async(leptos::reactive::computed::ScopedFuture::new(async move {
            let field = self.resolve().await;
            let mut html = String::new();
            field.to_html_with_buf(
                &mut html,
                &mut field_position,
                escape,
                mark_branches,
                extra_attrs,
            );
            [StreamChunk::Sync(html)].into()
        }));
    }
    fn hydrate<const FROM_SERVER: bool>(
        self,
        cursor: &Cursor,
        position: &PositionState,
    ) -> Self::State {
        self.view.hydrate::<FROM_SERVER>(cursor, position)
    }
    fn into_owned(self) -> Self {
        self
    }
}
