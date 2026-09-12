import { WasmWebGL } from "../makepad_platform/web_gl.js";

// The capabilities an embedded region is refused, in the order the Rust side
// reads them. Adding one here without adding it there would report a refusal
// with nothing to say, so the two lists are checked against each other by a
// test on the Rust side.
const UNSUPPORTED_CODES = [
    "open_url",
    "browser_update_url",
    "browser_history_go",
    "fullscreen",
    "document_title",
];

// One Makepad region drawn into a canvas the host page owns. The Rust side
// creates the Cx first and hands the region id in; this object only holds
// browser resources (GL context, listeners, timers) and releases all of them
// in `destroy()`.
export class EmbeddedRegion extends WasmWebGL {
    constructor(wasm, canvas, region, msg_class, runtime) {
        super(wasm, undefined, canvas, { embedded: true, region, msg_class });
        this.runtime = runtime;
        this.pump_scheduled = false;
        this.pump_depth = 0;
        this.pending_release = null;
        this.suspended = false;
        this.pump_on_resume = false;
        this.context_lost = false;
        // The default action of this event is to make the loss permanent for
        // this canvas, so it is prevented; the region is rebuilt on a new
        // canvas either way, but a page that keeps this one can restore it.
        this.on_context_lost = (event) => {
            event.preventDefault();
            if (this.context_lost || this.destroyed) {
                return;
            }
            this.context_lost = true;
            this.runtime.record_error(`region ${this.wasm_app}: WebGL context lost`);
            if (this.runtime.fatal) {
                return;
            }
            try {
                this.exports.rustify_region_context_lost(this.wasm_app);
            } catch (error) {
                this.runtime.enter_fatal(error);
            }
        };
        canvas.addEventListener("webglcontextlost", this.on_context_lost);
    }

    // The embedded contract refuses the capabilities that belong to the page
    // rather than to a region. The base class warns once; this also puts it in
    // the runtime's record, where whoever has to act on it will look. The code
    // is an index rather than the name, because a diagnostic's detail is a
    // static string on the Rust side.
    unsupported(capability) {
        const first = !this.unsupported_reported.has(capability);
        super.unsupported(capability);
        const code = UNSUPPORTED_CODES.indexOf(capability);
        if (!first || code < 0 || this.runtime.fatal || this.destroyed) {
            return;
        }
        try {
            this.exports.rustify_note_unsupported(this.wasm_app, code);
        } catch (error) {
            this.runtime.enter_fatal(error);
        }
    }

    // Nothing may be pumped into a context that is gone: the batch would draw
    // with GL objects the browser has already taken back.
    wasm_is_callable() {
        return !this.runtime.fatal && !this.context_lost;
    }

    // Every batch that reaches the canvas begins by binding it and clearing
    // it, so this is the one call per frame this region actually presented.
    // A frame budget needs that rather than an animation frame callback: the
    // browser keeps calling back at 60 Hz while the picture stands still, and
    // an interval measured between those callbacks passes without anything
    // having been drawn.
    FromWasmBeginRenderCanvas(args) {
        this.runtime.frames += 1;
        super.FromWasmBeginRenderCanvas(args);
    }

    destroy() {
        this.canvas.removeEventListener("webglcontextlost", this.on_context_lost);
        super.destroy();
    }

    // A canvas laid out to nothing, or hidden, has no area to draw into. The
    // region keeps its state and stops working until it has one again; the
    // application hears about it because a hidden region is not a broken one.
    update_window_info() {
        super.update_window_info();
        const empty = this.canvas.clientWidth === 0 || this.canvas.clientHeight === 0;
        if (empty === this.suspended || this.destroyed || this.runtime.fatal) {
            return;
        }
        this.suspended = empty;
        try {
            this.exports.rustify_region_suspended(this.wasm_app, empty ? 1 : 0);
        } catch (error) {
            this.runtime.enter_fatal(error);
            return;
        }
        if (!empty && this.pump_on_resume) {
            this.pump_on_resume = false;
            this.request_pump();
        }
    }

    report_startup_failure(error) {
        this.runtime.record_error(`region ${this.wasm_app}: startup failed: ${error}`);
    }

    // Ends the region once its pump has left the JS stack: until then the pump
    // still reads this instance's views into wasm memory, and the batch it
    // returned still points into buffers the Cx owns. `release` drops that Cx,
    // so it runs last.
    destroy_when_pump_returns(release) {
        if (this.pump_depth > 0) {
            this.pending_release = release;
            return;
        }
        this.destroy();
        release();
    }

    // Rust asks for a pump after it queued work for this region; several
    // requests in one task collapse into one pump on the microtask queue.
    request_pump() {
        if (this.context_lost) {
            return;
        }
        if (this.suspended) {
            // Whatever asked for this pump is still queued in wasm; it runs
            // when the region has somewhere to draw it.
            this.pump_on_resume = true;
            return;
        }
        if (this.pump_scheduled || this.destroyed || this.runtime.fatal) {
            return;
        }
        this.pump_scheduled = true;
        queueMicrotask(() => {
            this.pump_scheduled = false;
            if (!this.destroyed && this.to_wasm) {
                this.do_wasm_pump();
                this.present();
            }
        });
    }

