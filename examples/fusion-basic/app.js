import { boot, show_fatal } from "./loader.js";

const status = document.getElementById("status");
const handles = new Map();

boot({ wasm_url: new URL("./fusion-basic.wasm", import.meta.url) })
    .then(({ app, hooks }) => {
        const api = {
            hooks,
            mount(container_id) {
                const handle = app.fusion_basic_mount(container_id);
                handles.set(container_id, handle);
                return handle;
            },
            dispose(container_id) {
                const handle = handles.get(container_id);
                if (handle === undefined) {
                    return false;
                }
                handles.delete(container_id);
                return app.fusion_basic_dispose(handle);
            },
            live_regions() {
                return app.fusion_basic_live_regions();
            },
        };
        api.mount("scope-a");
        api.mount("scope-b");
        window.__fusion_basic = api;
        status.dataset.status = "ready";
        status.textContent = "ready";
    })
    .catch((error) => {
        status.dataset.status = "failed";
        show_fatal(status, error);
    });
