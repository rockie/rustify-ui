import {test,expect,Page,Locator} from '@playwright/test';
const failures=new WeakMap<Page,string[]>();
const intentionalFailures=new WeakMap<Page,Set<string>>();

async function openTools(page:Page){
    const details=page.getByTestId('content-tools');
    await details.locator('summary').click();await expect(details).toHaveAttribute('open','');
    await expect(page.getByTestId('content-scroll')).toHaveAttribute('data-y-overflow','');
}
async function frames(page:Page,count=3){
    await page.evaluate(async count=>{const frame=(window as any).contentNativeFrame as typeof requestAnimationFrame;for(let i=0;i<count;i++)await new Promise<void>(resolve=>frame(()=>resolve()));},count);
}
async function setVisibility(page:Page,mode:string){
    const root=page.getByTestId('content-scroll');
    for(let i=0;i<4 && await root.getAttribute('data-visibility')!==mode;i++)await page.getByTestId('content-scroll-visibility').click();
    await expect(root).toHaveAttribute('data-visibility',mode);
}
async function setAxes(page:Page,mode:string){
    const root=page.getByTestId('content-scroll');
    for(let i=0;i<3 && await root.getAttribute('data-scrollbars')!==mode;i++)await page.getByTestId('content-scroll-axes').click();
    await expect(root).toHaveAttribute('data-scrollbars',mode);
}
async function scrollGeometry(viewport:Locator){
    return viewport.evaluate(element=>({x:element.scrollLeft,y:element.scrollTop,width:element.clientWidth,height:element.clientHeight,fullWidth:element.scrollWidth,fullHeight:element.scrollHeight}));
}

test.beforeEach(async({page},info)=>{
    const errors:string[]=[];failures.set(page,errors);intentionalFailures.set(page,new Set());
    page.on('pageerror',error=>errors.push(error.message));
    page.on('console',message=>{if(message.type()==='error')errors.push(message.text());});
    page.on('requestfailed',request=>{if(![...(intentionalFailures.get(page)||[])].some(path=>request.url().endsWith(path)))errors.push(`${request.url()}: ${request.failure()?.errorText}`);});
    await page.addInitScript(()=>{
        const activeObservers=new Map<ResizeObserver,Set<Element>>();
        const NativeObserver=window.ResizeObserver;
        window.ResizeObserver=class extends NativeObserver {
            targets=new Set<Element>();
            observe(target:Element,options?:ResizeObserverOptions){this.targets.add(target);activeObservers.set(this,this.targets);super.observe(target,options);}
            unobserve(target:Element){this.targets.delete(target);if(this.targets.size===0)activeObservers.delete(this);super.unobserve(target);}
            disconnect(){activeObservers.delete(this);this.targets.clear();super.disconnect();}
        };
        const nativeFrame=window.requestAnimationFrame.bind(window),cancel=window.cancelAnimationFrame.bind(window);
        const frameRecords=new Map<number,{called:boolean,cancelled:boolean}>(),pendingFrames=new Set<number>();
        (window as any).contentNativeFrame=nativeFrame;
        window.requestAnimationFrame=callback=>{const id=nativeFrame(time=>{pendingFrames.delete(id);frameRecords.get(id)!.called=true;callback(time);});pendingFrames.add(id);frameRecords.set(id,{called:false,cancelled:false});return id;};
        window.cancelAnimationFrame=id=>{pendingFrames.delete(id);const record=frameRecords.get(id);if(record)record.cancelled=true;cancel(id);};
        const schedule=window.setTimeout.bind(window),clear=window.clearTimeout.bind(window),timers=new Set<number>();
        window.setTimeout=((handler:TimerHandler,delay?:number,...args:any[])=>{
            if(delay!==80 || typeof handler!=='function')return schedule(handler,delay,...args);
            const id=schedule(()=>{timers.delete(id);handler(...args);},delay);timers.add(id);return id;
        }) as typeof setTimeout;
        window.clearTimeout=id=>{if(id!==undefined)timers.delete(id);clear(id);};
        const add=EventTarget.prototype.addEventListener,remove=EventTarget.prototype.removeEventListener;
        const watched=new Set(['pointermove','pointerup','pointercancel']),listeners=new Map<any,string>();
        EventTarget.prototype.addEventListener=function(type:string,listener:any,options:any){if(this===document && watched.has(type) && listener)listeners.set(listener,type);return add.call(this,type,listener,options);};
        EventTarget.prototype.removeEventListener=function(type:string,listener:any,options:any){if(this===document && watched.has(type))listeners.delete(listener);return remove.call(this,type,listener,options);};
        (window as any).contentResources={activeObservers,frameRecords,pendingFrames,timers,listeners};
    });
    await page.goto(info.project.name==='dom-primitives'?'/':'/panels.html');
    await expect(page.locator('#status')).toHaveAttribute('data-status','ready');
    await expect.poll(()=>page.locator('link[href="content.css"]').evaluate(element=>!!(element as HTMLLinkElement).sheet)).toBe(true);
    await openTools(page);
    await expect(page.getByTestId('content-avatar')).toHaveAttribute('data-state','loaded');
});
test.afterEach(async({page})=>{expect(failures.get(page)).toEqual([]);});

