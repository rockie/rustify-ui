use super::app::{event_closure, AppCell, Outbox, RegionApp};
use crate::{RegionId, Registry};
use makepad_widgets::makepad_platform::makepad_wasm_bridge::ToWasmMsg;
use makepad_widgets::*;
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// JS-side owner of canvases, GL contexts, listeners and timers. One
    /// object per runtime; every call carries the region it concerns.
    pub type HostHooks;

    #[wasm_bindgen(method)]
    fn create_region(this: &HostHooks, region: u32, canvas: &web_sys::HtmlCanvasElement) -> bool;

    #[wasm_bindgen(method)]
    fn destroy_region(this: &HostHooks, region: u32);

    #[wasm_bindgen(method)]
    fn request_pump(this: &HostHooks, region: u32);

    #[wasm_bindgen(method)]
    fn request_signal_pump(this: &HostHooks);

    #[wasm_bindgen(method)]
    fn defer(this: &HostHooks, callback: &JsValue);
}

type Deferred = Box<dyn FnOnce(&mut Cx)>;

struct Region {
    /// Taken out for the duration of a pump so re-entrant calls from event
    /// handlers never observe a borrowed registry.
    cx: Option<Box<Cx>>,
    app: Rc<dyn Any>,
    deliver: Rc<dyn Fn()>,
    /// Told when the region's canvas loses or regains a drawable size.
    suspend: Rc<dyn Fn(bool)>,
    deferred: Vec<Deferred>,
    disposing: bool,
}

thread_local! {
    static HOOKS: RefCell<Option<HostHooks>> = const { RefCell::new(None) };
    static REGIONS: RefCell<Registry<Region>> = RefCell::new(Registry::new());
}

#[wasm_bindgen]
pub fn rustify_makepad_boot(hooks: HostHooks) {
    Cx::init_log();
    HOOKS.with(|slot| *slot.borrow_mut() = Some(hooks));
    SignalToUI::set_signal_hook(Some(on_signal));
}

/// Makepad's UI and action signals are process-wide flags. Raising one wakes
/// the host, which reads the flags once and hands them to every live region;
/// without this the host would have to poll for an edge that is almost never
/// there.
fn on_signal() {
    with_hooks(|hooks| hooks.request_signal_pump());
}

fn with_hooks<R>(f: impl FnOnce(&HostHooks) -> R) -> Option<R> {
    HOOKS.with(|slot| slot.borrow().as_ref().map(f))
}

/// Runs `f` on a fresh browser task, giving the browser its turn in between.
///
/// The host owns the task rather than `setTimeout` here, so a runtime that has
/// failed can drop it: the callback is wasm code, and a trapped module must
/// not be re-entered.
pub fn defer(f: impl FnOnce() + 'static) {
    let callback = Closure::once_into_js(f);
    with_hooks(|hooks| hooks.defer(&callback));
}

/// Creates a region: its own `Cx`, script VM and widget tree, drawn into
/// `canvas`. Everything the app emitted during a pump is handed to
/// `on_actions` in one call, after the pump has returned to a safe point.
pub fn create_region<A: RegionApp>(
    canvas: &web_sys::HtmlCanvasElement,
    on_actions: impl Fn(Vec<A::Action>) + 'static,
    on_suspended: impl Fn(bool) + 'static,
) -> Option<RegionId> {
    let app: AppCell<A> = Rc::new(RefCell::new(None));
    let outbox: Outbox<A> = Rc::new(RefCell::new(Vec::new()));
    let mut cx = Box::new(Cx::new(event_closure(app.clone(), outbox.clone())));
    cx.init_cx_os();

    let deliver: Rc<dyn Fn()> = Rc::new(move || {
        let pending: Vec<A::Action> = outbox.borrow_mut().drain(..).collect();
        if !pending.is_empty() {
            on_actions(pending);
        }
    });
    let id = REGIONS.with(|regions| {
        regions.borrow_mut().insert(Region {
            cx: Some(cx),
            app,
            deliver,
            suspend: Rc::new(on_suspended),
            deferred: Vec::new(),
            disposing: false,
        })
    });
    let created = with_hooks(|hooks| hooks.create_region(id.raw(), canvas)).unwrap_or(false);
    if !created {
        REGIONS.with(|regions| regions.borrow_mut().remove(id));
        return None;
    }
    Some(id)
}

