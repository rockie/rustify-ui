import { boot, release_container, show_fatal } from './rustify-dom-loader.js';
const status = document.getElementById('status');
try {
    const { app, hooks } = await boot({ glue_url: './pkg/app.js', wasm_url: './pkg/app_bg.wasm', on_fatal: () => release_container(document.getElementById('menu-benchmark')) });
    const parts = new URLSearchParams(location.search).has('parts');
    if (parts) app.menus_mount(); else app.menu_benchmark_mount();
    window.__menus = { dispose: () => app.menus_dispose(), stats: () => hooks.runtime.stats(), trap: () => app.menus_trap(),
        async peer() {
            const container=document.createElement('div');container.id='menu-peer';document.body.append(container);
            const peer=await boot({ glue_url:'./pkg/app.js',wasm_url:'./pkg/app_bg.wasm' });peer.app.menus_mount_in(container.id);
            window.__menu_peer={dispose:()=>peer.app.menus_dispose(),stats:()=>peer.hooks.runtime.stats()};
        } };
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