test('avatar uses real URL images, readable failure fallback, and cached recovery',async({page})=>{
    const root=page.getByTestId('content-avatar'),image=page.getByTestId('content-avatar-image'),fallback=page.getByTestId('content-avatar-fallback');
    await expect(root).toHaveAttribute('data-state','loaded');await expect(image).toHaveAttribute('alt','Jamie Morgan');await expect(image).toHaveAttribute('data-caller','retained');
    await expect(fallback).toBeHidden();await expect(root).toHaveAccessibleName('Jamie Morgan');
    await expect.poll(()=>image.evaluate(element=>(element as HTMLImageElement).naturalWidth)).toBeGreaterThan(0);
    await page.getByTestId('content-photo-broken').click();await expect(root).toHaveAttribute('data-state','error');await expect(fallback).toBeVisible();await expect(fallback).toHaveText('JM');
    await expect(image).toHaveAttribute('inert','');await expect(image).toHaveCSS('opacity','0');
    await page.getByTestId('content-photo-good').click();await expect(root).toHaveAttribute('data-state','loaded');await expect(fallback).toBeHidden();
    await page.getByTestId('content-photo-empty').click();await expect(root).toHaveAttribute('data-state','error');await expect(fallback).toBeVisible();await expect(image).not.toHaveAttribute('src',/.+/);
});

test('delayed fallback cancels for a newer source and stale image events cannot replace it',async({page})=>{
    const root=page.getByTestId('content-avatar'),image=page.getByTestId('content-avatar-image'),fallback=page.getByTestId('content-avatar-fallback');
    await expect(root).toHaveAttribute('data-state','loaded');
    let release!:()=>void;const gate=new Promise<void>(resolve=>release=resolve);
    let finished!:()=>void;const settled=new Promise<void>(resolve=>finished=resolve);
    intentionalFailures.get(page)!.add('/content-avatar-slow.svg');
    await page.route('**/content-avatar-slow.svg',async route=>{try{const response=await route.fetch();await gate;await route.fulfill({response});}finally{finished();}});
    try{
        const request=page.waitForRequest('**/content-avatar-slow.svg');
        const initial=await page.evaluate(async()=>{document.querySelector<HTMLButtonElement>('[data-testid="content-photo-slow"]')!.click();await Promise.resolve();await Promise.resolve();return {visible:document.querySelector('[data-testid="content-avatar-fallback"]')!.getAttribute('data-visible'),timers:(window as any).contentResources.timers.size};});
        await request;expect(initial.visible).toBe('false');expect(initial.timers).toBeGreaterThan(0);
        await expect(root).toHaveAttribute('data-state','loading');await expect(fallback).toBeVisible();
        const old=await image.elementHandle();expect(old).not.toBeNull();
        await page.getByTestId('content-photo-good').click();await expect(root).toHaveAttribute('data-state','loaded');await image.evaluate(element=>(element as HTMLElement).dataset.identity='latest');
        release();await settled;await old!.evaluate(element=>{element.dispatchEvent(new Event('error'));element.dispatchEvent(new Event('load'));});
        await expect(image).toHaveAttribute('src','content-avatar.svg');await expect(image).toHaveAttribute('data-identity','latest');await expect(root).toHaveAttribute('data-state','loaded');await expect(fallback).toBeHidden();
    }finally{release();}
});

