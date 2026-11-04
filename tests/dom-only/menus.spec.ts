import { test, expect, Page } from '@playwright/test';
const failures = new WeakMap<Page, string[]>();
const part = (page: Page, name: string) => page.getByTestId(name);
async function keyOpen(page: Page) {
    await part(page,'menu-trigger').focus();await page.keyboard.press('ArrowDown');
    await expect(part(page,'menu-alpha')).toBeFocused();
}
async function policy(page: Page) {
    // Application policy can change while its scope is inert under a modal menu.
    await part(page,'menu-accept').evaluate((node:HTMLButtonElement)=>node.click());
}
async function outside(page: Page) { await page.mouse.click(3,3); }
async function value(page:Page,name:string) {return Number(await part(page,name).textContent());}
async function twoFrames(page:Page) {await page.evaluate(()=>new Promise<void>(resolve=>requestAnimationFrame(()=>requestAnimationFrame(()=>resolve()))));}
async function closedResources(page:Page) {
    await twoFrames(page);
    return page.evaluate(()=>({...(window as any).__menus.stats(),listeners:(window as any).__menuProbe.listeners.size,
        nativeTimers:(window as any).__menuProbe.timers.size,mutations:(window as any).__menuProbe.mutations.size,
        nodes:document.querySelectorAll('*').length}));
}
test.beforeEach(async({page},info)=>{
    const errors:string[]=[];failures.set(page,errors);
    page.on('pageerror',error=>errors.push(error.message));
    page.on('console',message=>{if(message.type()==='error')errors.push(message.text());});
    page.on('requestfailed',request=>errors.push(`${request.url()}: ${request.failure()?.errorText}`));
    await page.addInitScript(()=>{
        const add=EventTarget.prototype.addEventListener,remove=EventTarget.prototype.removeEventListener;
        const listeners=new Set<{target:EventTarget,type:string,listener:any,capture:boolean}>();
        EventTarget.prototype.addEventListener=function(type:string,listener:any,options:any){
            const capture=typeof options==='boolean'?options:!!options?.capture;
            if((this===window||this===document)&&listener&&!options?.signal?.aborted&&!options?.once&&!([...listeners].some(row=>row.target===this&&row.type===type&&row.listener===listener&&row.capture===capture))){
                const row={target:this,type,listener,capture};listeners.add(row);
                if(options?.signal)add.call(options.signal,'abort',()=>listeners.delete(row),{once:true});
            }
            return add.call(this,type,listener,options);
        };
        EventTarget.prototype.removeEventListener=function(type:string,listener:any,options:any){
            const capture=typeof options==='boolean'?options:!!options?.capture;
            for(const row of listeners)if(row.target===this&&row.type===type&&row.listener===listener&&row.capture===capture)listeners.delete(row);
            return remove.call(this,type,listener,options);
        };
        const schedule=window.setTimeout.bind(window),clear=window.clearTimeout.bind(window),timers=new Set<number>(),scheduled:number[]=[];
        window.setTimeout=((callback:TimerHandler,delay?:number,...args:unknown[])=>{
            if(typeof callback!=='function'||![100,150,200,300,700,1000].includes(delay??0))return schedule(callback,delay,...args);
            scheduled.push(delay!);const id=schedule((...values:unknown[])=>{timers.delete(id);return callback.apply(window,values);},delay,...args);timers.add(id);return id;
        }) as typeof window.setTimeout;
        window.clearTimeout=(id?:number)=>{if(id!==undefined)timers.delete(id);clear(id);};
        const Native=window.MutationObserver,mutations=new Set<MutationObserver>();
        window.MutationObserver=class extends Native {
            observe(target:Node,options?:MutationObserverInit){super.observe(target,options);mutations.add(this);}
            disconnect(){mutations.delete(this);super.disconnect();}
        };
        (window as any).__menuProbe={listeners,timers,mutations,scheduled};
    });
    await page.goto(info.project.name==='dom-primitives'?'/':'/menus.html?parts');
    await expect(page.locator('#status')).toHaveAttribute('data-status','ready');
    await part(page,'menu-parts').locator('details').first().locator('summary').click();
});
test.afterEach(async({page},info)=>{
    const errors=failures.get(page)??[];
    if(info.title.includes('trap'))expect(errors.filter(error=>!/deliberate menu instance trap|unreachable|panicked at/.test(error))).toEqual([]);
    else expect(errors).toEqual([]);
});

