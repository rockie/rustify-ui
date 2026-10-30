import { boot, show_fatal } from './rustify-dom-loader.js';
const status = document.getElementById('status');
try {
    const { app, hooks } = await boot({ glue_url: './pkg/app.js', wasm_url: './pkg/app_bg.wasm' });
    app.field_samples_mount();
    window.__field_samples = { dispose: () => app.field_samples_dispose(), stats: () => hooks.runtime.stats() };
    status.dataset.status = 'ready'; status.textContent = 'Ready';
} catch (error) { show_fatal(status, error); }
