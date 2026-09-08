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
        this.runtime.pumps += 1;
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
    let signal_pump_scheduled = false;

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

    const runtime = {
        fatal: null,
        // Bounded: a runtime that keeps failing must not grow an unbounded log.
        errors: [],
        pumps: 0,
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
            for (const host of regions.values()) {
                timers += host.timers.length;
                if (host.req_anim_frame_id) {
                    animation_frames += 1;
                }
            }
            return {
                regions: regions.size,
                timers,
                animation_frames,
                errors: runtime.errors.length,
                // Linear memory never shrinks, so a leak shows up as growth
                // that keeps pace with the number of rounds.
                memory: wasm._memory.buffer.byteLength,
                pumps: runtime.pumps,
            };
        },
        enter_fatal(error) {
            if (runtime.fatal) {
                return;
            }
            runtime.fatal = error;
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
            // Flags raised before this region existed still need a reader.
            schedule_signal_pump();
            return true;
        },
        destroy_region(region) {
            const host = regions.get(region);
            if (host) {
                regions.delete(region);
                host.destroy();
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
    };
}