test.describe('menus: controlled command trees',()=>{
    test('keyboard navigation, typeahead, disabled items and Tab remain inside the menu',async({page})=>{
        await keyOpen(page);
        await page.keyboard.press('ArrowDown');await expect(part(page,'menu-archive')).toBeFocused();
        await page.keyboard.press('Home');await expect(part(page,'menu-alpha')).toBeFocused();
        await page.keyboard.press('a');await expect(part(page,'menu-archive')).toBeFocused();
        await page.keyboard.press('a');await expect(part(page,'menu-alpha')).toBeFocused();
        await page.keyboard.press('Tab');await expect(part(page,'menu-alpha')).toBeFocused();
        await page.keyboard.press('Shift+Tab');await expect(part(page,'menu-alpha')).toBeFocused();
        await page.keyboard.press('End');await expect(part(page,'menu-sub-trigger')).toBeFocused();
        await page.keyboard.press('Home');await page.keyboard.press('Enter');
        await expect(part(page,'menu-content')).toHaveCount(0);await expect(part(page,'menu-last')).toHaveText('alpha');
        await expect(part(page,'menu-selections')).toHaveText('1');await expect(part(page,'menu-trigger')).toBeFocused();
    });
    test('pointer opening focuses the panel and selection cancellation preserves controlled changes',async({page})=>{
        await part(page,'menu-cancel').click();await part(page,'menu-trigger').click();
        await expect(part(page,'menu-content')).toBeFocused();await expect(part(page,'menu-content').locator('[data-highlighted]')).toHaveCount(0);
        await expect(part(page,'menu-checked')).toHaveAttribute('aria-checked','mixed');
        await part(page,'menu-checked').click();await expect(part(page,'menu-checked')).toHaveAttribute('aria-checked','true');
        await expect(part(page,'menu-changes')).toHaveText('1');await expect(part(page,'menu-content')).toBeVisible();
        await policy(page);await part(page,'menu-checked').click();await expect(part(page,'menu-checked')).toHaveAttribute('aria-checked','true');
        await expect(part(page,'menu-changes')).toHaveText('2');
        await part(page,'menu-radio-name').click();await expect(part(page,'menu-changes')).toHaveText('2');
        await part(page,'menu-radio-date').click();await expect(part(page,'menu-radio-date')).toHaveAttribute('aria-checked','false');
        await expect(part(page,'menu-changes')).toHaveText('3');
        await page.keyboard.press('Escape');await expect(part(page,'menu-content')).toBeVisible();
        await policy(page);await page.keyboard.press('Escape');await expect(part(page,'menu-content')).toHaveCount(0);
    });
    test('refused opening emits one request and leaves the accepted closed state',async({page})=>{
        await policy(page);await part(page,'menu-trigger').focus();await page.keyboard.press('ArrowDown');
        await expect(part(page,'menu-requests')).toHaveText('1');await expect(part(page,'menu-trigger')).toHaveAttribute('aria-expanded','false');
        await expect(part(page,'menu-content')).toHaveCount(0);
    });
    test('two submenu levels enter, return and escape one layer at a time',async({page})=>{
        await keyOpen(page);await page.keyboard.press('End');await page.keyboard.press('ArrowRight');
        await expect(part(page,'menu-sub-copy')).toBeFocused();
        await page.keyboard.type('copy ');await expect(part(page,'menu-sub-content')).toBeVisible();await expect(part(page,'menu-selections')).toHaveText('0');
        await page.keyboard.press('End');await page.keyboard.press('ArrowRight');await expect(part(page,'menu-second-action')).toBeFocused();
        await page.keyboard.press('ArrowLeft');await expect(part(page,'menu-second-content')).toHaveCount(0);await expect(part(page,'menu-second-trigger')).toBeFocused();
        await page.keyboard.press('Escape');await expect(part(page,'menu-sub-content')).toHaveCount(0);await expect(part(page,'menu-sub-trigger')).toBeFocused();
        await page.keyboard.press('Escape');await expect(part(page,'menu-content')).toHaveCount(0);await expect(part(page,'menu-trigger')).toBeFocused();
    });
    test('a submenu command delivers once and closes its entire tree',async({page})=>{
        await keyOpen(page);await page.keyboard.press('End');await page.keyboard.press('ArrowRight');
        await expect(part(page,'menu-sub-copy')).toBeFocused();const requests=await value(page,'menu-requests');await page.keyboard.press('Enter');
        await expect(part(page,'menu-last')).toHaveText('copy-link');await expect(part(page,'menu-selections')).toHaveText('1');
        await expect(part(page,'menu-content')).toHaveCount(0);await expect(part(page,'menu-sub-content')).toHaveCount(0);
        await expect(part(page,'menu-requests')).toHaveText(String(requests+2));
    });
    test('outside pointer closes the tree unless the root outside hook cancels',async({page})=>{
        await part(page,'menu-cancel-outside').click();await keyOpen(page);await page.keyboard.press('End');await page.keyboard.press('ArrowRight');
        await outside(page);await expect(part(page,'menu-content')).toBeVisible();
        await part(page,'menu-content').focus();await page.keyboard.press('Escape');
        await expect(part(page,'menu-content')).toHaveCount(0);await expect(part(page,'menu-trigger')).toBeFocused();
        await part(page,'menu-cancel-outside').click();await keyOpen(page);await page.keyboard.press('End');await page.keyboard.press('ArrowRight');
        await outside(page);await expect(part(page,'menu-content')).toHaveCount(0);await expect(part(page,'menu-sub-content')).toHaveCount(0);
    });
    test('RTL reverses submenu entry and backward return',async({page})=>{
        await part(page,'menu-direction').click();await keyOpen(page);await page.keyboard.press('End');await page.keyboard.press('ArrowLeft');
        await expect(part(page,'menu-sub-copy')).toBeFocused();await page.keyboard.press('ArrowRight');
        await expect(part(page,'menu-sub-content')).toHaveCount(0);await expect(part(page,'menu-sub-trigger')).toBeFocused();
    });
    test('pointer intent and the diagonal grace corridor keep the submenu reachable',async({page})=>{
        await part(page,'menu-trigger').click();const trigger=part(page,'menu-sub-trigger');await trigger.hover();
        await expect(part(page,'menu-sub-content')).toBeVisible();
        const from=await trigger.boundingBox(),to=await part(page,'menu-sub-copy').boundingBox();expect(from&&to).toBeTruthy();
        await page.mouse.move(from!.x+from!.width-2,from!.y+from!.height/2);
        await page.mouse.move(to!.x+to!.width/2,to!.y+to!.height/2,{steps:6});
        await expect(part(page,'menu-sub-copy')).toBeFocused();await expect(part(page,'menu-sub-content')).toBeVisible();
    });
});

