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
}

type Deferred = Box<dyn FnOnce(&mut Cx)>;

struct Region {
    /// Taken out for the duration of a pump so re-entrant calls from event
    /// handlers never observe a borrowed registry.
    cx: Option<Box<Cx>>,
    app: Rc<dyn Any>,
    deliver: Rc<dyn Fn()>,
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
}

fn with_hooks<R>(f: impl FnOnce(&HostHooks) -> R) -> Option<R> {
    HOOKS.with(|slot| slot.borrow().as_ref().map(f))
}

/// Creates a region: its own `Cx`, script VM and widget tree, drawn into
/// `canvas`. Actions the app emits are handed to `on_action` after each pump.
pub fn create_region<A: RegionApp>(
    canvas: &web_sys::HtmlCanvasElement,
    on_action: impl Fn(A::Action) + 'static,
) -> Option<RegionId> {
    let app: AppCell<A> = Rc::new(RefCell::new(None));
    let outbox: Outbox<A> = Rc::new(RefCell::new(Vec::new()));
    let mut cx = Box::new(Cx::new(event_closure(app.clone(), outbox.clone())));
    cx.init_cx_os();

    let deliver: Rc<dyn Fn()> = Rc::new(move || {
        let pending: Vec<A::Action> = outbox.borrow_mut().drain(..).collect();
        for action in pending {
            on_action(action);
        }
    });
    let id = REGIONS.with(|regions| {
        regions.borrow_mut().insert(Region {
            cx: Some(cx),
            app,
            deliver,
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

/// Tears the region down: JS resources first, then the `Cx`. When called from
/// inside the region's own pump the teardown completes once the pump returns.
pub fn destroy_region(id: RegionId) {
    let idle = REGIONS.with(|regions| {
        let mut regions = regions.borrow_mut();
        let Some(region) = regions.get_mut(id) else {
            return None;
        };
        region.disposing = true;
        Some(region.cx.is_some())
    });
    if idle == Some(true) {
        finish_destroy(id);
    }
}

fn finish_destroy(id: RegionId) {
    with_hooks(|hooks| hooks.destroy_region(id.raw()));
    let removed = REGIONS.with(|regions| regions.borrow_mut().remove(id));
    drop(removed);
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
    let disposing = REGIONS.with(|regions| {
        let mut regions = regions.borrow_mut();
        match regions.get_mut(id) {
            Some(region) => {
                region.cx = Some(cx);
                region.disposing
            }
            None => true,
        }
    });
    deliver();
    if disposing {
        finish_destroy(id);
    }
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
