import { boot, release_container, show_fatal } from './rustify-dom-loader.js';
const status = document.getElementById('status');
try {
    const { app, hooks } = await boot({ glue_url: './pkg/app.js', wasm_url: './pkg/app_bg.wasm', on_fatal: () => release_container(document.getElementById('app')) });
    app.primitives_mount();
    window.__primitives = { dispose: () => app.primitives_dispose(), stats: () => hooks.runtime.stats() };
    window.__menus = { dispose: () => app.primitives_dispose(), stats: () => hooks.runtime.stats(), trap: () => app.menus_trap(),
        async peer() {
            const container=document.createElement('div');container.id='menu-peer';document.body.append(container);
            const peer=await boot({glue_url:'./pkg/app.js',wasm_url:'./pkg/app_bg.wasm'});peer.app.menus_mount_in(container.id);
            window.__menu_peer={dispose:()=>peer.app.menus_dispose(),stats:()=>peer.hooks.runtime.stats()};
        } };
    status.dataset.status = 'ready';
    status.textContent = 'Ready';
} catch (error) {
    show_fatal(status, error);
}