test.describe('context menus',()=>{
    test('right click opens once and disabled triggers preserve the browser default',async({page})=>{
        await part(page,'context-trigger').click({button:'right'});await expect(part(page,'context-content')).toBeFocused();
        await expect(part(page,'menu-requests')).toHaveText('1');await page.keyboard.press('Escape');
        await part(page,'menu-disable-context').click();
        expect(await part(page,'context-trigger').evaluate(node=>node.dispatchEvent(new MouseEvent('contextmenu',{bubbles:true,cancelable:true,button:2})))).toBe(true);
        await expect(part(page,'context-content')).toHaveCount(0);await expect(part(page,'menu-requests')).toHaveText('2');
    });
    for(const key of ['Shift+F10','ContextMenu'])test(`keyboard ${key} opens once through its matching keyup`,async({page})=>{
        const trigger=part(page,'context-trigger');await trigger.focus();
        if(key==='Shift+F10')await page.keyboard.down('Shift');await page.keyboard.down(key==='Shift+F10'?'F10':'ContextMenu');
        await expect(part(page,'context-copy')).toBeFocused();
        await trigger.dispatchEvent('contextmenu',{button:2,clientX:10,clientY:10});
        await page.keyboard.up(key==='Shift+F10'?'F10':'ContextMenu');if(key==='Shift+F10')await page.keyboard.up('Shift');
        await expect(part(page,'menu-requests')).toHaveText('1');await page.keyboard.press('Escape');await expect(trigger).toBeFocused();
    });
    test('simulated touch long press opens and movement cancels a pending press',async({page})=>{
        const trigger=part(page,'context-trigger');const box=await trigger.boundingBox();expect(box).toBeTruthy();
        const event={pointerId:17,pointerType:'touch',button:0,clientX:box!.x+8,clientY:box!.y+8};
        await trigger.dispatchEvent('pointerdown',event);await expect(part(page,'context-content')).toBeVisible();
        await trigger.dispatchEvent('pointerup',event);await page.keyboard.press('Escape');
        await trigger.dispatchEvent('pointerdown',event);await trigger.dispatchEvent('pointermove',{...event,clientX:event.clientX+20});
        await page.waitForTimeout(750);await expect(part(page,'context-content')).toHaveCount(0);await expect(part(page,'menu-requests')).toHaveText('2');
    });
});

