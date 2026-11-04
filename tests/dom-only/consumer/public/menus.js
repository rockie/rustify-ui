import { boot, show_fatal } from './rustify-dom-loader.js';
const status = document.getElementById('status');
try {
    const { app, hooks } = await boot({ glue_url: './pkg/app.js', wasm_url: './pkg/app_bg.wasm' });
    app.menu_benchmark_mount();
    const text = (name) => document.querySelector(`[data-testid="bench-${name}"]`).textContent;
    window.__menu_bench = {
        policy: (accept) => app.menu_benchmark_policy(accept),
        snapshot: () => ({ requests: Number(text('requests')), accepts: Number(text('accepts')),
            commands: Number(text('commands')), last: text('last'), open: text('open') === 'true' }),
        stats: () => ({ ...hooks.runtime.stats(), nodes: document.querySelectorAll('*').length,
            geometry_rules: document.adoptedStyleSheets.reduce((n, sheet) => n + sheet.cssRules.length, 0) }),
    };
    status.dataset.status = 'ready';
    status.textContent = 'Ready';
} catch (error) { show_fatal(status, error); }
