import { WasmWebGL } from "../makepad_platform/web_gl.js";

// One Makepad region drawn into a canvas the host page owns. The Rust side
// creates the Cx first and hands the region id in; this object only holds
// browser resources (GL context, listeners, timers) and releases all of them
// in `destroy()`.
export class EmbeddedRegion extends WasmWebGL {
    constructor(wasm, canvas, region, msg_class, runtime) {
        super(wasm, undefined, canvas, { embedded: true, region, msg_class });
        this.runtime = runtime;
        this.pump_scheduled = false;
    }

    // Rust asks for a pump after it queued work for this region; several
    // requests in one task collapse into one pump on the microtask queue.
    request_pump() {
        if (this.pump_scheduled || this.destroyed || this.runtime.fatal) {
            return;
        }
        this.pump_scheduled = true;
        queueMicrotask(() => {
            this.pump_scheduled = false;
            if (!this.destroyed && this.to_wasm) {
                this.do_wasm_pump();
            }
        });
    }

    // A Rust panic traps the whole module. Linear memory survives the trap but
    // no Rust state can be trusted afterwards - the Cx of this pump was moved
    // out of the registry and never handed back - so the trap ends the runtime
    // rather than this region alone.
    do_wasm_pump() {
        if (this.runtime.fatal) {
            return;
        }
        try {
            super.do_wasm_pump();
        } catch (error) {
            this.runtime.enter_fatal(error);
        }
    }
}

// The object handed to `rustify_makepad_boot`. Method names are the contract
// with the Rust `HostHooks` binding. `on_fatal` is called once, with the error
// that killed the runtime, after every region's browser resources are released.
export function create_host_hooks(wasm, msg_class, on_fatal) {
    const regions = new Map();
    let signal_timer = null;

    // Makepad's UI/action signals are process-wide flags, so one poll per
    // runtime reads them and every live region gets the signal event. The
    // poll only runs while regions exist.
    const poll_signals = () => {
        if (runtime.fatal) {
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
    const stop_signal_poll = () => {
        if (signal_timer !== null) {
            window.clearInterval(signal_timer);
            signal_timer = null;
        }
    };
    const update_signal_poll = () => {
        if (regions.size > 0 && signal_timer === null && !runtime.fatal) {
            signal_timer = window.setInterval(poll_signals, 16);
        } else if (regions.size === 0) {
            stop_signal_poll();
        }
    };

    const runtime = {
        fatal: null,
        // Bounded: a runtime that keeps failing must not grow an unbounded log.
        errors: [],
        record_error(message) {
            if (runtime.errors.length < 64) {
                runtime.errors.push(message);
            }
            console.error(`[rustify] ${message}`);
        },
        enter_fatal(error) {
            if (runtime.fatal) {
                return;
            }
            runtime.fatal = error;
            stop_signal_poll();
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
            update_signal_poll();
            return true;
        },
        destroy_region(region) {
            const host = regions.get(region);
            if (host) {
                regions.delete(region);
                host.destroy();
            }
            update_signal_poll();
        },
        request_pump(region) {
            const host = regions.get(region);
            if (host) {
                host.request_pump();
            }
        },
    };
}