test('aspect ratio preserves last valid geometry and separator semantics never add a tab stop',async({page})=>{
    const ratio=page.getByTestId('content-ratio'),separator=page.getByTestId('content-separator');
    const shape=async()=>ratio.evaluate(element=>{const box=element.getBoundingClientRect();return box.width/box.height;});
    await expect.poll(shape).toBeCloseTo(1.5,2);await page.getByTestId('content-ratio-square').click();await expect.poll(shape).toBeCloseTo(1,2);
    await page.getByTestId('content-ratio-invalid').click();await expect(ratio).toHaveAttribute('data-rustify-issue','ratio');await expect.poll(shape).toBeCloseTo(1,2);
    await page.getByTestId('content-ratio-wide').click();await expect(ratio).not.toHaveAttribute('data-rustify-issue');await expect.poll(shape).toBeCloseTo(1.5,2);
    await expect(separator).toHaveRole('separator');await expect(separator).toHaveAttribute('aria-orientation','horizontal');
    await page.getByTestId('content-separator-direction').click();await expect(separator).toHaveAttribute('aria-orientation','vertical');
    await expect(separator).not.toHaveAttribute('tabindex');await page.getByTestId('content-separator-decorative').click();await expect(separator).toHaveAttribute('role','presentation');await expect(separator).toHaveAttribute('aria-hidden','true');await expect(separator).not.toHaveAttribute('aria-orientation');
});

test('viewport wheel stays nonblocking while the custom scrollbar consumes its own wheel input',async({page})=>{
    const viewport=page.getByTestId('content-scroll-viewport');
    await viewport.evaluate(element=>element.addEventListener('wheel',event=>{
        element.setAttribute('data-wheel-cancelable',String(event.cancelable));
    },{passive:true,once:true}));
    await viewport.hover();await page.mouse.wheel(0,120);
    await expect(viewport).toHaveAttribute('data-wheel-cancelable','false');
    await expect.poll(async()=>(await scrollGeometry(viewport)).y).toBeGreaterThan(0);
    await viewport.evaluate(element=>{
        element.scrollTop=0;
        window.addEventListener('wheel',event=>element.setAttribute('data-bar-wheel-prevented',String(event.defaultPrevented)),{passive:true,once:true});
    });
    await page.getByTestId('content-scroll-y').hover();await page.mouse.wheel(0,120);
    await expect(viewport).toHaveAttribute('data-bar-wheel-prevented','true');
    await expect.poll(async()=>(await scrollGeometry(viewport)).y).toBeGreaterThan(0);
});

test('scroll area mirrors real native two-axis scrolling, keyboard and resizing',async({page})=>{
    const root=page.getByTestId('content-scroll'),viewport=page.getByTestId('content-scroll-viewport');
    await expect(root).toHaveAttribute('id','content-scroll-root');await expect(viewport).toHaveAccessibleName('Release history');await expect(viewport).toHaveAttribute('tabindex','0');
    await expect(page.getByTestId('content-thumb-x')).toBeVisible();await expect(page.getByTestId('content-thumb-y')).toBeVisible();
    expect(await root.locator('[tabindex="0"]').count()).toBe(1);await expect(page.getByTestId('content-scroll-y')).not.toHaveAttribute('role','scrollbar');
    await viewport.hover();await page.mouse.wheel(0,120);await expect.poll(async()=>(await scrollGeometry(viewport)).y).toBeGreaterThan(0);
    await viewport.focus();await viewport.press('PageDown');await expect.poll(async()=>(await scrollGeometry(viewport)).y).toBeGreaterThan(0);
    await viewport.press('End');await expect.poll(async()=>{const box=await scrollGeometry(viewport);return box.fullHeight-box.height-box.y;}).toBeLessThan(2);
    await viewport.evaluate(element=>{element.scrollLeft=160;element.scrollTop=80;});
    await expect.poll(()=>page.getByTestId('content-thumb-x').evaluate(element=>element.getBoundingClientRect().x-document.querySelector('[data-testid="content-scroll-x"]')!.getBoundingClientRect().x)).toBeGreaterThan(0);
    const before=(await viewport.boundingBox())!.height;await page.getByTestId('content-scroll-resize').click();await expect.poll(async()=>(await viewport.boundingBox())!.height).toBeGreaterThan(before);
    await page.getByTestId('content-scroll-length').click();await expect(root).not.toHaveAttribute('data-x-overflow');await expect(root).not.toHaveAttribute('data-y-overflow');await expect(page.getByTestId('content-thumb-x')).toBeHidden();await expect(page.getByTestId('content-thumb-y')).toBeHidden();
});

