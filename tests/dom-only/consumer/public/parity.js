import { boot, release_container, show_fatal } from './rustify-dom-loader.js';
const status = document.getElementById('status');
try {
    const { app } = await boot({ glue_url: './pkg/app.js', wasm_url: './pkg/app_bg.wasm',
        on_fatal: () => release_container(document.getElementById('parity')) });
    app.parity_mount();
    window.__parity = {
        policy: (component, policy) => app.parity_policy(component, policy),
        dispose: () => app.parity_dispose(),
        snapshot(component) {
            const row = document.querySelector(`[data-parity="${component}"]`);
            if (!row) throw new Error(`Unknown parity component: ${component}`);
            const requests = row.querySelector('[data-requests]').textContent;
            return { value: row.querySelector('[data-value]').textContent,
                requests: requests ? requests.split('\n') : [],
                focus: document.activeElement?.getAttribute('data-testid'),
                open: component === 'hover-card' ? !!document.querySelector('[data-testid="hover-card-content"]')
                    : document.querySelector('[data-testid="select"]')?.getAttribute('aria-expanded') === 'true' };
        },
        formData: () => Object.fromEntries(new FormData(document.getElementById('parity-form'))),
    };
    status.dataset.status = 'ready';status.textContent = 'Ready';
} catch (error) { show_fatal(status, error); }