    // Draws what the pump just changed, in the frame it was changed in.
    //
    // A pump that only applied props leaves the region wanting to draw, and
    // what it asks for is an animation frame - the next one. Two of those
    // requests land in the same browser frame, because the request made while
    // a callback is already pending is dropped as a duplicate, so a region
    // driven once a frame drew on every second frame and a display at 60 Hz
    // presented it at 30.
    //
    // Serving the request here instead: the props pump runs on a microtask of
    // the task that changed them, which is still inside the frame, so the
    // draw lands in that frame's paint. A request made during this draw is a
    // real next frame - an animation running on - and goes back to the
    // browser untouched.
    present() {
        if (!this.req_anim_frame_id || this.destroyed || this.runtime.fatal) {
            return;
        }
        if (this.suspended || this.context_lost) {
            return;
        }
        window.cancelAnimationFrame(this.req_anim_frame_id);
        this.req_anim_frame_id = 0;
        this.to_wasm.ToWasmAnimationFrame({ time: performance.now() / 1000.0 });
        this.do_wasm_pump();
    }

    // A Rust panic traps the whole module. Linear memory survives the trap but
    // no Rust state can be trusted afterwards - the Cx of this pump was moved
    // out of the registry and never handed back - so the trap ends the runtime
    // rather than this region alone.
    do_wasm_pump() {
        if (this.runtime.fatal || this.context_lost) {
            return;
        }
        if (this.suspended) {
            this.pump_on_resume = true;
            return;
        }
        this.runtime.pumps += 1;
        this.pump_depth += 1;
        try {
            super.do_wasm_pump();
        } catch (error) {
            this.runtime.enter_fatal(error);
        } finally {
            this.pump_depth -= 1;
            const release = this.pending_release;
            if (this.pump_depth === 0 && release) {
                this.pending_release = null;
                try {
                    this.destroy();
                    release();
                } catch (error) {
                    this.runtime.enter_fatal(error);
                }
            }
        }
    }
}

/// The attribute one page-level owner of the address bar writes on the root
/// element, as `<instance>:<scope>`. It is here as well as in the SDK because
/// a trap runs no Rust destructor: after one, the only thing that can take the
/// attribute off is the host.
const URL_OWNER_ATTRIBUTE = "data-rustify-url-owner";