test('scroll pointer drag preserves grab offset and author root id, then cancels capture',async({page})=>{
    const viewport=page.getByTestId('content-scroll-viewport'),track=page.getByTestId('content-scroll-y'),thumb=page.getByTestId('content-thumb-y');
    await page.getByTestId('content-scroll').scrollIntoViewIfNeeded();
    await viewport.evaluate(element=>element.scrollTop=0);await frames(page);
    const listenerBaseline=await page.evaluate(()=>(window as any).contentResources.listeners.size);
    const geometry=await scrollGeometry(viewport),trackBox=(await track.boundingBox())!,thumbBox=(await thumb.boundingBox())!;
    const grab=thumbBox.height*.75,delta=Math.min(36,(trackBox.height-thumbBox.height)*.4),expected=delta/(trackBox.height-thumbBox.height)*(geometry.fullHeight-geometry.height);
    await expect.poll(()=>thumb.evaluate((element,point)=>document.elementFromPoint(point.x,point.y)?.closest('[data-testid="content-thumb-y"]')===element,{x:thumbBox.x+thumbBox.width/2,y:thumbBox.y+grab})).toBe(true);
    await track.evaluate(element=>element.addEventListener('gotpointercapture',event=>(window as any).contentPointer=(event as PointerEvent).pointerId,{once:true}));
    await page.mouse.move(thumbBox.x+thumbBox.width/2,thumbBox.y+grab);await page.mouse.down();await page.mouse.move(thumbBox.x+thumbBox.width/2,thumbBox.y+grab+delta,{steps:4});
    await expect.poll(async()=>Math.abs((await scrollGeometry(viewport)).y-expected)).toBeLessThan(3);
    await expect.poll(()=>track.evaluate(element=>element.hasPointerCapture((window as any).contentPointer))).toBe(true);
    const beforeCancel=(await scrollGeometry(viewport)).y;
    await track.dispatchEvent('pointercancel',{pointerId:await page.evaluate(()=>(window as any).contentPointer),pointerType:'mouse',isPrimary:true});await page.mouse.move(thumbBox.x+thumbBox.width/2,thumbBox.y+grab+delta+30);await page.mouse.up();await frames(page);
    expect(Math.abs((await scrollGeometry(viewport)).y-beforeCancel)).toBeLessThan(2);await expect.poll(()=>track.evaluate(element=>element.hasPointerCapture((window as any).contentPointer))).toBe(false);
    expect(await page.evaluate(()=>(window as any).contentResources.listeners.size)).toBe(listenerBaseline);
});

test('RTL horizontal track and wheel use the native logical distance consistently',async({page})=>{
    const viewport=page.getByTestId('content-scroll-viewport'),track=page.getByTestId('content-scroll-x'),thumb=page.getByTestId('content-thumb-x');
    await page.getByTestId('content-scroll-direction').click();await expect(page.getByTestId('content-scroll')).toHaveAttribute('dir','rtl');
    await page.getByTestId('content-scroll').scrollIntoViewIfNeeded();
    await viewport.evaluate(element=>element.scrollLeft=0);await frames(page);
    const box=await track.boundingBox();expect(box).not.toBeNull();await page.mouse.click(box!.x+5,box!.y+box!.height/2);
    await expect.poll(async()=>(await scrollGeometry(viewport)).x).toBeLessThan(-100);
    await expect.poll(async()=>{const area=await scrollGeometry(viewport),t=(await track.boundingBox())!,h=(await thumb.boundingBox())!;const fraction=Math.abs(area.x)/(area.fullWidth-area.width);return Math.abs(h.x-t.x-(t.width-h.width)*(1-fraction));}).toBeLessThan(2);
    await viewport.evaluate(element=>element.scrollLeft=-80);await viewport.hover();await page.mouse.wheel(-100,0);await expect.poll(async()=>(await scrollGeometry(viewport)).x).toBeLessThan(-80);
    await page.getByTestId('content-scroll-boundary').click();await expect(viewport).toHaveCSS('overscroll-behavior','auto');
    await page.getByTestId('content-scroll-boundary').click();await expect(viewport).toHaveCSS('overscroll-behavior','contain');
    await setAxes(page,'vertical');await expect(track).toBeHidden();await setAxes(page,'horizontal');await expect(page.getByTestId('content-scroll-y')).toBeHidden();await setAxes(page,'both');await expect(track).toBeVisible();
});