/// Runs `f` against the region's `Cx` and app at the next safe point and
/// schedules a pump so the result is drawn. Returns `false` when the region
/// is gone or hosts a different app type.
pub fn apply<A: RegionApp>(id: RegionId, f: impl FnOnce(&mut Cx, &mut A) + 'static) -> bool {
    let queued = REGIONS.with(|regions| {
        let mut regions = regions.borrow_mut();
        let Some(region) = regions.get_mut(id) else {
            return false;
        };
        if region.disposing {
            return false;
        }
        let Ok(app) = region.app.clone().downcast::<RefCell<Option<A>>>() else {
            return false;
        };
        region.deferred.push(Box::new(move |cx| {
            if let Some(app) = &mut *app.borrow_mut() {
                f(cx, app);
            }
        }));
        true
    });
    if queued {
        with_hooks(|hooks| hooks.request_pump(id.raw()));
    }
    queued
}

/// Tears the region down. The region stops accepting work here; the host
/// decides when the rest is safe, because it knows when the pump it is running
/// has finished with what that pump produced.
pub fn destroy_region(id: RegionId) {
    let notify = REGIONS.with(|regions| {
        let mut regions = regions.borrow_mut();
        match regions.get_mut(id) {
            Some(region) if !region.disposing => {
                region.disposing = true;
                true
            }
            _ => false,
        }
    });
    if notify {
        with_hooks(|hooks| hooks.destroy_region(id.raw()));
    }
}

/// Drops the region's `Cx`. Called by the host once it has released its own
/// resources and consumed the last batch the region produced: that batch
/// points straight into buffers the `Cx` owns.
#[export_name = "rustify_region_release"]
pub unsafe extern "C" fn rustify_region_release(region: u32) {
    let removed = REGIONS.with(|regions| regions.borrow_mut().remove(RegionId::from_raw(region)));
    drop(removed);
}

/// Called by the host when the region's canvas gains or loses a drawable
/// size. A suspended region draws nothing until it has an area again.
#[export_name = "rustify_region_suspended"]
pub unsafe extern "C" fn rustify_region_suspended(region: u32, suspended: u32) {
    let notify = REGIONS.with(|regions| {
        regions
            .borrow()
            .get(RegionId::from_raw(region))
            .map(|region| region.suspend.clone())
    });
    // Outside the borrow: the application is about to write a signal, and
    // whatever that wakes may reach the registry again.
    if let Some(notify) = notify {
        notify(suspended != 0);
    }
}

pub fn live_region_count() -> usize {
    REGIONS.with(|regions| regions.borrow().len())
}

/// Entry point for the JS host: processes one message batch for `region` and
/// returns the outgoing batch, or 0 when the region no longer exists (the
/// incoming message is freed either way).
#[export_name = "rustify_region_process"]
pub unsafe extern "C" fn rustify_region_process(region: u32, msg_ptr: u32) -> u32 {
    let id = RegionId::from_raw(region);
    let taken = REGIONS.with(|regions| {
        let mut regions = regions.borrow_mut();
        let region = regions.get_mut(id)?;
        if region.disposing {
            return None;
        }
        let cx = region.cx.take()?;
        Some((
            cx,
            std::mem::take(&mut region.deferred),
            region.deliver.clone(),
        ))
    });
    let Some((mut cx, deferred, deliver)) = taken else {
        drop(ToWasmMsg::take_ownership(msg_ptr));
        return 0;
    };
    let out = cx.process_to_wasm_with(msg_ptr, |cx| {
        for f in deferred {
            f(cx);
        }
    });
    let orphan = REGIONS.with(|regions| {
        let mut regions = regions.borrow_mut();
        match regions.get_mut(id) {
            Some(region) => {
                region.cx = Some(cx);
                None
            }
            None => Some(cx),
        }
    });
    drop(orphan);
    deliver();
    out
}

/// Messages queued while the `Cx` was created, before the JS host existed.
#[export_name = "rustify_region_first_msg"]
pub unsafe extern "C" fn rustify_region_first_msg(region: u32) -> u32 {
    REGIONS.with(|regions| {
        let mut regions = regions.borrow_mut();
        regions
            .get_mut(RegionId::from_raw(region))
            .and_then(|region| region.cx.as_mut())
            .and_then(|cx| cx.os.from_wasm.take())
            .map(|msg| msg.release_ownership())
            .unwrap_or(0)
    })
}
