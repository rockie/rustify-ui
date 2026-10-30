import { boot, show_fatal } from './rustify-dom-loader.js';

const status = document.getElementById('status');
try {
    const { app, hooks } = await boot({ glue_url: './pkg/app.js', wasm_url: './pkg/app_bg.wasm' });
    app.form_benchmark_mount();
    const text = (id) => document.querySelector(`[data-testid="${id}"]`).textContent;
    window.__form_bench = {
        reset: () => app.form_benchmark_reset(),
        policy: (accept, readOnly = false, disabled = false) => app.form_benchmark_policy(accept, readOnly, disabled),
        snapshot: () => ({
            checked: text('accepted-check') === 'true', text: text('accepted-text'),
            slider: Number(text('accepted-slider')), selected: text('accepted-select'),
            requests: Number(text('requests')), accepts: Number(text('accepts')), commits: 0,
        }),
        stats: () => hooks.runtime.stats(),
        measurements: [],
    };
    status.dataset.status = 'ready';
    status.textContent = 'Ready';
} catch (error) {
    show_fatal(status, error);
}
