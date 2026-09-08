import { WasmWebGL } from "../makepad_platform/web_gl.js";

// One Makepad region drawn into a canvas the host page owns. The Rust side
// creates the Cx first and hands the region id in; this object only holds
// browser resources (GL context, listeners, timers) and releases all of them
// in `destroy()`.
export class EmbeddedRegion extends WasmWebGL {
    constructor(wasm, canvas, region, msg_class) {
        super(wasm, undefined, canvas, { embedded: true, region, msg_class });
        this.pump_scheduled = false;
    }

    // Rust asks for a pump after it queued work for this region; several
    // requests in one task collapse into one pump on the microtask queue.
    request_pump() {
        if (this.pump_scheduled || this.destroyed) {
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
}

// The object handed to `rustify_makepad_boot`. Method names are the contract
// with the Rust `HostHooks` binding.
export function create_host_hooks(wasm, msg_class) {
    const regions = new Map();
    let signal_timer = null;

    // Makepad's UI/action signals are process-wide flags, so one poll per
    // runtime reads them and every live region gets the signal event. The
    // poll only runs while regions exist.
    const poll_signals = () => {
        const flags = wasm.exports.wasm_check_signal();
        if (flags === 0) {
            return;
        }
        for (const host of regions.values()) {
            if (!host.destroyed && host.to_wasm) {
                host.to_wasm.ToWasmSignal({ flags });
                host.do_wasm_pump();
            }
        }
    };
    const update_signal_poll = () => {
        if (regions.size > 0 && signal_timer === null) {
            signal_timer = window.setInterval(poll_signals, 16);
        } else if (regions.size === 0 && signal_timer !== null) {
            window.clearInterval(signal_timer);
            signal_timer = null;
        }
    };

    return {
        regions,
        create_region(region, canvas) {
            const host = new EmbeddedRegion(wasm, canvas, region, msg_class);
            if (!host.gl) {
                host.destroy();
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
