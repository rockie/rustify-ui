use makepad_widgets::*;
use std::cell::RefCell;
use std::rc::Rc;

/// A Makepad application that owns the widget tree of one GPU region.
///
/// The host application never touches `Cx` directly: it projects state in
/// through [`apply_props`](RegionApp::apply_props) and receives typed actions
/// that the region pushed into its outbox while handling events. Actions are
/// delivered after the region pump has returned to a safe point, so handlers
/// may freely write signals or apply new props.
pub trait RegionApp: ScriptNew + 'static {
    type Props: 'static;
    type Action: 'static;

    fn script_mod(vm: &mut ScriptVm) -> ScriptValue;

    fn apply_props(&mut self, cx: &mut Cx, props: &Self::Props);

    /// How the scope should admit one of this region's actions. A region that
    /// reports a stream of pointer states declares those continuous, so a
    /// flood of them can neither delay nor crowd out the clicks between them.
    fn pace(action: &Self::Action) -> crate::Pace {
        let _ = action;
        crate::Pace::Discrete
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, outbox: &mut Vec<Self::Action>);
}

pub(crate) type AppCell<A> = Rc<RefCell<Option<A>>>;
pub(crate) type Outbox<A> = Rc<RefCell<Vec<<A as RegionApp>::Action>>>;

/// Builds the event handler `Cx::new` drives. The app instance is created on
/// `Event::Startup` from its script module, exactly like `app_main!` does for
/// a single-app process, but the handle stays shared so the region can reach
/// the app outside of event dispatch.
pub(crate) fn event_closure<A: RegionApp>(
    app: AppCell<A>,
    outbox: Outbox<A>,
) -> Box<dyn FnMut(&mut Cx, &Event)> {
    Box::new(move |cx, event| {
        if let Event::Startup = event {
            let built = cx.with_vm(|vm| {
                let value = A::script_mod(vm);
                A::script_from_value(vm, value)
            });
            *app.borrow_mut() = Some(built);
        }
        if let Some(app) = &mut *app.borrow_mut() {
            app.handle_event(cx, event, &mut outbox.borrow_mut());
        }
    })
}