test('visibility policies follow overflow, pointer or native scroll without hiding active focus',async({page})=>{
    const track=page.getByTestId('content-scroll-y'),viewport=page.getByTestId('content-scroll-viewport');
    await setVisibility(page,'hover');await page.getByTestId('content-outside').click();await expect(track).toHaveAttribute('data-state','hidden');
    await page.getByTestId('content-scroll').hover();await expect(track).toHaveAttribute('data-state','visible');
    await viewport.focus();await page.mouse.move(0,0);await expect(track).toHaveAttribute('data-state','visible');
    await page.getByTestId('content-outside').focus();await expect(track).toHaveAttribute('data-state','hidden');
    await setVisibility(page,'scroll');await page.getByTestId('content-outside').click();await expect(track).toHaveAttribute('data-state','hidden');
    await viewport.focus();await viewport.evaluate(element=>element.scrollTop+=60);await expect(track).toHaveAttribute('data-state','visible');await frames(page);
    expect(await page.evaluate(()=>(window as any).contentResources.timers.size)).toBe(0);
    await page.getByTestId('content-outside').focus();await expect(track).toHaveAttribute('data-state','hidden');
    await setVisibility(page,'auto');await expect(track).toHaveAttribute('data-state','visible');await page.getByTestId('content-scroll-length').click();await expect(track).toHaveAttribute('data-state','hidden');
    await setVisibility(page,'always');await expect(track).toHaveAttribute('data-state','visible');await expect(track).toHaveAttribute('data-disabled','');
});

test('removing an active scroll area cancels its frames, timer, observers and drag listeners',async({page})=>{
    const viewport=page.getByTestId('content-scroll-viewport'),track=page.getByTestId('content-scroll-y'),thumb=page.getByTestId('content-thumb-y');
    await expect(page.getByTestId('content-avatar')).toHaveAttribute('data-state','loaded');
    await page.getByTestId('content-scroll').scrollIntoViewIfNeeded();await frames(page);
    const listenerBaseline=await page.evaluate(()=>(window as any).contentResources.listeners.size);
    await page.evaluate(()=>{const state=(window as any).contentResources;(window as any).contentAreaObservers=[...state.activeObservers].filter(([,targets]:any)=>[...targets].some((target:any)=>!!target.closest('#content-scroll-root'))).map(([observer]:any)=>observer);});
    expect(await page.evaluate(()=>(window as any).contentAreaObservers.length)).toBeGreaterThan(0);
    await track.evaluate(element=>element.addEventListener('pointerdown',event=>(window as any).contentPointer=(event as PointerEvent).pointerId,{once:true}));
    const thumbBox=(await thumb.boundingBox())!;await page.mouse.move(thumbBox.x+thumbBox.width/2,thumbBox.y+thumbBox.height/2);await page.mouse.down();
    await expect.poll(()=>page.evaluate(()=>(window as any).contentResources.listeners.size)).toBeGreaterThan(listenerBaseline);
    const held=await track.elementHandle();
    const pending=await viewport.evaluate(element=>{element.dispatchEvent(new Event('scroll'));const state=(window as any).contentResources;const ids=[...state.pendingFrames] as number[];document.querySelector<HTMLButtonElement>('[data-testid="content-scroll-mount"]')!.click();return ids;});
    await expect(page.getByTestId('content-scroll')).toHaveCount(0);await page.mouse.up();await frames(page);
    expect(pending.length).toBeGreaterThan(0);
    const result=await page.evaluate(ids=>{const state=(window as any).contentResources;return {cancelled:ids.some(id=>state.frameRecords.get(id)?.cancelled) && ids.filter(id=>state.frameRecords.get(id)?.cancelled).every(id=>!state.frameRecords.get(id)?.called),observers:(window as any).contentAreaObservers.filter((observer:any)=>state.activeObservers.has(observer)).length,listeners:state.listeners.size,timers:state.timers.size};},pending);
    expect(result).toEqual({cancelled:true,observers:0,listeners:listenerBaseline,timers:0});
    expect(await held!.evaluate(element=>element.hasPointerCapture((window as any).contentPointer))).toBe(false);
    await page.getByTestId('content-scroll-mount').click();await expect(page.getByTestId('content-scroll')).toHaveAttribute('data-y-overflow','');
    await setVisibility(page,'scroll');await page.getByTestId('content-outside').click();
    const timerCount=await viewport.evaluate(element=>{element.scrollTop=40;element.dispatchEvent(new Event('scroll'));const count=(window as any).contentResources.timers.size;document.querySelector<HTMLButtonElement>('[data-testid="content-scroll-mount"]')!.click();return count;});
    expect(timerCount).toBeGreaterThan(0);await expect(page.getByTestId('content-scroll')).toHaveCount(0);await frames(page);expect(await page.evaluate(()=>(window as any).contentResources.timers.size)).toBe(0);
});

