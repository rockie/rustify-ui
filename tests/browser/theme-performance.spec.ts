import {test,expect,waitForReady,waitForQuiet} from './support';
import {EVIDENCE,GPU_ONLY} from '../tier';
import {mkdir,writeFile} from 'node:fs/promises';
import {dirname} from 'node:path';

const snapshot=(page:any)=>page.evaluate(()=>window.__theme_studio.snapshot());
const percentile=(values:number[],p:number)=>[...values].sort((a,b)=>a-b)[Math.ceil(values.length*p)-1];

test.describe('theme update measurements',{tag:EVIDENCE},()=>{
 test.skip(({headless})=>headless,GPU_ONLY);
 test('page timestamps measure DOM commits, GPU acknowledgements, frame merging and idle',async({page,browser},info)=>{
  await page.setViewportSize({width:1440,height:900});await waitForReady(page);
  await page.getByTestId('renderer-compare').click();await page.getByTestId('theme-gpu').scrollIntoViewIfNeeded();
  await expect.poll(async()=>{const s=await snapshot(page);return s.gpu_state==='Ready'&&s.drawn_revision===s.revision;}).toBe(true);
  await waitForQuiet(page);
  await page.evaluate(()=>{
   const entries:{start_ms:number,duration_ms:number}[]=[];
   const observer=new PerformanceObserver(list=>{for(const entry of list.getEntries())entries.push({start_ms:entry.startTime,duration_ms:entry.duration});});
   observer.observe({type:'longtask'});
   (window as any).__themeLongTasks={entries,observer};
  });
  const canvas=await page.getByTestId('theme-gpu').elementHandle();
  const measurements=await page.evaluate(async()=>{
   const api=window.__theme_studio,scope=document.querySelector('[data-testid="studio-preview"]')!,sample=scope.querySelector('[data-testid="preview-primary"]')!;
   const frames=()=>new Promise<void>(resolve=>requestAnimationFrame(()=>resolve()));
   const start=api.snapshot(),regions=[...api.hooks.regions.keys()],rows:any[]=[];
   for(let i=0;i<40;i++){
    const authored=`rgb(${80+i} 40 160)`,expected=`rgb(${80+i}, 40, 160)`,at=performance.now();let cssAt=0,settledAt=0;
    const observe=()=>{const style=getComputedStyle(sample);if(!cssAt&&style.getPropertyValue('--primary').trim()===authored)cssAt=performance.now();if(!settledAt&&style.backgroundColor===expected)settledAt=performance.now();};
    const observer=new MutationObserver(observe);
    observer.observe(scope,{attributes:true,subtree:true});api.edit('primary',authored);const editedAt=performance.now();
    const snapshotAt=performance.now();let state=api.snapshot();const initialSnapshotMs=performance.now()-snapshotAt;
    const deadline=at+5000;
    while(performance.now()<deadline){state=api.snapshot();observe();const value=state.document.overrides?.light?.primary??state.document.base?.light?.primary??state.document.styles?.light?.primary;if(cssAt&&settledAt&&state.drawn_revision===state.revision&&value===authored)break;await frames();}
    observer.disconnect();const timing=state.timings.find((row:any)=>row[0]===state.revision);
    if(!cssAt||!settledAt||!timing||timing[3]===0||state.drawn_revision!==state.revision)throw new Error(`revision did not reach DOM and GPU: ${JSON.stringify({i,cssAt,settledAt,color:getComputedStyle(sample).backgroundColor,mode:state.mode,revision:state.revision,drawn:state.drawn_revision,gpu_state:state.gpu_state,timing})}`);
    rows.push({revision:state.revision,input_ms:at,css_commit_ms:cssAt,color_settled_ms:settledAt,gpu_ms:timing[3],sync_edit_ms:editedAt-at,initial_snapshot_ms:initialSnapshotMs,publish_latency_ms:timing[2]-at,css_projection_ms:cssAt-timing[2],css_commit_latency_ms:cssAt-at,color_settled_latency_ms:settledAt-at,gpu_latency_ms:timing[3]-at});
   }
   const after=api.snapshot(),beforeNoop=api.snapshot();
   const current=beforeNoop.document.overrides?.light?.primary??beforeNoop.document.base?.light?.primary??beforeNoop.document.styles?.light?.primary;
   for(let i=0;i<120;i++)api.edit('primary',current);
   await frames();await frames();const afterNoop=api.snapshot();
   return {start,after,rows,regions,beforeNoop,afterNoop};
  });
  expect(measurements.afterNoop.sdk_resolve_count).toBe(measurements.beforeNoop.sdk_resolve_count);
  expect(measurements.afterNoop.publish_count).toBe(measurements.beforeNoop.publish_count);
  expect(measurements.afterNoop.history).toEqual(measurements.beforeNoop.history);
  await page.getByTestId('editor-controls').click();await page.getByTestId('hsl-hue-slider').scrollIntoViewIfNeeded();
  const gesture=await page.evaluate(async()=>{
   const api=window.__theme_studio,input=document.querySelector('[data-testid="hsl-hue-slider"]') as HTMLInputElement;
   const before=api.snapshot();let inputs=0;const at=performance.now();
   for(let frame=0;frame<60;frame++){
    for(let j=0;j<4;j++){input.value=String(frame*2+j*.25+1);input.dispatchEvent(new Event('input',{bubbles:true}));inputs++;}
    await new Promise<void>(resolve=>requestAnimationFrame(()=>resolve()));
   }
   (document.querySelector('[data-testid="hsl-apply"]') as HTMLButtonElement).click();
   const deadline=performance.now()+10000;let after=api.snapshot();
   while(performance.now()<deadline){after=api.snapshot();if(!after.history.gesture&&after.drawn_revision===after.revision)break;await new Promise<void>(resolve=>requestAnimationFrame(()=>resolve()));}
   return {inputs,duration_ms:performance.now()-at,before,after,regions:[...api.hooks.regions.keys()]};
  });
  expect(gesture.after.history.gesture).toBe(false);expect(gesture.after.history.undo).toBe(gesture.before.history.undo+1);
  expect(gesture.after.revision).toBe(gesture.after.drawn_revision);expect(gesture.regions).toEqual(measurements.regions);
  expect(gesture.after.publish_count-gesture.before.publish_count).toBeLessThanOrEqual(61);
  expect(gesture.after.publish_count-gesture.before.publish_count).toBeLessThan(gesture.inputs);
  expect(await canvas!.evaluate(el=>el===document.querySelector('[data-testid="theme-gpu"]')&&el.isConnected)).toBe(true);
  await page.getByTestId('theme-gpu').scrollIntoViewIfNeeded();await waitForQuiet(page);
  const cdp=await page.context().newCDPSession(page);await cdp.send('Performance.enable');
  const task=async()=>{const {metrics}=await cdp.send('Performance.getMetrics');return metrics.find(m=>m.name==='TaskDuration')!.value;};
  const before=await page.evaluate(()=>window.__theme_studio.stats()),cpuBefore=await task();
  await page.evaluate(()=>new Promise(resolve=>setTimeout(resolve,5000)));
  const after=await page.evaluate(()=>window.__theme_studio.stats()),cpuAfter=await task();
  expect(after.frames-before.frames).toBeLessThanOrEqual(1);expect(after.regions).toBe(1);
  const css=measurements.rows.map(r=>r.css_commit_latency_ms),settled=measurements.rows.map(r=>r.color_settled_latency_ms),gpu=measurements.rows.map(r=>r.gpu_latency_ms);
  const environment=await page.evaluate(async()=>{
   const gl=(document.querySelector('[data-testid="theme-gpu"]') as HTMLCanvasElement).getContext('webgl2')!,extension=gl.getExtension('WEBGL_debug_renderer_info');
   const manifest=await (await fetch('./build-manifest.json')).json();
   return {collected_at:new Date().toISOString(),user_agent:navigator.userAgent,visibility:document.visibilityState,dpr:devicePixelRatio,root_font:getComputedStyle(document.documentElement).fontSize,gpu_renderer:gl.getParameter(extension?extension.UNMASKED_RENDERER_WEBGL:gl.RENDERER),webgl_version:gl.getParameter(gl.VERSION),build_id:manifest.build_id,wasm_bytes:manifest.files['theme-studio.wasm']};
  });
  const longTasks=await page.evaluate(()=>{
   const {entries,observer}=(window as any).__themeLongTasks;
   for(const entry of observer.takeRecords())entries.push({start_ms:entry.startTime,duration_ms:entry.duration});
   observer.disconnect();delete (window as any).__themeLongTasks;
   return {count:entries.length,total_ms:entries.reduce((sum:number,entry:any)=>sum+entry.duration_ms,0),max_ms:Math.max(0,...entries.map((entry:any)=>entry.duration_ms)),entries};
  });
  const stages=Object.fromEntries(['sync_edit_ms','initial_snapshot_ms','publish_latency_ms','css_projection_ms'].map(name=>{const values=measurements.rows.map(row=>row[name]);return [name,{p50:percentile(values,.5),p95:percentile(values,.95)}];}));
  const report={project:info.project.name,viewport:page.viewportSize(),environment:{...environment,browser_version:browser.version(),executable_path:process.env.RUSTIFY_CHROMIUM||null},long_tasks:longTasks,samples:40,dom_css_commit:{p50:percentile(css,.5),p95:percentile(css,.95)},dom_color_settled:{p50:percentile(settled,.5),p95:percentile(settled,.95)},gpu_draw_ack:{p50:percentile(gpu,.5),p95:percentile(gpu,.95)},stages,resolves:{editor:measurements.after.resolve_count-measurements.start.resolve_count,sdk_total:measurements.after.sdk_resolve_count-measurements.start.sdk_resolve_count},gesture:{inputs:gesture.inputs,published:gesture.after.publish_count-gesture.before.publish_count,duration_ms:gesture.duration_ms,editor_resolves:gesture.after.resolve_count-gesture.before.resolve_count,sdk_total_resolves:gesture.after.sdk_resolve_count-gesture.before.sdk_resolve_count,final_revision:gesture.after.revision,drawn_revision:gesture.after.drawn_revision},idle:{before,after,task_cpu_percent:(cpuAfter-cpuBefore)*1000/5000*100},raw:measurements.rows};
  const output=process.env.RUSTIFY_THEME_PERF_OUT||'docs/validation/theme-studio/M6/performance.json';
  await mkdir(dirname(output),{recursive:true});await writeFile(output,JSON.stringify(report,null,2));
  console.log(JSON.stringify({...report,raw:undefined}));
 });

 test('configuration changes measure native requests, complete snapshots and idle without replacing the region',async({page,browser},info)=>{
  await page.setViewportSize({width:1440,height:900});await waitForReady(page);
  for(const field of ['accent_color','gray_color','panel_background','radius']) await page.getByTestId(`studio-config-${field}-reset`).click();
  await page.getByTestId('renderer-compare').click();await page.getByTestId('theme-gpu').scrollIntoViewIfNeeded();
  await expect.poll(async()=>{const state=await snapshot(page);return state.gpu_state==='Ready'&&state.drawn_revision===state.revision;}).toBe(true);
  await waitForQuiet(page);
  const canvas=await page.getByTestId('theme-gpu').elementHandle();
  const result=await page.evaluate(async()=>{
   const api=window.__theme_studio,scope=document.querySelector('[data-testid="studio-preview"]') as HTMLElement;
   const frame=()=>new Promise<void>(resolve=>requestAnimationFrame(()=>resolve()));
   const longTasks:{start_ms:number,duration_ms:number}[]=[];
   const observer=new PerformanceObserver(list=>{for(const entry of list.getEntries())longTasks.push({start_ms:entry.startTime,duration_ms:entry.duration});});
   observer.observe({type:'longtask'});
   const start=api.snapshot(),regions=[...api.hooks.regions.keys()],rows:any[]=[];
   const choices:[string,string[]][]=[['accent_color',['pink','blue']],['gray_color',['mauve','slate']],['panel_background',['translucent','solid']],['radius',['full','small']],['scaling',['110','90']]];
   for(let i=0;i<40;i++){
    const [field,values]=choices[i%choices.length],value=values[Math.floor(i/choices.length)%2];
    const trigger=document.querySelector(`[data-testid="studio-config-${field}"]`) as HTMLElement;
    if(!trigger)throw new Error(`missing native configuration control ${field}`);
    trigger.scrollIntoView({block:'nearest'});trigger.click();
    const selector=`[data-testid="studio-config-${field}-list"] [data-testid="option-${value}"]`;
    let option=document.querySelector(selector) as HTMLElement|null;
    const opened=performance.now();
    while(!option&&performance.now()-opened<1000){await frame();option=document.querySelector(selector) as HTMLElement|null;}
    if(!option)throw new Error(`missing configuration option ${field}=${value}`);
    const before=api.snapshot(),style=scope.style.cssText,at=performance.now();let cssAt=0;
    const changed=()=>{if(!cssAt&&scope.style.cssText!==style)cssAt=performance.now();};
    const mutation=new MutationObserver(changed);mutation.observe(scope,{attributes:true,attributeFilter:['style']});
    option.click();let state=api.snapshot();
    while(performance.now()<at+5000){
     state=api.snapshot();changed();
     if(cssAt&&String(state.document.config[field])===value&&state.revision>before.revision&&state.drawn_revision===state.revision)break;
     await frame();
    }
    mutation.disconnect();const timing=state.timings.find((row:any)=>row[0]===state.revision);
    if(!cssAt||!timing?.[3]||state.drawn_revision!==state.revision||String(state.document.config[field])!==value)throw new Error(`configuration did not reach DOM and GPU: ${JSON.stringify({field,value,cssAt,state})}`);
    rows.push({field,value,revision:state.revision,input_ms:at,css_commit_ms:cssAt,gpu_ms:timing[3],css_commit_latency_ms:cssAt-at,gpu_latency_ms:timing[3]-at});
   }
   for(const entry of observer.takeRecords())longTasks.push({start_ms:entry.startTime,duration_ms:entry.duration});observer.disconnect();
   return {start,after:api.snapshot(),regions,after_regions:[...api.hooks.regions.keys()],rows,long_tasks:{count:longTasks.length,total_ms:longTasks.reduce((sum,entry)=>sum+entry.duration_ms,0),max_ms:Math.max(0,...longTasks.map(entry=>entry.duration_ms)),entries:longTasks}};
  });
  expect(result.after_regions).toEqual(result.regions);expect(result.after.drawn_revision).toBe(result.after.revision);
  expect(result.after.publish_count-result.start.publish_count).toBe(40);
  expect(await canvas!.evaluate(element=>element===document.querySelector('[data-testid="theme-gpu"]')&&element.isConnected)).toBe(true);
  await page.getByTestId('theme-gpu').scrollIntoViewIfNeeded();await waitForQuiet(page);
  const before=await page.evaluate(()=>window.__theme_studio.stats());
  await page.evaluate(()=>new Promise(resolve=>setTimeout(resolve,5000)));
  const after=await page.evaluate(()=>window.__theme_studio.stats());
  expect(after.frames-before.frames).toBeLessThanOrEqual(1);expect(after.pumps-before.pumps).toBe(0);expect(after.regions).toBe(1);
  const environment=await page.evaluate(async()=>{
   const gl=(document.querySelector('[data-testid="theme-gpu"]') as HTMLCanvasElement).getContext('webgl2')!,extension=gl.getExtension('WEBGL_debug_renderer_info');
   const manifest=await(await fetch('./build-manifest.json')).json();
   return {collected_at:new Date().toISOString(),dpr:devicePixelRatio,root_font:getComputedStyle(document.documentElement).fontSize,gpu_renderer:gl.getParameter(extension?extension.UNMASKED_RENDERER_WEBGL:gl.RENDERER),build_id:manifest.build_id,wasm_bytes:manifest.files['theme-studio.wasm']};
  });
  const css=result.rows.map(row=>row.css_commit_latency_ms),gpu=result.rows.map(row=>row.gpu_latency_ms);
  const report={project:info.project.name,viewport:page.viewportSize(),environment:{...environment,browser_version:browser.version(),executable_path:process.env.RUSTIFY_CHROMIUM||null},samples:40,dom_css_commit:{p50:percentile(css,.5),p95:percentile(css,.95)},gpu_draw_ack:{p50:percentile(gpu,.5),p95:percentile(gpu,.95)},resolves:{editor:result.after.resolve_count-result.start.resolve_count,sdk_total:result.after.sdk_resolve_count-result.start.sdk_resolve_count},published:result.after.publish_count-result.start.publish_count,final_revision:result.after.revision,drawn_revision:result.after.drawn_revision,final_config:result.after.document.config,regions:result.regions,long_tasks:result.long_tasks,idle:{before,after},raw:result.rows};
  const output=process.env.RUSTIFY_THEME_CONFIG_PERF_OUT||'docs/validation/dom-components-p1/performance-config.json';
  await mkdir(dirname(output),{recursive:true});await writeFile(output,JSON.stringify(report,null,2));
  console.log(JSON.stringify({...report,raw:undefined}));
 });
});
