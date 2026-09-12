import { boot, release_container, show_fatal, StartupError } from "./loader.js";

const status = document.getElementById("status");
const wasm_url = new URL("./data-workbench.wasm", import.meta.url);

let handle = null;

const runtime_fatal = (error) => {
    // The mounted views and their listeners live in the module that just
    // trapped, so they have to go with it. Removing the nodes is a JS-only
    // path: calling the application's dispose would re-enter that module.
    release_container(document.getElementById("workbench"));
    handle = null;
    delete window.__data_workbench;
    status.dataset.status = "fatal";
    show_fatal(
        status,
        new StartupError("RuntimeFatal", `${error}; reload the page, the sample is generated again`)
    );
};

boot({ wasm_url, on_fatal: runtime_fatal })
    .then(({ app, hooks, build, instance }) => {
        app.data_workbench_identify(instance, build);
        window.__data_workbench = {
            hooks,
            instance,
            mount(container_id = "workbench") {
                handle = app.data_workbench_mount(container_id);
                return handle;
            },
            dispose() {
                if (handle === null) {
                    return false;
                }
                const spent = app.data_workbench_dispose(handle);
                handle = null;
                return spent;
            },
            snapshot() {
                return JSON.parse(app.data_workbench_snapshot());
            },
            diagnostics() {
                return JSON.parse(app.data_workbench_diagnostics());
            },
            live_regions() {
                return app.data_workbench_live_regions();
            },
            errors() {
                return hooks.runtime.errors;
            },
            stats() {
                return hooks.runtime.stats();
            },
            /// The checksum over every cell, as a decimal string: JSON has no
            /// sixty-four bit integer, and this one is compared, not counted.
            dataset_hash() {
                return app.data_workbench_dataset_hash();
            },
            row_id(row) {
                return app.data_workbench_row_id(row);
            },
            cell(row, column) {
                return app.data_workbench_cell(row, column);
            },
            look_at(x, y) {
                return app.data_workbench_look_at(x, y);
            },
            freeze_scene(on) {
                return app.data_workbench_freeze_scene(on);
            },
            scene_visible() {
                return app.data_workbench_scene_visible();
            },
            /// The objects chosen in the scene, in order.
            scene_chosen() {
                return Array.from(app.data_workbench_scene_chosen());
            },
            scene_label(id) {
                return app.data_workbench_scene_label(id);
            },
            /// How many times the table has worked out its visible range: the
            /// table's content version, for a frame budget.
            window_version() {
                return Number(app.data_workbench_window_version());
            },
            selected() {
                return Array.from(app.data_workbench_selected());
            },
            select_rows(from, count) {
                return Array.from(app.data_workbench_select_rows(from, count));
            },
            open_row(row) {
                return app.data_workbench_open_row(row);
            },
            /// A window of the view, as sample positions: what the table is
            /// showing, in the order it is showing it.
            view(from, count) {
                return Array.from(app.data_workbench_view(from, count));
            },
            sort(column, ascending = true) {
                return app.data_workbench_sort(column, ascending);
            },
            cancel_job() {
                return app.data_workbench_cancel_job();
            },
            sort_probe(column, ascending = true) {
                return app.data_workbench_sort_probe(column, ascending);
            },
            sort_probe_state() {
                return JSON.parse(app.data_workbench_sort_probe_state());
            },
            sort_probe_order(from, count) {
                return Array.from(app.data_workbench_sort_probe_order(from, count));
            },
        };
        window.__data_workbench.mount();
        status.dataset.status = "ready";
        status.textContent = "ready";
    })
    .catch((error) => {
        status.dataset.status = "failed";
        show_fatal(status, error);
    });
