import { boot, release_container, show_fatal, StartupError } from "./loader.js";
const status = document.getElementById("status");
const containers = ["studio-editor", "studio-preview", "studio-compare"];
const wasm_url = new URL("./theme-studio.wasm", import.meta.url);
let live = null;
let id = null;
let observer = null;
let controller = null;
const environment = () => ({
  rem: Number.parseFloat(getComputedStyle(document.documentElement).fontSize),
  reduce: matchMedia("(prefers-reduced-motion: reduce)").matches,
});
function fatal(error) {
  observer?.disconnect(); controller?.abort();
  containers.forEach((name) => release_container(document.getElementById(name)));
  id = null; delete window.__theme_studio;
  status.dataset.status = "fatal";
  const died = live;
  live = null;
  show_fatal(status, new StartupError("RuntimeFatal", `${error}; restart to restore the last saved draft.`), {
    restart: died && died.restarts < died.restart_limit ? async () => {
      try { const next = await died.restart({ on_fatal: fatal }); if (next) publish(next); }
      catch (error) { status.dataset.status="failed";show_fatal(status,error); }
    } : null,
  });
}
function publish(started) {
  live = started;
  const { app, hooks, build } = started;
  app.theme_studio_identify(started.instance,build);
  const handle = {
    hooks,
    mount() {
      if (id !== null) return id;
      const { rem, reduce } = environment();
      id = app.theme_studio_mount(...containers,rem,reduce);
      controller = new AbortController();
      window.addEventListener("resize",handle.environment,{signal:controller.signal});
      matchMedia("(prefers-reduced-motion: reduce)").addEventListener("change",handle.environment,{signal:controller.signal});
      observer = new MutationObserver(handle.environment);
      observer.observe(document.documentElement,{attributes:true,attributeFilter:["style","class"]});
      return id;
    },
    dispose() { observer?.disconnect(); controller?.abort(); if (id===null) return false; const result=app.theme_studio_dispose(id);id=null;return result; },
    async reset() {
      if(id===null) handle.mount(); else app.theme_studio_reset(id);
      await new Promise((resolve)=>requestAnimationFrame(()=>requestAnimationFrame(resolve)));
      document.activeElement?.blur();window.scrollTo(0,0);hooks.runtime.errors.length=0;
    },
    edit(token,value) {return app.theme_studio_edit(id,token,value);},
    snapshot() {return JSON.parse(app.theme_studio_snapshot(id));},
    diagnostics() {return JSON.parse(app.theme_studio_diagnostics());},
    stats() {return hooks.runtime.stats();},
    environment() { const {rem,reduce}=environment();app.theme_studio_environment(id,rem,reduce); },
  };
  window.__theme_studio=handle;
  handle.mount();
  status.dataset.status="ready";status.textContent="ready";
}
boot({wasm_url,on_fatal:fatal}).then(publish).catch((error)=>{status.dataset.status="failed";show_fatal(status,error);});
