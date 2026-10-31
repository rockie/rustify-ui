import { boot, show_fatal } from './rustify-dom-loader.js';
const status = document.getElementById('status');
try {
    const { app, hooks } = await boot({ glue_url: './pkg/app.js', wasm_url: './pkg/app_bg.wasm' });
    app.panel_benchmark_mount();
    const text = (name) => document.querySelector(`[data-testid="bench-${name}"]`).textContent;
    window.__panel_bench = {
        policy: (accept) => app.panel_benchmark_policy(accept),
        snapshot: () => ({ requests: Number(text('requests')), accepts: Number(text('accepts')),
            active: text('active'), dialog: text('dialog-state') === 'true', tooltip: text('tooltip-state') === 'true',
            progress: Number(text('progress-state')) }),
        stats: () => ({ ...hooks.runtime.stats(), nodes: document.querySelectorAll('*').length,
            geometry_rules: document.adoptedStyleSheets.reduce((n, sheet) => n + sheet.cssRules.length, 0) }),
    };
    status.dataset.status = 'ready';
    status.textContent = 'Ready';
} catch (error) { show_fatal(status, error); }