test.describe('menubar and toolbar',()=>{
    test('menubar moves between enabled triggers and switches directly without a close request',async({page})=>{
        await part(page,'menubar-file').focus();await page.keyboard.press('ArrowRight');await expect(part(page,'menubar-edit')).toBeFocused();
        await page.keyboard.press('Home');await page.keyboard.press('ArrowDown');await expect(part(page,'menubar-new')).toBeFocused();
        await page.keyboard.press('ArrowRight');await expect(part(page,'menubar-undo')).toBeFocused();await expect(part(page,'menubar-log')).toHaveText('file,edit');
        await page.keyboard.press('Escape');await expect(part(page,'menubar-edit')).toBeFocused();await expect(part(page,'menubar-log')).toHaveText('file,edit,closed');
        await part(page,'menubar-file').click();await part(page,'menubar-view').hover();await expect(part(page,'menubar-view-content')).toBeVisible();
        await expect(part(page,'menubar-log')).toHaveText('file,edit,closed,file,view');
        await part(page,'menubar-edit').dispatchEvent('pointerdown',{button:0,pointerType:'touch'});
        await expect(part(page,'menubar-edit-content')).toBeVisible();await expect(part(page,'menubar-log')).toHaveText('file,edit,closed,file,view,edit');
    });
    test('menubar refuses a switch and reverses root and content navigation in RTL',async({page})=>{
        await part(page,'menubar-file').click();await policy(page);await part(page,'menubar-view').hover();
        await expect(part(page,'menubar-file-content')).toBeVisible();await expect(part(page,'menubar-log')).toHaveText('file,view');
        await policy(page);await outside(page);await part(page,'menu-direction').click();
        await part(page,'menubar-file').focus();await page.keyboard.press('ArrowLeft');await expect(part(page,'menubar-edit')).toBeFocused();
        await page.keyboard.press('ArrowDown');await expect(part(page,'menubar-undo')).toBeFocused();await page.keyboard.press('ArrowLeft');await expect(part(page,'menubar-details')).toBeFocused();
    });
    test('menubar submenu backward closes locally and forward switches the top-level menu',async({page})=>{
        await part(page,'menubar-file').focus();await page.keyboard.press('ArrowDown');await expect(part(page,'menubar-new')).toBeFocused();await page.keyboard.press('End');await page.keyboard.press('ArrowRight');
        await expect(part(page,'menubar-sub-copy')).toBeFocused();await page.keyboard.press('ArrowLeft');await expect(part(page,'menubar-sub-trigger')).toBeFocused();
        await page.keyboard.press('ArrowRight');await expect(part(page,'menubar-sub-copy')).toBeFocused();await page.keyboard.press('ArrowRight');await expect(part(page,'menubar-undo')).toBeFocused();
        await expect(part(page,'menubar-log')).toHaveText('file,edit');
    });
    test('toolbar owns one stop across groups, preserves controlled selection and embedded menu focus',async({page})=>{
        const toolbar=part(page,'toolbar');await expect(toolbar.locator('[tabindex="0"]')).toHaveCount(1);
        await part(page,'toolbar-save').focus();await page.keyboard.press('ArrowRight');await expect(part(page,'toolbar-bold')).toBeFocused();
        await expect(part(page,'toolbar-italic')).toHaveAttribute('aria-pressed','true');await page.keyboard.press('Space');
        await expect(part(page,'toolbar-log')).toHaveText('bold,italic');await expect(part(page,'toolbar-value')).toHaveText('bold,italic');
        await policy(page);await part(page,'toolbar-bold').focus();await page.keyboard.press('Space');await expect(part(page,'toolbar-value')).toHaveText('bold,italic');
        await expect(part(page,'toolbar-log')).toHaveText('bold,italic;italic');await policy(page);
        await page.keyboard.press('ArrowRight');await page.keyboard.press('ArrowRight');await expect(part(page,'toolbar-menu')).toBeFocused();
        await page.keyboard.press('ArrowDown');await expect(part(page,'toolbar-menu-content')).toBeVisible();await page.keyboard.press('Escape');
        await expect(part(page,'toolbar-menu')).toBeFocused();await page.keyboard.press('End');await expect(part(page,'toolbar-link')).toBeFocused();
        await part(page,'toolbar-link').evaluate(node=>node.addEventListener('click',()=>node.setAttribute('data-clicked','true'),{once:true}));
        await page.keyboard.press('Space');await expect(part(page,'toolbar-link')).toHaveAttribute('data-clicked','true');
    });
    test('toolbar leaves search editing native and handles vertical and RTL movement',async({page})=>{
        const search=part(page,'toolbar-search');await search.fill('document');
        for(const key of ['Home','End']){
            await search.evaluate(node=>{window.addEventListener('keydown',event=>node.setAttribute('data-key-canceled',String(event.defaultPrevented)),{once:true});});
            await search.press(key);await expect(search).toBeFocused();await expect(search).toHaveAttribute('data-key-canceled','false');await expect(search).toHaveValue('document');
        }
        await search.evaluate((node:HTMLInputElement)=>node.setSelectionRange(node.value.length,node.value.length));await search.press('ArrowLeft');
        expect(await search.evaluate((node:HTMLInputElement)=>node.selectionStart)).toBe(7);await expect(search).toBeFocused();
        await part(page,'toolbar-back').focus();await page.keyboard.press('ArrowDown');await expect(part(page,'toolbar-forward')).toBeFocused();
        await expect(part(page,'toolbar-vertical-separator')).toHaveAttribute('aria-orientation','horizontal');
        await part(page,'menu-direction').click();await part(page,'toolbar-save').focus();await page.keyboard.press('ArrowLeft');await expect(part(page,'toolbar-bold')).toBeFocused();
    });
});