test('themed native table and definition list preserve long, empty and multiple-value semantics',async({page},info)=>{
    test.skip(info.project.name==='dom-primitives','Themed static content has no duplicate raw API.');
    const table=page.getByTestId('content-table'),list=page.getByTestId('content-data-list');
    await expect(table).toHaveRole('table');await expect(table).toHaveAccessibleName('Release review checklist');await expect(table).not.toHaveAttribute('role','grid');
    await expect(table.locator('thead th')).toHaveCount(3);await expect(table.locator('thead th').first()).toHaveAttribute('scope','col');await expect(table.locator('tbody th')).toHaveAttribute('scope','row');
    await expect(page.getByTestId('content-table-status')).toHaveAttribute('headers','content-status-column content-review-row');
    expect(await list.evaluate(element=>element.tagName)).toBe('DL');expect(await page.getByTestId('content-owner-item').evaluate(element=>[...element.children].map(child=>child.tagName))).toEqual(['DT','DD','DD']);
    await expect(page.getByTestId('content-long-value')).toContainText('without-shortening-the-original-value');
    await page.getByTestId('content-empty').click();await expect(page.getByTestId('content-table-empty')).toHaveAttribute('colspan','3');await expect(list).toContainText('No release details yet.');
    await page.getByTestId('content-empty').click();await expect(page.getByTestId('content-table-status')).toHaveText('In review');
});

test('typed Badge and Card retain real links while Callout only announces an explicit role',async({page},info)=>{
    test.skip(info.project.name==='dom-primitives','Themes-only entries.');
    const badge=page.getByTestId('content-badge'),card=page.getByTestId('content-card'),callout=page.getByTestId('content-callout');
    await expect(badge).toHaveRole('link');await expect(card).toHaveRole('link');await expect(badge).toHaveAttribute('href','#content-native-result');await expect(card).toHaveAttribute('href','#content-native-result');await badge.click();await expect(page.getByTestId('content-native-result')).toHaveText('Status link opened.');
    await card.click();await expect(page.getByTestId('content-native-result')).toHaveText('Release details link opened.');await expect(callout).not.toHaveAttribute('role');
    await expect(page.getByTestId('content-callout-icon')).toHaveAttribute('aria-hidden','true');await page.getByTestId('content-callout-role').click();await expect(callout).toHaveRole('status');await page.getByTestId('content-callout-role').click();await expect(callout).toHaveRole('alert');
    await page.getByTestId('content-callout-role').click();await expect(callout).not.toHaveAttribute('role');
});

test('selection cards retain native FormData, descriptions and disabled successful-control rules',async({page},info)=>{
    test.skip(info.project.name==='dom-primitives','Existing themed choice-card composition.');
    const form=page.getByTestId('content-choice-form');
    const data=()=>form.evaluate(element=>{const data=new FormData(element as HTMLFormElement);return {channels:data.getAll('channels'),delivery:data.get('delivery')};});
    expect(await data()).toEqual({channels:['email'],delivery:'review'});
    const feed=form.getByRole('checkbox',{name:'Team feed',exact:true});await expect(feed).toHaveAccessibleDescription('Keep release notes in the team feed.');await feed.check();await form.getByRole('radio',{name:'Ready to share',exact:true}).check();
    expect(await data()).toEqual({channels:['email','feed'],delivery:'ready'});await expect(form.getByRole('checkbox',{name:'Offline export',exact:true})).toBeDisabled();
    await page.getByTestId('content-choice-save').click();await expect(page.getByTestId('content-choice-saved')).toHaveText('Sharing preferences saved locally.');
});