// The object handed to `rustify_makepad_boot`. Method names are the contract
// with the Rust `HostHooks` binding. `on_fatal` is called once, with the error
// that killed the runtime, after every region's browser resources are released.
//
// `instance` is this runtime's number on the page. It is what tells this
// instance's page-level marks from another instance's, which matters exactly
// when one of them dies and the other must not notice.
export function create_host_hooks(wasm, msg_class, on_fatal, instance = 1) {
    const regions = new Map();
    // Browser tasks the SDK asked for, by task id. The host owns them because
    // their callbacks are wasm code: a runtime that has failed drops them
    // instead of letting them re-enter a module nothing can trust.
    const tasks = new Set();
    const deferred = new Map();
    // Timers for the tasks that asked to wait. Held so a runtime that has
    // failed can cancel them: a message already posted cannot be unposted, but
    // a timer that has not fired can be stopped before it re-enters the module.
    const delays = new Map();
    let next_task = 1;
    let signal_pump_scheduled = false;

    // A turn of the browser, taken through a message port rather than a
    // timer. From the fifth nested timer on, `setTimeout(…, 0)` is clamped to
    // at least 4 ms: 150 chained timers measured 716-743 ms on this machine
    // against 0-4 ms for 150 port messages. Work that yields once a slice
    // would spend its whole budget on the clamp.
    const turns = new MessageChannel();
    const run_task = (id) => {
        const callback = deferred.get(id);
        if (callback === undefined) {
            return;
        }
        deferred.delete(id);
        tasks.delete(id);
        delays.delete(id);
        try {
            callback();
        } catch (error) {
            runtime.enter_fatal(error);
        }
    };
    turns.port1.onmessage = (event) => run_task(event.data);

    // Makepad's UI/action signals are process-wide flags, so one read per
    // runtime serves every live region. Rust raises the flags and calls
    // `request_signal_pump`, so nothing polls for an edge that is almost
    // never there.
    const drain_signals = () => {
        if (runtime.fatal || regions.size === 0) {
            return;
        }
        let flags;
        try {
            flags = wasm.exports.wasm_check_signal();
        } catch (error) {
            runtime.enter_fatal(error);
            return;
        }
        if (flags === 0) {
            return;
        }
        for (const host of [...regions.values()]) {
            if (!host.destroyed && host.to_wasm) {
                host.to_wasm.ToWasmSignal({ flags });
                host.do_wasm_pump();
            }
        }
    };
    // Signals are raised inside a pump as often as an action is posted; the
    // read is deferred to one microtask so a pump wakes the runtime once.
    const schedule_signal_pump = () => {
        if (signal_pump_scheduled || runtime.fatal || regions.size === 0) {
            return;
        }
        signal_pump_scheduled = true;
        queueMicrotask(() => {
            signal_pump_scheduled = false;
            drain_signals();
        });
    };

    // Every page-level listener this instance takes out is registered with
    // this signal, on both sides of the boundary: the SDK asks for it through
    // `listener_options`, and the loader uses it for its own. Aborting it is
    // the only removal a trap can perform, because `Drop` does not run after
    // one.
    const controller = new AbortController();

    const runtime = {
        fatal: null,
        instance,
        signal: controller.signal,
        // Bounded: a runtime that keeps failing must not grow an unbounded log.
        errors: [],
        pumps: 0,
        frames: 0,
        record_error(message) {
            if (runtime.errors.length < 64) {
                runtime.errors.push(message);
            }
            console.error(`[rustify] ${message}`);
        },
        // What this runtime currently holds on the browser's side, so a host
        // can compare teardown against the baseline it started from.
        stats() {
            let timers = 0;
            let animation_frames = 0;
            let gpu_bytes = 0;
            for (const host of regions.values()) {
                timers += host.timers.length;
                if (host.req_anim_frame_id) {
                    animation_frames += 1;
                }
                gpu_bytes += host.gpu_bytes || 0;
            }
            return {
                regions: regions.size,
                timers,
                animation_frames,
                tasks: tasks.size,
                errors: runtime.errors.length,
                // What the regions have asked the GPU to hold, as their own
                // ledgers count it: every buffer and texture uploaded through
                // the renderer, and nothing the driver adds around them. A
                // region that has been destroyed is not in this map, so a
                // runtime with no regions holds nothing.
                gpu_bytes,
                // Linear memory never shrinks, so a leak shows up as growth
                // that keeps pace with the number of rounds.
                memory: wasm._memory.buffer.byteLength,
                pumps: runtime.pumps,
                // Frames this runtime presented. What a frame budget counts:
                // see `FromWasmBeginRenderCanvas`.
                frames: runtime.frames,
            };
        },
        // The order here is the contract, not a preference.
        //
        // The listeners go first: until they are gone, a `popstate` or a
        // `scroll` arriving while the rest of this runs would re-enter a
        // module that cannot be trusted. The page-level marks go next, so the
        // page is free for a replacement before anything slower happens. Only
        // then are the tasks dropped and the regions torn down, and `on_fatal`
        // is last because it is the page's turn to show something.
        enter_fatal(error) {
            if (runtime.fatal) {
                return;
            }
            runtime.fatal = error;
            controller.abort();
            const root = document.documentElement;
            const owner = root.getAttribute(URL_OWNER_ATTRIBUTE);
            if (owner !== null && owner.startsWith(`${instance}:`)) {
                root.removeAttribute(URL_OWNER_ATTRIBUTE);
            }
            // Messages already posted still arrive; dropping the callbacks is
            // what keeps them from re-entering the module. A timer has not
            // been posted yet, so it can be stopped outright.
            for (const timer of delays.values()) {
                clearTimeout(timer);
            }
            delays.clear();
            deferred.clear();
            tasks.clear();
            const live = [...regions.values()];
            regions.clear();
            for (const host of live) {
                host.destroy();
            }
            if (on_fatal) {
                on_fatal(error);
            } else {
                console.error(error);
            }
        },
    };

    return {
        regions,
        runtime,
        /// Aborted when this instance fails, and what every page-level
        /// listener it takes out is registered with.
        signal: controller.signal,
        create_region(region, canvas) {
            if (runtime.fatal) {
                return false;
            }
            const host = new EmbeddedRegion(wasm, canvas, region, msg_class, runtime);
            if (!host.gl) {
                host.destroy();
                runtime.record_error(`region ${region}: no WebGL2 context for the canvas`);
                return false;
            }
            regions.set(region, host);
            // Flags raised before this region existed still need a reader.
            schedule_signal_pump();
            return true;
        },
        destroy_region(region) {
            const host = regions.get(region);
            if (host) {
                regions.delete(region);
                // An application can close a region from inside the action
                // callback its own pump is delivering, so both halves of the
                // region outlive that call: the browser resources until the
                // pump leaves the stack, the Cx until the batch that pump
                // returned has been drawn and freed.
                host.destroy_when_pump_returns(() => {
                    if (runtime.fatal) {
                        return;
                    }
                    wasm.exports.rustify_region_release(region);
                });
            }
        },
        request_pump(region) {
            const host = regions.get(region);
            if (host) {
                host.request_pump();
            }
        },
        request_signal_pump() {
            schedule_signal_pump();
        },
        defer(callback) {
            if (runtime.fatal) {
                return;
            }
            const id = next_task;
            next_task += 1;
            deferred.set(id, callback);
            tasks.add(id);
            turns.port2.postMessage(id);
        },
        // The same, after a wait. A region that has to ask the browser for
        // something again in a quarter of a second owns that wait through the
        // runtime for the reason `defer` does: the callback is wasm code, and
        // a module that has trapped must not be re-entered.
        defer_after(callback, ms) {
            if (runtime.fatal) {
                return;
            }
            const id = next_task;
            next_task += 1;
            deferred.set(id, callback);
            tasks.add(id);
            delays.set(id, setTimeout(() => run_task(id), ms));
        },
    };
}