test.describe('navigation menu: native destinations',()=>{
    test('hover delay and skip window reveal inline content with measured geometry',async({page})=>{
        const before=await page.evaluate(()=>(window as any).__menuProbe.scheduled.filter((delay:number)=>delay===200).length);
        await part(page,'navigation-learn').hover();await expect(part(page,'navigation-learn-content')).toBeVisible();
        expect(await page.evaluate(()=>(window as any).__menuProbe.scheduled.filter((delay:number)=>delay===200).length)).toBe(before+1);
        await expect(part(page,'navigation-guide')).toHaveAttribute('aria-current','page');
        await expect.poll(()=>part(page,'navigation-viewport').evaluate(node=>getComputedStyle(node).getPropertyValue('--rustify-navigation-menu-viewport-width'))).not.toBe('');
        const geometry=await part(page,'navigation-learn-content').evaluate(node=>{
            const root=node.closest('[data-rustify-navigation]')!,indicator=root.querySelector('[data-testid="navigation-indicator"]')!,viewport=root.querySelector('[data-testid="navigation-viewport"]')!;
            return {inline:node.parentElement?.tagName,contentWidth:(node as HTMLElement).offsetWidth,viewportWidth:viewport.getBoundingClientRect().width,
                size:parseFloat(getComputedStyle(indicator).getPropertyValue('--rustify-navigation-menu-indicator-size')),listChildren:[...root.querySelector('ul')!.children].map(node=>node.tagName)};
        });
        expect(geometry.inline).toBe('LI');expect(geometry.listChildren.every(tag=>tag==='LI')).toBe(true);expect(geometry.size).toBeGreaterThan(0);
        expect(Math.abs(geometry.contentWidth-geometry.viewportWidth)).toBeLessThanOrEqual(2);
        await part(page,'navigation-learn').focus();await page.keyboard.press('ArrowDown');await page.keyboard.press('Escape');
        await expect(part(page,'navigation-learn-content')).toBeHidden();await part(page,'navigation-resources').hover();
        await expect(part(page,'navigation-resources-content')).toBeVisible();
        expect(await page.evaluate(()=>(window as any).__menuProbe.scheduled.filter((delay:number)=>delay===200).length)).toBe(before+1);
    });
    test('native Tab order, directional entry and sub Escape preserve navigation boundaries',async({page})=>{
        await part(page,'navigation-learn').focus();await page.keyboard.press('ArrowRight');await expect(part(page,'navigation-resources')).toBeFocused();
        await page.keyboard.press('End');await expect(part(page,'navigation-help')).toBeFocused();
        await page.keyboard.press('Home');await page.keyboard.press('Enter');await page.keyboard.press('ArrowDown');await expect(part(page,'navigation-guide')).toBeFocused();
        await page.keyboard.press('Tab');await expect(part(page,'navigation-new-tab')).toBeFocused();await page.keyboard.press('Tab');await expect(part(page,'navigation-examples')).toBeFocused();
        await page.keyboard.press('Enter');await page.keyboard.press('ArrowDown');await expect(part(page,'navigation-example-link')).toBeFocused();
        await page.keyboard.press('Escape');await expect(part(page,'navigation-examples-content')).toHaveCount(0);await expect(part(page,'navigation-examples')).toBeFocused();
        await expect(part(page,'navigation-learn-content')).toBeVisible();await page.keyboard.press('Escape');await expect(part(page,'navigation-learn-content')).toBeHidden();
        await expect(part(page,'navigation-learn')).toBeFocused();
        await part(page,'navigation-guide').evaluate((node:HTMLElement)=>node.focus());await expect(part(page,'navigation-learn')).toBeFocused();
    });
    test('native links navigate and modifier clicks leave the accepted navigation open',async({page})=>{
        await part(page,'navigation-learn').click();
        const popup=page.context().waitForEvent('page');await part(page,'navigation-new-tab').click({modifiers:[process.platform==='darwin'?'Meta':'Control']});
        const child=await popup;await expect(child).toHaveURL(/#rui-/);await child.close();
        await expect(part(page,'navigation-value')).toHaveText('learn');await expect(part(page,'navigation-log')).toHaveText('learn');
        const href=await part(page,'navigation-guide').getAttribute('href');await part(page,'navigation-guide').click();
        expect(new URL(page.url()).hash).toBe(href);await expect(part(page,'navigation-log')).toHaveText('learn,closed');
        await expect(part(page,'navigation-selections')).toHaveText('1');
    });
    test('outside pointer and focus requests deduplicate and another scope layer is exempt',async({page})=>{
        await part(page,'navigation-learn').click();await part(page,'navigation-layer-trigger').click();
        await expect(part(page,'navigation-layer')).toBeVisible();await expect(part(page,'navigation-value')).toHaveText('learn');
        await page.keyboard.press('Escape');await expect(part(page,'navigation-layer')).toHaveCount(0);await expect(part(page,'navigation-value')).toHaveText('learn');
        await policy(page);const before=(await part(page,'navigation-log').textContent())!;
        await part(page,'menu-outside').click();await expect(part(page,'navigation-log')).toHaveText(`${before},closed`);await expect(part(page,'navigation-value')).toHaveText('learn');
        await policy(page);await part(page,'navigation-learn').focus();await page.keyboard.press('Enter');await expect(part(page,'navigation-value')).toBeEmpty();
    });
    test('RTL follows the declared horizontal direction and refused opening keeps content inert',async({page})=>{
        await part(page,'menu-direction').click();await part(page,'navigation-learn').focus();await page.keyboard.press('ArrowLeft');await expect(part(page,'navigation-resources')).toBeFocused();
        await policy(page);await part(page,'navigation-learn').focus();await page.keyboard.press('Enter');
        await expect(part(page,'navigation-log')).toHaveText('learn');await expect(part(page,'navigation-learn-content')).toBeHidden();
        await expect(part(page,'navigation-learn-content')).toHaveAttribute('inert','');
    });
});

test('sample states keep empty commands, loading destinations and field errors understandable',async({page})=>{
    const search=part(page,'toolbar-search');await search.fill('x');await expect(search).toHaveAttribute('aria-invalid','true');await expect(search).toHaveAccessibleDescription('Use at least two characters.');
    await search.fill('document');await expect(search).toHaveAttribute('aria-invalid','false');await expect(part(page,'toolbar-search-error')).toHaveCount(0);
    await search.press('Alt+Shift+N');await expect(part(page,'menu-last')).toHaveText('new');
    await part(page,'menu-empty-demo').click();await part(page,'toolbar-menu').focus();await page.keyboard.press('ArrowDown');
    await expect(part(page,'toolbar-empty')).toBeVisible();await expect(part(page,'toolbar-empty')).toHaveAttribute('aria-disabled','true');await expect(part(page,'toolbar-menu-content')).toBeFocused();
    await page.keyboard.press('Escape');await expect(part(page,'toolbar-menu')).toBeFocused();
    await part(page,'navigation-loading-demo').click();await part(page,'navigation-resources').click();await expect(part(page,'navigation-loading')).toHaveText('Refreshing reference links…');await expect(part(page,'navigation-reference')).toBeVisible();
    await part(page,'navigation-resources').focus();await page.keyboard.press('ArrowDown');await expect(part(page,'navigation-reference')).toBeFocused();
});

test.describe('lifecycle and a11y',()=>{
    test('removing active triggers and the current toolbar item releases or transfers focus',async({page})=>{
        await keyOpen(page);await page.keyboard.press('End');await page.keyboard.press('ArrowRight');
        await part(page,'menu-sub-trigger').evaluate(node=>node.remove());await expect(part(page,'menu-sub-content')).toHaveCount(0);await expect(part(page,'menu-content')).toBeVisible();
        await part(page,'menu-trigger').evaluate(node=>node.remove());await expect(part(page,'menu-content')).toHaveCount(0);
        await part(page,'navigation-learn').click();await part(page,'navigation-learn').focus();await page.keyboard.press('F3');
        await expect(part(page,'navigation-learn')).toHaveCount(0);await expect(part(page,'navigation-learn-content')).toBeHidden();
        await part(page,'toolbar-bold').focus();await page.keyboard.press('F2');await expect(part(page,'toolbar-bold')).toHaveCount(0);await expect(part(page,'toolbar-italic')).toBeFocused();
    });
    test('open-close cycles leave no native timers, observers or page listeners behind',async({page})=>{
        const cycle=async()=>{
            await keyOpen(page);await page.keyboard.press('End');await page.keyboard.press('ArrowRight');await expect(part(page,'menu-sub-copy')).toBeFocused();
            await page.keyboard.press('Escape');await expect(part(page,'menu-sub-trigger')).toBeFocused();await page.keyboard.press('Escape');await expect(part(page,'menu-trigger')).toBeFocused();
            await part(page,'menubar-file').focus();await page.keyboard.press('ArrowDown');await expect(part(page,'menubar-new')).toBeFocused();await page.keyboard.press('Escape');await expect(part(page,'menubar-file')).toBeFocused();
            await part(page,'navigation-learn').focus();await page.keyboard.press('Enter');await expect(part(page,'navigation-learn-content')).toBeVisible();await page.keyboard.press('ArrowDown');await page.keyboard.press('Escape');await expect(part(page,'navigation-learn-content')).toBeHidden();
        };
        await cycle();const before=await closedResources(page);for(let i=0;i<3;i++)await cycle();const after=await closedResources(page);
        expect(after).toEqual(before);expect(after.nativeTimers).toBe(0);expect(after.timers).toBe(0);expect(after.tasks).toBe(0);
    });
    for(const mode of ['dispose','trap'] as const)test(`${mode} of an open menu and navigation leaves the second instance interactive`,async({page})=>{
        await page.evaluate(()=>(window as any).__menus.peer());
        const first=part(page,'menu-parts').first(),peer=page.locator('#menu-peer');const original=await first.elementHandle();
        await first.getByTestId('navigation-learn').click();await first.getByTestId('menu-trigger').focus();await page.keyboard.press('ArrowDown');await page.keyboard.press('End');await page.keyboard.press('ArrowRight');
        if(mode==='trap')await page.evaluate(()=>{try{(window as any).__menus.trap();}catch{}});
        else await page.evaluate(()=>(window as any).__menus.dispose());
        expect(await original!.evaluate(node=>node.isConnected)).toBe(false);await expect(part(page,'menu-parts')).toHaveCount(1);
        await expect.poll(()=>page.evaluate(()=>(window as any).__menus.stats())).toMatchObject({tasks:0,timers:0,observers:0});
        expect(await peer.evaluate(node=>!!node.closest('[inert]'))).toBe(false);
        await peer.getByTestId('toolbar-save').click();await expect(peer.getByTestId('menu-last')).toHaveText('save');
        await peer.getByTestId('menubar-file').focus();await page.keyboard.press('ArrowDown');await expect(page.getByTestId('menubar-new')).toBeFocused();await page.keyboard.press('Escape');
        await page.evaluate(()=>(window as any).__menu_peer.dispose());
        await expect.poll(()=>page.evaluate(()=>(window as any).__menuProbe.timers.size)).toBe(0);
    });
    test('a11y: names, state references and raw/theme separation remain truthful',async({page},info)=>{
        await expect(part(page,'menubar')).toHaveAccessibleName('Application commands');await expect(part(page,'toolbar')).toHaveAccessibleName('Document formatting');
        await expect(part(page,'navigation')).toHaveAccessibleName('Document resources');
        await keyOpen(page);await expect(part(page,'menu-content')).toHaveRole('menu');await expect(part(page,'menu-content')).toHaveAccessibleName('Document commands');
        await expect(part(page,'menu-disabled')).toHaveAccessibleDescription('Requires approval');await expect(part(page,'menu-checked')).toHaveRole('menuitemcheckbox');
        const issues=await page.evaluate(()=>{
            const missing:string[]=[];
            for(const node of document.querySelectorAll('[data-testid=menu-parts] [aria-controls],[data-testid=menu-parts] [aria-labelledby],[data-testid=menu-parts] [aria-describedby],[data-testid=menu-content] [aria-controls],[data-testid=menu-content] [aria-labelledby],[data-testid=menu-content] [aria-describedby]'))for(const name of ['aria-controls','aria-labelledby','aria-describedby'])for(const id of (node.getAttribute(name)??'').split(/\s+/).filter(Boolean))if(document.querySelectorAll(`[id="${CSS.escape(id)}"]`).length!==1)missing.push(`${name}:${id}`);
            return missing;
        });expect(issues).toEqual([]);
        if(info.project.name==='dom-primitives'){
            expect(await page.locator('link[rel=stylesheet]').evaluateAll(nodes=>nodes.map(node=>node.getAttribute('href')))).not.toContain('./rustify.css');
            await expect(part(page,'menu-content')).not.toHaveClass(/rustify-menu/);
        }else await expect(part(page,'menu-content')).toHaveClass(/rustify-menu/);
    });
    for(const width of [390,768,1440])test(`a11y: command surfaces remain reachable at ${width}px`,async({page},info)=>{
        test.skip(info.project.name==='dom-primitives','The raw consumer supplies its own demonstration layout; the SDK skin is the responsive surface.');
        await page.setViewportSize({width,height:900});await part(page,'menu-parts').locator('details').first().locator('summary').click();
        const bar=await part(page,'toolbar').boundingBox(), preview=await part(page,'context-trigger').boundingBox();
        expect(bar!.x+bar!.width).toBeCloseTo(preview!.x+preview!.width,0);
        await part(page,'menu-parts').screenshot({path:`docs/validation/dom-components-p4/m4-shell-closed-${width}.png`});
        await part(page,'menu-trigger').click();await expect(part(page,'menu-content')).toBeFocused();const box=await part(page,'menu-content').boundingBox();expect(box).toBeTruthy();expect(box!.x).toBeGreaterThanOrEqual(0);expect(box!.x+box!.width).toBeLessThanOrEqual(width);
        await page.keyboard.press('Escape');await expect(part(page,'menu-content')).toHaveCount(0);await part(page,'navigation-learn').click();const nav=await part(page,'navigation-learn-content').boundingBox();expect(nav).toBeTruthy();expect(nav!.x+nav!.width).toBeLessThanOrEqual(width);
        expect(await page.evaluate(()=>document.documentElement.scrollWidth)).toBeLessThanOrEqual(width);
        await part(page,'menu-parts').screenshot({path:`docs/validation/dom-components-p4/m4-shell-${width}.png`});
    });
});