test('themed parameters cover every declared size and variant while local utility remains effective',async({page},info)=>{
    test.skip(info.project.name==='dom-primitives','Themed parameter matrix.');
    const avatar=page.getByTestId('content-avatar'),badge=page.getByTestId('content-badge');
    const widths=new Set<number>();for(let i=0;i<9;i++){widths.add(Math.round((await avatar.boundingBox())!.width));await page.getByTestId('content-size').click();}
    expect([...widths].sort((a,b)=>a-b)).toEqual([24,32,40,48,64,80,96,128,160]);
    const seen={avatar:new Set<string>(),badge:new Set<string>(),card:new Set<string>(),callout:new Set<string>(),table:new Set<string>()};
    for(let i=0;i<4;i++){for(const key of Object.keys(seen) as (keyof typeof seen)[]){seen[key].add((await page.getByTestId(`content-${key}`).getAttribute('data-variant'))!);}await page.getByTestId('content-variant').click();}
    expect([...seen.avatar].sort()).toEqual(['soft','solid']);expect([...seen.badge].sort()).toEqual(['outline','soft','solid','surface']);expect([...seen.card].sort()).toEqual(['classic','ghost','surface']);expect([...seen.callout].sort()).toEqual(['outline','soft','surface']);expect([...seen.table].sort()).toEqual(['ghost','surface']);
    await page.getByTestId('content-local-color').click();await expect(badge).toHaveAttribute('data-color','amber');await page.getByTestId('content-high-contrast').click();await expect(badge).toHaveAttribute('data-high-contrast','true');
    const radii=new Set<string>();for(let i=0;i<6;i++){await page.getByTestId('content-radius').click();radii.add(await avatar.evaluate(element=>getComputedStyle(element).borderTopLeftRadius));}
    expect([...radii].sort()).toEqual(['0px','4.5px','6px','9999px','9px']);
    await page.getByTestId('content-utility').click();await expect(badge).toHaveClass(/rustify-badge/);await expect(badge).toHaveCSS('padding-top','32px');await page.getByTestId('content-size').click();await expect(badge).toHaveClass(/rustify-badge/);await expect(badge).toHaveCSS('padding-top','32px');
    for(const [name,token,display] of [['content-owner-badge','rustify-badge','flex'],['content-owner-card','rustify-card','block']]){
        const component=page.getByTestId(name);await expect(component).toHaveClass(new RegExp(token));await expect(component).toHaveClass(new RegExp(name));await expect(component).toHaveCSS('display',display);
        expect(await component.evaluate(element=>parseFloat(getComputedStyle(element).paddingTop))).toBeGreaterThan(0);
    }
    await page.getByTestId('content-utility').click();await expect(badge).not.toHaveClass(/(?:^|\s)p-8(?:\s|$)/);await expect(badge).toHaveClass(/rustify-badge/);
    await expect(badge).not.toHaveCSS('padding-top','32px');
});

test('content remains readable in both appearances, narrow layouts and reduced motion',async({page},info)=>{
    for(const mode of ['light','dark']){
        if(mode==='dark')await page.getByTestId('content-theme').click();
        if(info.project.name==='dom-only')await expect(page.getByTestId('content-theme-boundary')).toHaveAttribute('data-theme',mode);else await expect(page.getByTestId('content-samples')).toHaveAttribute('data-mode',mode);
        for(const width of [390,768,1280]){
            await page.setViewportSize({width,height:900});await expect.poll(()=>page.getByTestId('content-samples').evaluate(element=>element.scrollWidth-element.clientWidth)).toBeLessThanOrEqual(1);
            await page.getByTestId('content-samples').screenshot({path:`docs/validation/dom-components-p3/m5-content-${info.project.name}-${mode}-${width}.png`});
        }
    }
    await page.emulateMedia({reducedMotion:'reduce'});await expect(page.getByTestId('content-scroll-y')).toHaveCSS('transition-duration','0s');
    if(info.project.name==='dom-only'){
        await page.getByTestId('stage-details-tab').click();await expect(page.getByTestId('stage-owner-avatar')).toHaveAccessibleName('Jamie Morgan');await expect(page.getByTestId('stage-details-table')).toHaveAccessibleName('Release task details');await expect(page.getByTestId('stage-data-list')).toContainText('Release operations');
    }
});
