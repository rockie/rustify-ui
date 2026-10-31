import { test, expect, Page } from '@playwright/test';
const errors=new WeakMap<Page,string[]>();

test.beforeEach(async ({page},info)=>{
    const failures:string[]=[];errors.set(page,failures);
    page.on('pageerror',error=>failures.push(error.message));
    page.on('console',message=>{if(message.type()==='error')failures.push(message.text());});
    page.on('requestfailed',request=>failures.push(request.url()));
    await page.goto(info.project.name==='dom-primitives' ? '/' : '/panels.html');
    await expect(page.locator('#status')).toHaveAttribute('data-status','ready');
});
test('manual tabs separate focus from accepted value and preserve panel state',async({page})=>{
    const root=page.getByTestId('manual-tabs'),a=page.getByTestId('manual-a'),b=page.getByTestId('manual-b');
    await page.getByTestId('panel-notes').fill('Saved draft');
    await a.focus();await page.keyboard.press('ArrowRight');await expect(b).toBeFocused();
    await expect(a).toHaveAttribute('aria-selected','true');await expect(page.getByTestId('panel-requests')).toHaveText('0');
    await page.keyboard.press('Enter');await expect(b).toHaveAttribute('aria-selected','true');
    await page.getByTestId('panel-refuse').click();await b.focus();await page.keyboard.press('Home');await expect(a).toBeFocused();
    await page.keyboard.press('Space');await expect(b).toHaveAttribute('aria-selected','true');
    await page.getByTestId('panel-refuse').click();await a.click();await expect(page.getByTestId('panel-notes')).toHaveValue('Saved draft');
    await page.getByTestId('panel-direction').click();await a.focus();await page.keyboard.press('ArrowLeft');await expect(b).toBeFocused();
    await expect(root).toHaveAttribute('dir','rtl');
});
test('removed and disabled focused tabs recover without rewriting application value',async({page})=>{
    const a=page.getByTestId('manual-a'),b=page.getByTestId('manual-b'),c=page.getByTestId('manual-c');
    await b.focus();await page.keyboard.press('Delete');await expect(b).toHaveCount(0);await expect(c).toBeFocused();
    await expect(page.getByTestId('panel-value')).toHaveText('a');
    await page.keyboard.press('d');await expect(c).toBeDisabled();await expect(a).toBeFocused();
    await page.keyboard.press('Delete');await expect(page.getByTestId('manual-tabs')).toBeFocused();
    await expect(page.getByTestId('panel-value')).toHaveText('a');
    await expect(page.getByTestId('panel-notes')).toBeHidden();
});
test('native disclosure requests respect application refusal and hidden contents',async({page})=>{
    const first=page.getByTestId('accordion-first'),second=page.getByTestId('accordion-second');
    await first.click();await expect(first).toHaveAttribute('aria-expanded','true');
    await first.focus();await page.keyboard.press('ArrowDown');await expect(second).toBeFocused();
    await page.keyboard.press('Enter');await expect(second).toHaveAttribute('aria-expanded','true');
    await expect(page.getByTestId('accordion-first-content')).toBeHidden();
    await page.getByTestId('panel-refuse').click();
    await page.getByTestId('collapsible-trigger').click();await expect(page.getByTestId('collapsible-content')).toBeHidden();
    await page.getByTestId('panel-refuse').click();
    await page.getByTestId('collapsible-trigger').click();await expect(page.getByTestId('collapsible-content')).toBeVisible();
    await page.getByTestId('collapsible-trigger').click();await expect(page.getByTestId('collapsible-content')).toHaveAttribute('inert','');
});

test.afterEach(async ({page},info)=>{
    expect(errors.get(page)).toEqual([]);
    if(info.status==='passed' && info.title.startsWith('native disclosure')){
        await page.getByTestId('panel-parts').screenshot({path:info.outputPath('panel-parts.png')});
    }
});

test('accordion uses reordered live nodes and recovers after removal or disabling',async({page})=>{
    const one=page.getByTestId('dynamic-one'),two=page.getByTestId('dynamic-two'),three=page.getByTestId('dynamic-three');
    await one.focus();await page.keyboard.press('r');await page.keyboard.press('ArrowDown');await expect(three).toBeFocused();
    await page.keyboard.press('Delete');await expect(two).toBeFocused();
    await page.keyboard.press('d');await expect(one).toBeFocused();
    await page.getByTestId('remove-outside').click();await expect(page.getByTestId('remove-outside')).toBeFocused();
});

test('automatic tabs accept or refuse one request and native listeners cancel defaults',async({page})=>{
    const x=page.getByTestId('auto-x'),y=page.getByTestId('auto-y'),count=page.getByTestId('auto-request-count');
    await x.focus();await page.keyboard.press('ArrowRight');await expect(y).toBeFocused();await expect(y).toHaveAttribute('aria-selected','true');await expect(count).toHaveText('1');
    await page.getByTestId('panel-refuse').click();await y.focus();await page.keyboard.press('Home');await expect(x).toBeFocused();await expect(y).toHaveAttribute('aria-selected','true');await expect(count).toHaveText('2');
    await page.getByTestId('panel-cancel').click();await y.focus();await page.keyboard.press('Home');await expect(y).toBeFocused();await y.click();await expect(count).toHaveText('2');await expect(page.getByTestId('target-calls')).toHaveText('1');
});

test('themed navigation and segmented controls preserve native form and link outcomes',async({page},info)=>{
    test.skip(info.project.name==='dom-primitives','Themes-only components.');
    const selected=()=>page.evaluate(()=>new FormData(document.querySelector('#panel-choice-form') as HTMLFormElement).get('panel-view'));
    expect((await page.getByRole('radiogroup',{name:'Presentation',exact:true}).boundingBox())!.height).toBe(32);
    expect(await selected()).toBe('list');await page.getByText('Grid',{exact:true}).click();expect(await selected()).toBe('grid');
    await page.getByText('Grid',{exact:true}).click();expect(await selected()).toBe('grid');
    await page.getByRole('link',{name:'Details',exact:true}).click();await expect(page).toHaveURL(/#panel-parts$/);
    const [popup]=await Promise.all([page.waitForEvent('popup'),page.getByRole('link',{name:'Open panel',exact:true}).click()]);
    await expect(popup.locator('#status')).toHaveAttribute('data-status','ready');expect(popup.url()).toContain('/panels.html');await popup.close();
    const style=await page.getByTestId('manual-a').evaluate(element=>{
        const pseudo=getComputedStyle(element,'::after');return {content:pseudo.content,height:pseudo.height,color:pseudo.backgroundColor,width:pseudo.width};
    });
    expect(style.content).toBe('""');expect(style.height).toBe('2px');expect(style.color).not.toBe('rgba(0, 0, 0, 0)');expect(parseFloat(style.width)).toBeGreaterThan(0);
});

test('core position applies four sides and retains finite geometry after rejected configuration',async({page})=>{
    await page.getByTestId('core-anchor').click();const layer=page.getByTestId('core-layer');await expect(layer).toBeVisible();
    for(const side of ['top','right','bottom','left']){
        await page.getByTestId(`core-${side}`).click();await expect(layer).toHaveAttribute('data-side',side);
        await expect.poll(async()=>page.evaluate(side=>{
            const a=document.querySelector('[data-testid="core-anchor"]')!.getBoundingClientRect(),p=document.querySelector('[data-testid="core-layer"]')!.getBoundingClientRect();
            const x=side==='left'?a.left-p.width-8:side==='right'?a.right+8:a.left+(a.width-p.width)/2;
            const y=side==='top'?a.top-p.height-8:side==='bottom'?a.bottom+8:a.top+(a.height-p.height)/2;
            return Math.max(Math.abs(p.x-x),Math.abs(p.y-y));
        },side)).toBeLessThan(1);
    }
    await page.getByTestId('core-invalid').click();await expect(layer).toHaveAttribute('data-rustify-issue','side_offset');
    await page.getByTestId('core-right').click();await expect(layer).toHaveAttribute('data-side','left');
    await page.getByTestId('core-boundary').click();await expect(layer).toHaveAttribute('data-rustify-issue','collision_boundary_ids');await expect(layer).toHaveAttribute('data-side','left');
    expect(await layer.evaluate(element=>Number.isFinite(parseFloat((element as HTMLElement).style.left)))).toBe(true);
    await page.keyboard.press('Escape');await expect(layer).toHaveCount(0);
});


test('core hierarchy keeps descendant portals inside the active modal and retires inactive branches',async({page})=>{
    await page.getByTestId('tree-open').click();
    const outer=page.getByTestId('tree-outer'),child=page.getByTestId('tree-child'),modal=page.getByTestId('tree-modal');
    await expect(outer).toBeVisible();await expect(outer).not.toHaveAttribute('aria-modal');
    await expect(page.getByTestId('tree-first')).toBeFocused();
    await page.getByTestId('tree-first').click();await expect(child).toBeVisible();
    expect(await child.evaluate(element=>!!element.parentElement?.closest('[data-testid="tree-outer"]'))).toBe(true);
    await page.getByTestId('tree-child-last').focus();await page.keyboard.press('Tab');await expect(page.getByTestId('tree-first')).toBeFocused();
    await page.getByTestId('tree-child-first').click();await expect(modal).toBeVisible();await expect(page.getByTestId('tree-modal-first')).toBeFocused();
    await page.getByTestId('tree-modal-last').focus();await page.keyboard.press('Tab');await expect(page.getByTestId('tree-modal-first')).toBeFocused();
    await expect(page.getByTestId('tree-first')).toHaveAttribute('inert','');
    await page.keyboard.press('Escape');await expect(modal).toHaveCount(0);await expect(child).toBeVisible();
    await page.keyboard.press('Escape');await expect(child).toHaveCount(0);await expect(outer).toBeVisible();
    await page.getByTestId('tree-first').click();await expect(child).toBeVisible();
    await page.getByTestId('tree-disable').click();await expect(outer).toBeHidden();await expect(child).toBeHidden();
    await expect(page.getByTestId('tree-close-count')).toHaveText('1');await expect(page.getByTestId('tree-open')).toBeFocused();
    await page.getByTestId('tree-activate').click();await expect(outer).toBeVisible();await expect(child).toBeVisible();
    await expect(page.getByTestId('tree-open-count')).toHaveText('2');
    await page.keyboard.press('Escape');await expect(child).toHaveCount(0);await page.keyboard.press('Escape');await expect(outer).toHaveCount(0);
    await expect(page.getByTestId('tree-close-count')).toHaveText('2');await expect(page.getByTestId('tree-open')).toBeFocused();
});


test('core notification descendants survive modal migration and a modal opened from the notification',async({page},info)=>{
    const toast=page.getByTestId('tree-toast'),popup=page.getByTestId('tree-notice-popup'),outer=page.getByTestId('tree-outer'),modal=page.getByTestId('tree-notice-modal');
    await page.getByTestId('tree-toast-show').click();await expect(toast).toBeVisible();
    await toast.evaluate(element=>(element as HTMLElement).dataset.identity='original');
    await page.getByTestId('tree-open').click();
    await expect.poll(()=>toast.evaluate(element=>!!element.closest('[data-testid="tree-outer"]'))).toBe(true);
    await page.getByTestId('tree-toast-action').click();await expect(popup).toBeVisible();
    const input=page.getByTestId('tree-notice-input');await input.fill('Retained notification task');await input.press('F2');
    await expect(outer).toBeHidden();await expect(popup).toBeVisible();await expect(input).toBeFocused();
    await expect(toast).toHaveAttribute('data-identity','original');await expect(input).toHaveValue('Retained notification task');
    await page.getByTestId('tree-activate').click();await expect(outer).toBeVisible();await expect(popup).toBeVisible();
    await page.getByTestId('tree-notice-confirm').click();await expect(modal).toBeVisible();await expect(page.getByTestId('tree-notice-cancel')).toBeFocused();
    expect(await toast.evaluate(element=>!!element.closest('[data-testid="tree-notice-modal"]'))).toBe(true);
    await expect.poll(async()=>popup.evaluate(element=>element.getBoundingClientRect().bottom-window.innerHeight)).toBeLessThanOrEqual(0);
    await page.screenshot({path:`docs/validation/dom-components-p3/m3-core-${info.project.name}.png`});
    await page.getByTestId('tree-notice-cancel').press('F2');await expect(outer).toBeHidden();await expect(modal).toBeVisible();await expect(popup).toBeVisible();
    await page.keyboard.press('Escape');await expect(modal).toHaveCount(0);await expect(input).toHaveValue('Retained notification task');await expect(page.getByTestId('tree-notice-confirm')).toBeFocused();
    await expect(toast).toHaveAttribute('data-identity','original');
    await page.keyboard.press('Escape');await expect(popup).toHaveCount(0);await expect(toast).toBeVisible();
});


test('core panel owns outside decisions and merges pointer focus with shared cancellation',async({page})=>{
    await page.getByTestId('outside-open').click();await expect(page.getByTestId('outside-panel')).toBeVisible();
    await expect(page.getByTestId('outside-shell')).toHaveAttribute('role','presentation');
    const requests=page.getByTestId('outside-requests'),trace=page.getByTestId('outside-trace');
    await page.getByTestId('outside-input').focus();await page.getByTestId('outside-cover').click();
    await expect(requests).toHaveText('1');await expect(trace).toHaveText('pi');await expect(page.getByTestId('outside-cover')).toBeFocused();
    await page.getByTestId('outside-cancel').click();await page.getByTestId('outside-cover').click();
    await expect(requests).toHaveText('1');await expect(trace).toHaveText('pc');
    await page.getByTestId('outside-input').focus();await page.getByTestId('panel-refuse').focus();await expect(trace).toHaveText('pcfc');await expect(requests).toHaveText('1');
    await page.getByTestId('outside-open').scrollIntoViewIfNeeded();
    await page.getByTestId('outside-open').scrollIntoViewIfNeeded();
    await page.getByTestId('outside-cancel').click();await page.getByTestId('panel-refuse').focus();await expect(trace).toHaveText('fi');await expect(requests).toHaveText('2');
});


for(const part of ['panel','trigger','disabled-trigger'])test(`core lifetime releases an invalid ${part} and recovers accepted open state`,async({page})=>{
    await page.getByTestId('life-open').click();await expect(page.getByTestId('life-panel')).toBeVisible();await expect(page.getByTestId('life-opened')).toHaveText('1');
    await page.getByTestId(part==='panel'?'life-remove-panel':part==='trigger'?'life-remove-trigger':'life-disable-trigger').click();
    await expect(page.getByTestId('life-shell')).toBeHidden();await expect(page.getByTestId('life-requests')).toHaveText('1');
    await expect(page.getByTestId('life-closed')).toHaveText('1');await expect(page.getByTestId('life-depth')).toHaveText('0');
    await page.getByTestId('life-restore').click();await expect(page.getByTestId('life-panel')).toBeVisible();
    await expect(page.getByTestId('life-opened')).toHaveText('2');await expect(page.getByTestId('life-requests')).toHaveText('1');
    await page.getByTestId('life-remove-trigger').click();await expect(page.getByTestId('life-shell')).toBeHidden();
    await expect(page.getByTestId('life-requests')).toHaveText('2');await expect(page.getByTestId('life-closed')).toHaveText('2');
});
test('core outside dismissal restores focus for an unfocusable surface and respects an external focus target',async({page})=>{
    await page.getByTestId('return-open').click();await expect(page.getByTestId('return-input')).toBeVisible();await page.getByTestId('return-input').focus();await expect(page.getByTestId('return-input')).toBeFocused();
    await page.getByTestId('return-external').click();await expect(page.getByTestId('return-shell')).toHaveCount(0);await expect(page.getByTestId('return-external')).toBeFocused();
    await page.getByTestId('return-open').click();await expect(page.getByTestId('return-input')).toBeVisible();await page.getByTestId('return-input').focus();await expect(page.getByTestId('return-input')).toBeFocused();
    await page.getByTestId('return-backdrop').click();await expect(page.getByTestId('return-shell')).toHaveCount(0);await expect(page.getByTestId('return-open')).toBeFocused();
});
test('core modal tab stops distinguish native radio form owners',async({page})=>{
    await page.getByTestId('radio-forms-open').click();await expect(page.getByTestId('radio-form-a')).toBeFocused();
    await page.keyboard.press('Tab');await expect(page.getByTestId('radio-form-b')).toBeFocused();
    await page.keyboard.press('Tab');await expect(page.getByTestId('radio-form-a')).toBeFocused();
    await page.keyboard.press('Shift+Tab');await expect(page.getByTestId('radio-form-b')).toBeFocused();
});


test('core viewport aligns inside visible scope bounds without covering adjacent scopes or the host',async({page},info)=>{
    await expect(page.getByTestId('viewport-error')).toBeEmpty();await page.getByTestId('viewport-row').scrollIntoViewIfNeeded();
    await page.getByTestId('viewport-a-open').click();
    const panel=page.getByTestId('viewport-a-panel'),shade=page.getByTestId('viewport-a-shade');await expect(panel).toBeVisible();
    const geometry=async(start=false)=>page.evaluate(start=>{
        const scope=document.querySelector('[data-testid="viewport-a"]')!.getBoundingClientRect(),panel=document.querySelector('[data-testid="viewport-a-panel"]')!.getBoundingClientRect(),shade=document.querySelector('[data-testid="viewport-a-shade"]')!.getBoundingClientRect();
        const x=Math.max(0,scope.x),y=Math.max(0,scope.y),right=Math.min(document.documentElement.clientWidth,scope.right),bottom=Math.min(document.documentElement.clientHeight,scope.bottom);
        const inset=parseFloat(getComputedStyle(document.documentElement).fontSize);
        return Math.max(Math.abs(panel.x-(x+(right-x-panel.width)/2)),Math.abs(panel.y-(start?y+inset:y+(bottom-y-panel.height)/2)),Math.abs(shade.x-x),Math.abs(shade.y-y),Math.abs(shade.right-right),Math.abs(shade.bottom-bottom),panel.right-right,panel.bottom-bottom);
    },start);
    await expect.poll(()=>geometry()).toBeLessThan(1);
    await page.getByTestId('viewport-a-align').click();await expect.poll(()=>geometry(true)).toBeLessThan(1);
    await page.getByTestId('viewport-b-open').click();await expect(page.getByTestId('viewport-b-panel')).toBeVisible();await expect(panel).toBeVisible();
    await page.getByTestId('viewport-host').click();await expect(page.getByTestId('viewport-host-hits')).toHaveText('1');await expect(page.getByTestId('viewport-host')).toBeFocused();
    await page.screenshot({path:`docs/validation/dom-components-p3/m3-scopes-${info.project.name}.png`});
    await page.getByTestId('viewport-hide').click();await expect(page.getByTestId('viewport-a-shell')).toBeHidden();await expect(page.getByTestId('viewport-a-requests')).toHaveText('1');
    await page.getByTestId('viewport-hide').click();await expect(panel).toBeVisible();await expect(page.getByTestId('viewport-a-requests')).toHaveText('1');
    await page.setViewportSize({width:480,height:720});await expect.poll(()=>geometry(true)).toBeLessThan(1);
    await expect.poll(async()=>panel.evaluate(element=>element.getBoundingClientRect().width)).toBeLessThan(240);
    await page.getByTestId('viewport-a-close').click();await page.getByTestId('viewport-b-close').click();await expect(shade).toHaveCount(0);
});


test('core viewport suspends a fully offscreen scope and recovers on scroll',async({page})=>{
    await page.getByTestId('viewport-row').scrollIntoViewIfNeeded();await page.getByTestId('viewport-a-open').click();
    await expect(page.getByTestId('viewport-a-panel')).toBeVisible();
    await page.evaluate(()=>window.scrollTo(0,0));await expect(page.getByTestId('viewport-a-shell')).toBeHidden();await expect(page.getByTestId('viewport-a-requests')).toHaveText('1');await expect.poll(()=>page.evaluate(()=>scrollY)).toBe(0);
    await page.getByTestId('viewport-row').scrollIntoViewIfNeeded();await expect(page.getByTestId('viewport-a-panel')).toBeVisible();await expect(page.getByTestId('viewport-a-requests')).toHaveText('1');
});

test('dialog parts retain force-mounted state and update live naming relationships',async({page})=>{
    const content=page.getByTestId('parts-dialog-content'),trigger=page.getByTestId('parts-dialog-trigger');
    await expect(content).toBeHidden();await expect(content).toHaveAttribute('inert','');await expect(trigger).toHaveAttribute('data-custom','retained');
    await trigger.click();await expect(content).toBeVisible();await expect(content).toHaveRole('dialog');await expect(content).toHaveAccessibleName('Edit task details');await expect(content).not.toHaveAttribute('aria-modal');
    await expect(page.getByTestId('parts-dialog-input')).toBeFocused();await page.getByTestId('parts-dialog-input').fill('Keep this draft');
    await expect(content).toHaveAccessibleDescription('Changes stay in this task.');
    await page.getByTestId('parts-dialog-title-empty').click();await expect(content).toHaveAttribute('data-rustify-issue','empty-title');await expect(content).not.toHaveAttribute('aria-labelledby');
    await page.getByTestId('parts-dialog-title-empty').click();await expect(content).toHaveAccessibleName('Edit task details');
    await page.getByTestId('parts-dialog-description').click();await expect(content).not.toHaveAttribute('aria-describedby');
    await page.getByTestId('parts-dialog-title-toggle').click();await expect(content).toHaveAttribute('data-rustify-issue','missing-title');await expect(content).not.toHaveAttribute('aria-labelledby');
    await page.getByTestId('parts-dialog-title-toggle').click();await expect(content).toHaveAccessibleName('Edit task details');
    await page.getByTestId('parts-dialog-close').click();await expect(content).toBeHidden();await expect(trigger).toBeFocused();
    await trigger.click();await expect(page.getByTestId('parts-dialog-input')).toHaveValue('Keep this draft');await expect(page.getByTestId('parts-dialog-input')).toBeFocused();await page.keyboard.press('Escape');await expect(content).toBeHidden();
    await expect(page.getByTestId('parts-dialog-opened')).toHaveText('2');await expect(page.getByTestId('parts-dialog-closed')).toHaveText('2');await expect(page.getByTestId('parts-dialog-requests')).toHaveText('4');
});
test('dialog parts honor native cancellation, dismiss hooks, and refused close requests',async({page})=>{
    const content=page.getByTestId('parts-dialog-content'),trigger=page.getByTestId('parts-dialog-trigger'),requests=page.getByTestId('parts-dialog-requests');
    await page.getByTestId('parts-dialog-native').click();await trigger.click();await expect(content).toBeHidden();await expect(requests).toHaveText('0');await expect(page.getByTestId('parts-dialog-targets')).toHaveText('1');
    await page.getByTestId('parts-dialog-native').click();await page.getByTestId('parts-dialog-cancel').click();await trigger.click();await expect(content).toBeVisible();
    await page.keyboard.press('Escape');await expect(content).toBeVisible();await expect(requests).toHaveText('1');
    const overlay=(await page.getByTestId('parts-dialog-overlay').boundingBox())!;await page.mouse.click(overlay.x+4,overlay.y+4);await expect(content).toBeVisible();await expect(requests).toHaveText('1');
    await page.getByTestId('parts-dialog-refuse-inside').click();await page.getByTestId('parts-dialog-close').click();await expect(content).toBeVisible();await expect(requests).toHaveText('2');await expect(page.getByTestId('parts-dialog-closed')).toHaveText('0');
    await page.getByTestId('parts-dialog-refuse-inside').click();await page.getByTestId('parts-dialog-close').click();await expect(content).toBeHidden();await expect(requests).toHaveText('3');
});
test('alert dialog starts on Cancel and only explicit uncanceled Action confirms',async({page})=>{
    await page.getByTestId('parts-dialog-trigger').click();await page.getByTestId('parts-alert-trigger').click();
    const alert=page.getByTestId('parts-alert-content'),actions=page.getByTestId('parts-alert-actions'),requests=page.getByTestId('parts-alert-requests');
    await expect(alert).toBeVisible();await expect(alert).toHaveRole('alertdialog');await expect(alert).toHaveAccessibleName('Remove this task?');await expect(alert).toHaveAccessibleDescription('The task and its unsaved changes will be removed.');await expect(alert).not.toHaveAttribute('aria-modal');
    await expect(page.getByTestId('parts-alert-cancel')).toBeFocused();
    const overlay=(await page.getByTestId('parts-alert-overlay').boundingBox())!;await page.mouse.click(overlay.x+4,overlay.y+4);await expect(alert).toBeVisible();await expect(actions).toHaveText('0');await expect(requests).toHaveText('1');
    await page.keyboard.press('Escape');await expect(alert).toHaveCount(0);await expect(actions).toHaveText('0');await expect(page.getByTestId('parts-dialog-content')).toBeVisible();
    await page.getByTestId('parts-alert-trigger').click();await page.getByTestId('parts-alert-cancel-action').click();await page.getByTestId('parts-alert-action').click();await expect(actions).toHaveText('1');await expect(requests).toHaveText('3');await expect(alert).toBeVisible();
    await page.getByTestId('parts-alert-cancel-action').click();await page.getByTestId('parts-alert-action').click();await expect(actions).toHaveText('2');await expect(requests).toHaveText('4');await expect(alert).toHaveCount(0);
});
test('alert dialog without Cancel diagnoses composition and never focuses the dangerous Action',async({page})=>{
    await page.getByTestId('parts-dialog-trigger').click();await page.getByTestId('parts-alert-missing').click();await page.getByTestId('parts-alert-trigger').click();
    const alert=page.getByTestId('parts-alert-content');await expect(alert).toHaveAttribute('data-rustify-issue','missing-cancel');await expect(alert).toBeFocused();await expect(page.getByTestId('parts-alert-action')).not.toBeFocused();
    await page.keyboard.press('Escape');await expect(alert).toHaveCount(0);await expect(page.getByTestId('parts-alert-actions')).toHaveText('0');
});

test('dialog auto-focus hooks choose the title and an external close destination',async({page})=>{
    await page.getByTestId('parts-dialog-focus').click();await page.getByTestId('parts-dialog-trigger').click();await expect(page.getByTestId('parts-dialog-title')).toBeFocused();
    await page.getByTestId('parts-dialog-close').click();await expect(page.getByTestId('parts-dialog-content')).toBeHidden();await expect(page.getByTestId('parts-dialog-native')).toBeFocused();await expect(page.getByTestId('parts-dialog-closed')).toHaveText('1');
});
test('dialog reopening from a close hook preserves the new session focus',async({page})=>{
    await page.getByTestId('parts-dialog-reopen').click();await page.getByTestId('parts-dialog-trigger').click();await expect(page.getByTestId('parts-dialog-title')).toBeFocused();
    await page.getByTestId('parts-dialog-close').click();await expect(page.getByTestId('parts-dialog-opened')).toHaveText('2');await expect(page.getByTestId('parts-dialog-title')).toBeFocused();await expect(page.getByTestId('parts-dialog-closed')).toHaveText('1');
    await page.getByTestId('parts-dialog-reopen-inside').click();await page.getByTestId('parts-dialog-close').click();await expect(page.getByTestId('parts-dialog-content')).toBeHidden();await expect(page.getByTestId('parts-dialog-closed')).toHaveText('2');
});
test('programmatic modeless dialogs focus content, close once, and respect outside focus',async({page})=>{
    await page.getByTestId('program-dialog-open').click();const content=page.getByTestId('program-dialog-content');await expect(content).toHaveAccessibleName('Modeless task notes');await expect(page.getByTestId('program-dialog-input')).toBeFocused();await expect(page.getByTestId('program-dialog-overlay')).toHaveCount(0);
    await page.getByTestId('program-dialog-close').click();await expect(content).toHaveCount(0);await expect(page.getByTestId('program-dialog-open')).toBeFocused();await expect(page.getByTestId('program-dialog-closed')).toHaveText('1');
    await page.getByTestId('program-dialog-open').click();await expect(page.getByTestId('program-dialog-input')).toBeFocused();await page.getByTestId('panel-refuse').focus();await expect(content).toHaveCount(0);await expect(page.getByTestId('panel-refuse')).toBeFocused();await expect(page.getByTestId('program-dialog-closed')).toHaveText('2');
});

test('dialog repairs duplicate live contents and late triggers without retaining a stale panel',async({page})=>{
    const trigger=page.getByTestId('dynamic-dialog-trigger'),content=page.getByTestId('dynamic-dialog-content');
    await page.getByTestId('dynamic-dialog-trigger-toggle').click();
    await page.getByTestId('dynamic-dialog-trigger-duplicate').click();await expect(trigger).toHaveAttribute('aria-disabled','true');await trigger.focus();await page.keyboard.press('Space');await expect(content).toBeHidden();
    await page.getByTestId('dynamic-dialog-trigger-duplicate').click();await expect(trigger).not.toHaveAttribute('aria-disabled');await trigger.click();await expect(page.getByTestId('dynamic-dialog-input')).toBeFocused();
    await page.getByTestId('dynamic-dialog-duplicate').click();await expect(content).toHaveAttribute('data-rustify-issue','duplicate-part');
    await page.getByTestId('dynamic-dialog-repair').click();await expect(page.getByTestId('dynamic-dialog-content-two')).toHaveCount(0);await expect(content).not.toHaveAttribute('data-rustify-issue');
    await expect(page.getByTestId('dynamic-dialog-input')).toBeFocused();await page.keyboard.press('Escape');await expect(content).toBeHidden();await expect(trigger).toBeFocused();await expect(page.getByTestId('dynamic-dialog-closed')).toHaveText('2');
});

test('alert confirmation respects native cancellation and refused close while validating its live explanation',async({page})=>{
    await page.getByTestId('parts-dialog-trigger').click();await page.getByTestId('parts-dialog-native-inside').click();await page.getByTestId('parts-alert-trigger').click();
    const alert=page.getByTestId('parts-alert-content'),actions=page.getByTestId('parts-alert-actions');
    await page.getByTestId('parts-alert-action').click();await expect(actions).toHaveText('0');await expect(page.getByTestId('parts-alert-requests')).toHaveText('1');await expect(alert).toBeVisible();
    await page.getByTestId('parts-alert-description-empty').click();await expect(alert).toHaveAttribute('data-rustify-issue','missing-description');await expect(alert).not.toHaveAttribute('aria-describedby');
    await page.getByTestId('parts-alert-description-empty').click();await expect(alert).toHaveAccessibleDescription('The task and its unsaved changes will be removed.');
    await page.keyboard.press('Escape');await expect(alert).toHaveCount(0);await page.getByTestId('parts-dialog-native-inside').click();await page.getByTestId('parts-alert-trigger').click();
    await page.getByTestId('parts-alert-refuse').click();await page.getByTestId('parts-alert-action').click();await expect(actions).toHaveText('1');await expect(alert).toBeVisible();await expect(page.getByTestId('parts-alert-requests')).toHaveText('4');
    await page.getByTestId('parts-alert-refuse').click();await page.getByTestId('parts-alert-cancel').click();await expect(alert).toHaveCount(0);await expect(actions).toHaveText('1');await expect(page.getByTestId('parts-dialog-content')).toBeVisible();
});

test('dialog surfaces keep content and nested confirmation inside the desktop and narrow viewport',async({page},info)=>{
    await page.getByTestId('parts-dialog-trigger').click();await expect(page.getByTestId('parts-dialog-input')).toBeFocused();
    const content=page.getByTestId('parts-dialog-content');
    const overflow=()=>content.evaluate(element=>{const r=element.getBoundingClientRect();return Math.max(-r.x,-r.y,r.right-document.documentElement.clientWidth,r.bottom-innerHeight);});
    await expect.poll(overflow).toBeLessThanOrEqual(1);await page.screenshot({path:`docs/validation/dom-components-p3/m3-dialog-desktop-${info.project.name}.png`});
    await page.setViewportSize({width:480,height:720});await expect.poll(overflow).toBeLessThanOrEqual(1);await page.screenshot({path:`docs/validation/dom-components-p3/m3-dialog-narrow-${info.project.name}.png`});
    await page.getByTestId('parts-alert-trigger').click();await expect(page.getByTestId('parts-alert-cancel')).toBeFocused();
    const alert=page.getByTestId('parts-alert-content');await expect.poll(()=>alert.evaluate(element=>{const r=element.getBoundingClientRect();return Math.max(-r.x,-r.y,r.right-document.documentElement.clientWidth,r.bottom-innerHeight);})).toBeLessThanOrEqual(1);
    await page.screenshot({path:`docs/validation/dom-components-p3/m3-alert-narrow-${info.project.name}.png`});
    await page.keyboard.press('Escape');await expect(alert).toHaveCount(0);await expect(page.getByTestId('parts-alert-trigger')).toBeFocused();await page.getByTestId('parts-dialog-close').click();await expect(content).toBeHidden();
});

test('popover separates position from native trigger and toggles with one accepted request',async({page})=>{
    const trigger=page.getByTestId('popover-trigger'),content=page.getByTestId('popover-content'),requests=page.getByTestId('popover-requests');
    await expect(content).toBeHidden();await expect(trigger).toHaveAttribute('data-custom','retained');
    await page.getByTestId('popover-native').click();await trigger.click();await expect(content).toBeHidden();await expect(requests).toHaveText('0');await expect(page.getByTestId('popover-targets')).toHaveText('1');
    await page.getByTestId('popover-native').click();await trigger.click();await expect(page.getByTestId('popover-input')).toBeFocused();await expect(content).toHaveRole('dialog');await expect(content).toHaveAccessibleName('Task schedule');await expect(content).not.toHaveAttribute('aria-modal');await expect(content).toHaveAttribute('data-side','bottom');await expect(content).toHaveAttribute('data-align','start');
    await expect.poll(async()=>{
        const [a,c]=await Promise.all([page.getByTestId('popover-anchor').boundingBox(),content.boundingBox()]);return Math.max(Math.abs(c!.x-a!.x),Math.abs(c!.y-a!.y-a!.height-8));
    }).toBeLessThan(1);
    await trigger.click();await expect(content).toBeHidden();await expect(requests).toHaveText('2');await expect(trigger).toBeFocused();
});
test('popover keeps refused state, cancels dismiss defaults and recovers its removed anchor',async({page})=>{
    await page.getByTestId('popover-cancel').click();await page.getByTestId('popover-trigger').click();
    const content=page.getByTestId('popover-content'),requests=page.getByTestId('popover-requests');await expect(page.getByTestId('popover-input')).toBeFocused();await page.getByTestId('popover-arrow-toggle').click();await expect(page.getByTestId('popover-arrow')).toBeVisible();
    await page.getByTestId('popover-name').click();await expect(content).toHaveAttribute('data-rustify-issue','missing-name');await expect(content).not.toHaveAttribute('aria-label');await page.getByTestId('popover-name').click();await expect(content).toHaveAccessibleName('Task schedule');await expect(content).not.toHaveAttribute('data-rustify-issue');
    await page.keyboard.press('Escape');await expect(content).toBeVisible();await page.getByTestId('popover-outside').click();await expect(content).toBeVisible();await expect(page.getByTestId('popover-outside')).toBeFocused();await expect(page.getByTestId('popover-clicks')).toHaveText('1');await expect(requests).toHaveText('1');
    await page.getByTestId('popover-input').fill('Retained schedule');await page.getByTestId('popover-refuse').click();await page.getByTestId('popover-close').click();await expect(content).toBeVisible();await expect(requests).toHaveText('2');
    await page.getByTestId('popover-remove').click();await expect(content).toBeHidden();await expect(page.getByTestId('popover-arrow')).toBeHidden();await expect(requests).toHaveText('3');
    await page.getByTestId('popover-restore').click();await expect(content).toBeVisible();await expect(page.getByTestId('popover-arrow')).toBeVisible();await expect(page.getByTestId('popover-input')).toHaveValue('Retained schedule');await expect(requests).toHaveText('3');
    await page.getByTestId('popover-refuse').click();await page.getByTestId('popover-close').click();await expect(content).toBeHidden();await expect(requests).toHaveText('4');
});

test('themed floating surfaces enforce explicit max dimensions over the structural defaults',async({page},info)=>{
    test.skip(info.project.name==='dom-primitives','Themed geometry');
    await page.getByTestId('popover-trigger').click();const popup=page.getByTestId('popover-content');await expect(popup).toHaveCSS('padding-top','16px');await expect(popup).toHaveCSS('border-radius','8px');await expect(popup).toHaveCSS('width','200px');await page.getByTestId('popover-close').click();
    await page.getByTestId('parts-dialog-trigger').click();await page.getByTestId('parts-dialog-layout').click();const dialog=page.getByTestId('parts-dialog-content');await expect(dialog).toHaveCSS('width','300px');await expect(dialog).toHaveCSS('height','180px');await page.getByTestId('parts-dialog-close').click();
});
test('modeless popover lets Shift Tab reach the preceding page control without stealing focus',async({page})=>{
    await page.getByTestId('popover-trigger').click();await expect(page.getByTestId('popover-input')).toBeFocused();
    await page.keyboard.press('Shift+Tab');await expect(page.getByTestId('stage-notes')).toBeFocused();await expect(page.getByTestId('popover-content')).toBeHidden();await expect(page.getByTestId('popover-requests')).toHaveText('2');
});

test('popover arrow measures each physical side, follows its resized depth and inherits the panel surface',async({page},info)=>{
    await page.getByTestId('popover-trigger').click();await page.getByTestId('popover-arrow-toggle').click();
    const content=page.getByTestId('popover-content'),arrow=page.getByTestId('popover-arrow');await expect(arrow).toBeVisible();await arrow.evaluate(element=>(element as HTMLElement).dataset.identity='original');
    const error=async(side:string)=>{
        const [a,c,t]=await Promise.all([page.getByTestId('popover-anchor').boundingBox(),content.boundingBox(),arrow.boundingBox()]);
        if(side==='bottom')return Math.max(Math.abs(c!.y-a!.y-a!.height-8-t!.height),Math.abs(t!.y+t!.height-c!.y),Math.abs(t!.x+t!.width/2-a!.x-a!.width/2));
        if(side==='top')return Math.max(Math.abs(a!.y-c!.y-c!.height-8-t!.height),Math.abs(t!.y-c!.y-c!.height),Math.abs(t!.x+t!.width/2-a!.x-a!.width/2));
        if(side==='left')return Math.max(Math.abs(a!.x-c!.x-c!.width-8-t!.width),Math.abs(t!.x-c!.x-c!.width),Math.abs(t!.y+t!.height/2-a!.y-a!.height/2));
        return Math.max(Math.abs(c!.x-a!.x-a!.width-8-t!.width),Math.abs(t!.x+t!.width-c!.x),Math.abs(t!.y+t!.height/2-a!.y-a!.height/2));
    };
    for(const side of ['top','right','left','bottom']){
        await page.getByTestId(`popover-place-${side}`).click();await expect(content).toHaveAttribute('data-side',side);await expect(arrow).toHaveAttribute('data-side',side);await expect.poll(()=>error(side)).toBeLessThan(1);
    }
    await arrow.evaluate(element=>(element as HTMLElement).style.setProperty('--rustify-floating-arrow-height','12px'));await expect(arrow).toHaveCSS('height','12px');await expect.poll(()=>error('bottom')).toBeLessThan(1);await expect(arrow).toHaveAttribute('data-identity','original');
    await content.evaluate(element=>(element as HTMLElement).style.backgroundColor='#ffe4a3');await expect(arrow).toHaveCSS('background-color','rgb(255, 228, 163)');
    await page.screenshot({path:`docs/validation/dom-components-p3/m4-popover-arrow-${info.project.name}.png`});
    await page.getByTestId('popover-arrow-toggle').click();await expect(arrow).toHaveCount(0);await expect.poll(async()=>{
        const [a,c]=await Promise.all([page.getByTestId('popover-anchor').boundingBox(),content.boundingBox()]);return Math.abs(c!.y-a!.y-a!.height-8);
    }).toBeLessThan(1);await page.getByTestId('popover-close').click();await expect(content).toBeHidden();
});

test('floating geometry batches dirty microtasks and only polls under Always',async({page})=>{
    await page.getByTestId('popover-trigger').click();await page.getByTestId('popover-arrow-toggle').click();await expect(page.getByTestId('popover-arrow')).toBeVisible();
    const result=await page.evaluate(async()=>{
        const panel=document.querySelector<HTMLElement>('[data-testid="popover-content"]')!,anchor=document.querySelector<HTMLElement>('[data-testid="popover-anchor"]')!;
        const mode=document.querySelector<HTMLButtonElement>('[data-testid="popover-animation"]')!,close=document.querySelector<HTMLButtonElement>('[data-testid="popover-close"]')!;
        const originalFrame=window.requestAnimationFrame,native=originalFrame.bind(window),originalRect=Element.prototype.getBoundingClientRect;
        const frames=async(n:number)=>{for(let i=0;i<n;i++)await new Promise<void>(resolve=>native(()=>resolve()));};
        await frames(3);let frame=0,ticking=true;const reads:Record<number,number>={};const tick=()=>{frame++;if(ticking)native(tick);};native(tick);
        let requests=0,callbacks=0;let animation:Animation|undefined;
        try {
            Element.prototype.getBoundingClientRect=function(){if(this===panel)reads[frame]=(reads[frame]||0)+1;return originalRect.call(this);};
            for(let i=0;i<8;i++){window.dispatchEvent(new Event('resize'));await new Promise<void>(resolve=>queueMicrotask(resolve));}
            await frames(3);Element.prototype.getBoundingClientRect=originalRect;ticking=false;
            window.requestAnimationFrame=callback=>{requests++;return native(time=>{callbacks++;callback(time);});};
            mode.click();await frames(3);requests=0;callbacks=0;
            animation=anchor.animate([{transform:'translateX(0)'},{transform:'translateX(48px)'}],{duration:160,fill:'forwards'});await animation.finished;await frames(2);
            const delta=Math.abs(panel.getBoundingClientRect().x-anchor.getBoundingClientRect().x),always=requests;
            mode.click();await frames(3);requests=0;await frames(3);const optimized=requests;
            mode.click();await frames(3);callbacks=0;close.click();await Promise.resolve();await frames(3);const closed=callbacks;
            return {peakReads:Math.max(0,...Object.values(reads)),delta,always,optimized,closed};
        } finally {ticking=false;Element.prototype.getBoundingClientRect=originalRect;window.requestAnimationFrame=originalFrame;animation?.cancel();}
    });
    expect(result.peakReads).toBeGreaterThan(0);expect(result.peakReads).toBeLessThanOrEqual(1);expect(result.delta).toBeLessThan(1);expect(result.always).toBeGreaterThan(0);expect(result.optimized).toBe(0);expect(result.closed).toBe(0);
});

test('tooltip focus describes the native target, preserves other descriptions, and never moves focus on Escape',async({page})=>{
    const trigger=page.getByTestId('tooltip-one'),tip=page.getByTestId('tooltip-one-content');
    await trigger.focus();await expect(tip).toBeVisible();await expect(tip).toHaveRole('tooltip');await expect(trigger).toBeFocused();
    const id=(await tip.getAttribute('id'))!;await expect(trigger).toHaveAttribute('aria-describedby',`hover-help ${id}`);
    await trigger.evaluate(element=>element.setAttribute('aria-describedby',`${element.getAttribute('aria-describedby')} hover-added hover-added`));
    await expect.poll(async()=>(await trigger.getAttribute('aria-describedby'))!.split(' ').sort()).toEqual(['hover-help',id,'hover-added'].sort());
    await page.keyboard.press('Escape');await expect(tip).toBeHidden();await expect(trigger).toBeFocused();await expect(trigger).toHaveAttribute('aria-describedby','hover-help hover-added');
    await page.keyboard.press('ArrowRight');await expect(tip).toBeHidden();
    await page.getByTestId('hover-outside').focus();await trigger.focus();await expect(tip).toBeVisible();
    await trigger.click();await expect(tip).toBeHidden();await expect(trigger).toBeFocused();await expect(page.getByTestId('hover-targets')).toHaveText('1');
});

test('hover timers honor controlled refusal, provider skip delay, opposite input, and unmount',async({page})=>{
    await page.evaluate(()=>{
        const schedule=window.setTimeout.bind(window),clear=window.clearTimeout.bind(window);
        const held=new Map<number,{delay:number,cancelled:boolean}>();(window as any).hoverTimers=held;
        window.setTimeout=((callback:TimerHandler,delay?:number,...args:any[])=>{const id=schedule(callback,delay,...args);if(delay===120)held.set(id,{delay,cancelled:false});return id;}) as typeof setTimeout;
        window.clearTimeout=(id?:number)=>{if(id!==undefined){const timer=held.get(id);if(timer)timer.cancelled=true;}clear(id);};
    });
    const first=page.getByTestId('tooltip-one'),second=page.getByTestId('tooltip-two'),tip=page.getByTestId('tooltip-one-content');
    const enter={pointerType:'mouse',clientX:10,clientY:10};
    await first.dispatchEvent('pointerenter',enter);await expect(tip).toBeHidden();await expect(page.getByTestId('hover-requests')).toHaveText('0');
    await first.dispatchEvent('pointerleave',enter);
    expect(await page.evaluate(()=>Array.from((window as any).hoverTimers.values()))).toEqual([{delay:120,cancelled:true}]);
    await first.dispatchEvent('pointerenter',enter);await expect(tip).toBeVisible();
    await first.dispatchEvent('click');await expect(tip).toBeHidden();
    await second.dispatchEvent('pointerenter',enter);await expect(page.getByTestId('tooltip-two-content')).toBeVisible();
    expect(await page.evaluate(()=>(window as any).hoverTimers.size)).toBe(2);
    await second.dispatchEvent('click');await expect(page.getByTestId('tooltip-two-content')).toHaveCount(0);
    // Observe beyond the declared skip window before testing a fresh delayed request.
    await page.waitForTimeout(450);
    await page.getByTestId('hover-accept').dispatchEvent('click');await first.dispatchEvent('pointerenter',enter);await expect(page.getByTestId('hover-requests')).toHaveText('3');await expect(tip).toBeHidden();
    await page.getByTestId('hover-accept').dispatchEvent('click');await first.dispatchEvent('pointerenter',enter);await page.getByTestId('hover-unmount').dispatchEvent('click');
    expect(await page.evaluate(()=>Array.from((window as any).hoverTimers.values()).every((timer:any)=>timer.cancelled))).toBe(true);
    await page.waitForTimeout(150);await expect(page.getByTestId('hover-requests')).toHaveText('3');
    await page.getByTestId('hover-unmount').dispatchEvent('click');await expect(tip).toBeHidden();
    await first.dispatchEvent('pointerenter',enter);await first.evaluate(element=>element.remove());
    await expect.poll(()=>page.evaluate(()=>Array.from((window as any).hoverTimers.values()).every((timer:any)=>timer.cancelled))).toBe(true);
    await page.waitForTimeout(150);await expect(page.getByTestId('hover-requests')).toHaveText('3');
});

test('tooltip native cancellation and invalid interactive content stay observable',async({page})=>{
    const trigger=page.getByTestId('tooltip-one'),tip=page.getByTestId('tooltip-one-content');
    await page.getByTestId('hover-native').click();
    await trigger.dispatchEvent('focusin',{bubbles:true,cancelable:true});await trigger.dispatchEvent('pointerenter',{pointerType:'mouse',cancelable:true});
    await expect(tip).toBeHidden();await expect(page.getByTestId('hover-requests')).toHaveText('0');
    await trigger.focus();await expect(tip).toBeVisible();await trigger.click();await expect(tip).toBeVisible();await expect(page.getByTestId('hover-requests')).toHaveText('1');
    await page.keyboard.press('Escape');await expect(tip).toBeHidden();await expect(trigger).toBeFocused();
    await page.getByTestId('hover-outside').focus();await page.getByTestId('hover-native').click();await page.getByTestId('hover-interactive').click();await trigger.focus();await expect(tip).toHaveAttribute('data-rustify-issue','interactive-tooltip');
    await expect(trigger).toBeFocused();await page.keyboard.press('Escape');await expect(trigger).toBeFocused();
});

test('hover reading crosses the real pointer corridor and preserves native links and selected text',async({page})=>{
    const trigger=page.getByTestId('hover-card-trigger'),card=page.getByTestId('hover-card-content');
    await trigger.hover();await expect(card).toBeVisible();await expect(card).not.toHaveAttribute('role');await expect(trigger).not.toBeFocused();
    const a=(await trigger.boundingBox())!,b=(await card.boundingBox())!;
    await page.mouse.move(a.x+a.width/2,a.y+a.height-1);await page.mouse.move((a.x+a.width/2+b.x+b.width/2)/2,(a.y+a.height+b.y)/2);await expect(card).toBeVisible();
    await page.getByTestId('hover-selectable').hover();await expect(card).toBeVisible();
    await page.getByTestId('hover-selectable').evaluate(element=>{const range=document.createRange();range.selectNodeContents(element);const selection=getSelection()!;selection.removeAllRanges();selection.addRange(range);});
    await page.mouse.move(5,5);await expect(card).toBeVisible();
    await page.evaluate(()=>getSelection()!.removeAllRanges());await expect(card).toBeHidden();
    await trigger.focus();await expect(card).toBeVisible();await expect(trigger).toBeFocused();await page.getByTestId('hover-card-link').click();await expect(page).toHaveURL(/#panel-parts$/);
});

test('detached anchored content and its arrow suspend without changing controlled open state',async({page})=>{
    const content=page.getByTestId('popover-content'),trigger=page.getByTestId('popover-trigger'),anchor=page.getByTestId('popover-anchor');
    await trigger.click();await page.getByTestId('popover-arrow-toggle').click();await expect(page.getByTestId('popover-arrow')).toBeVisible();
    await page.getByTestId('popover-detached').click();await page.getByTestId('popover-animation').click();
    await anchor.evaluate(element=>(element as HTMLElement).style.transform='translateX(-3000px)');
    await expect(content).toBeHidden();await expect(page.getByTestId('popover-arrow')).toBeHidden();await expect(trigger).toHaveAttribute('aria-expanded','true');await expect(page.getByTestId('popover-requests')).toHaveText('1');
    await anchor.evaluate(element=>(element as HTMLElement).style.transform='');await expect(content).toBeVisible();await expect(page.getByTestId('popover-arrow')).toBeVisible();await expect(page.getByTestId('popover-requests')).toHaveText('1');
});

test('local task panel preserves drafts and closes only the top of Dialog Popover Select',async({page})=>{
    await page.getByTestId('stage-notes').fill('Retained summary draft');await page.getByTestId('stage-details-tab').click();
    await page.getByTestId('stage-detail-notes').fill('Retained review draft');await page.getByTestId('stage-checklist').click();await expect(page.getByTestId('stage-checklist-content')).toBeVisible();
    await page.getByTestId('stage-references').click();await expect(page.getByTestId('stage-reference-content')).toBeVisible();
    await page.getByTestId('stage-summary-tab').click();await expect(page.getByTestId('stage-notes')).toHaveValue('Retained summary draft');
    await page.getByTestId('stage-edit').click();await expect(page.getByTestId('stage-draft-name')).toBeFocused();await page.getByTestId('stage-draft-name').fill('Unsaved title');
    await page.getByTestId('stage-options').click();await expect(page.getByTestId('stage-status-trigger')).toBeFocused();await page.keyboard.press('Space');await expect(page.getByTestId('stage-status-list')).toBeVisible();
    await page.keyboard.press('Escape');await expect(page.getByTestId('stage-status-list')).toHaveCount(0);await expect(page.getByTestId('stage-options-content')).toBeVisible();await expect(page.getByTestId('stage-status-trigger')).toBeFocused();
    await page.keyboard.press('Escape');await expect(page.getByTestId('stage-options-content')).toHaveCount(0);await expect(page.getByTestId('stage-editor')).toBeVisible();await expect(page.getByTestId('stage-options')).toBeFocused();
    await page.keyboard.press('Escape');await expect(page.getByTestId('stage-editor')).toHaveCount(0);await expect(page.getByTestId('stage-edit')).toBeFocused();await expect(page.getByTestId('stage-name')).toHaveText('Launch checklist');
    await page.getByTestId('stage-edit').click();await expect(page.getByTestId('stage-draft-name')).toHaveValue('Launch checklist');await page.getByTestId('stage-cancel').click();
    await page.getByTestId('stage-details-tab').click();await expect(page.getByTestId('stage-detail-notes')).toHaveValue('Retained review draft');
});

test('local task edits save accepted fields and dangerous confirmation defaults to Keep',async({page})=>{
    await page.getByTestId('stage-edit').click();await page.getByTestId('stage-draft-name').fill('Release checklist');await page.getByTestId('stage-options').click();await page.getByTestId('stage-status-trigger').click();
    await page.getByTestId('stage-status-ready').click();await expect(page.getByTestId('stage-status-trigger')).toBeFocused();await expect(page.getByTestId('stage-options-content')).toBeVisible();
    await page.getByTestId('stage-options-close').click();await page.getByTestId('stage-save').click();await expect(page.getByTestId('stage-name')).toHaveText('Release checklist');await expect(page.getByTestId('stage-status')).toHaveText('Ready');await expect(page.getByTestId('stage-feedback')).toHaveText('Task changes saved.');
    await page.getByTestId('stage-delete').click();await expect(page.getByTestId('stage-keep')).toBeFocused();await page.getByTestId('stage-keep').click();await expect(page.getByTestId('stage-name')).toHaveText('Release checklist');
    await page.getByTestId('stage-delete').click();await expect(page.getByTestId('stage-keep')).toBeFocused();await page.getByTestId('stage-confirm-delete').click();await expect(page.getByTestId('stage-empty')).toBeVisible();await expect(page.getByTestId('stage-edit')).toBeDisabled();
    await page.getByTestId('stage-restore').click();await expect(page.getByTestId('stage-name')).toHaveText('Launch checklist');await expect(page.getByTestId('stage-status')).toHaveText('Planned');await expect(page.getByTestId('stage-edit')).toBeEnabled();
});

test('task panel projects live local appearance and RTL into narrow nested content',async({page},info)=>{
    const rootStyle=await page.locator('html').getAttribute('style');
    await page.getByTestId('panel-theme').click();await page.getByTestId('panel-reading-direction').click();
    await page.setViewportSize({width:390,height:844});await page.getByTestId('stage-edit').click();await expect(page.getByTestId('stage-draft-name')).toBeFocused();await page.getByTestId('stage-options').click();
    const content=page.getByTestId('stage-options-content'),shell=content.locator('xpath=ancestor::*[contains(concat(" ",normalize-space(@class)," ")," rustify-layer ")][1]');
    await expect(shell).toHaveAttribute('data-theme','dark');await expect(shell).toHaveAttribute('dir','rtl');
    await expect.poll(async()=>content.evaluate(element=>{const r=element.getBoundingClientRect();return Math.max(-r.left,r.right-document.documentElement.clientWidth,-r.top,r.bottom-innerHeight);})).toBeLessThanOrEqual(1);
    const dark=await content.evaluate(element=>getComputedStyle(element).backgroundColor);
    if(info.project.name==='dom-only'){
        const surfaces=await page.locator('[data-testid=stage-editor],[data-testid=stage-options-content]').evaluateAll(elements=>elements.map(element=>{const color=getComputedStyle(element).backgroundColor;return color.startsWith('rgba(')?Number(color.slice(5,-1).split(',')[3]):1;}));
        expect(surfaces).toEqual([1,1]);
        await page.screenshot({path:`docs/validation/dom-components-p3/m4-task-${info.project.name}-390-rtl-dark.png`});
    }
    await page.getByTestId('panel-theme').dispatchEvent('click');await expect(shell).toHaveAttribute('data-theme','light');
    if(info.project.name==='dom-only')await expect.poll(()=>content.evaluate(element=>getComputedStyle(element).backgroundColor)).not.toBe(dark);
    await page.getByTestId('stage-status-trigger').click();await expect(page.getByTestId('stage-status-list')).toBeVisible();await page.screenshot({path:`docs/validation/dom-components-p3/m4-task-${info.project.name}-390-rtl.png`});
    await page.keyboard.press('Escape');await page.keyboard.press('Escape');await page.keyboard.press('Escape');
    await page.setViewportSize({width:1280,height:900});await page.getByTestId('stage-panel').scrollIntoViewIfNeeded();await page.getByTestId('stage-panel').screenshot({path:`docs/validation/dom-components-p3/m4-task-${info.project.name}.png`});
    expect(await page.locator('html').getAttribute('style')).toBe(rootStyle);
});

test('modal Popover constrains its scope while independent scope and host keep native interaction',async({page})=>{
    await page.getByTestId('viewport-row').scrollIntoViewIfNeeded();await page.getByTestId('viewport-a-popover-open').click();
    const content=page.getByTestId('viewport-a-popover-content');await expect(content).toBeVisible();await expect(content).toHaveRole('dialog');await expect(content).not.toHaveAttribute('aria-modal');await expect(page.getByTestId('viewport-a-popover-input')).toBeFocused();
    await page.getByTestId('viewport-a-popover-refuse').click();expect(await page.getByTestId('viewport-a-open').evaluate(element=>!!element.closest('[inert]'))).toBe(true);
    await page.getByTestId('viewport-b-open').click();await expect(page.getByTestId('viewport-b-panel')).toBeVisible();await expect(page.getByTestId('viewport-b-align')).toBeFocused();await expect(content).toBeVisible();
    await page.getByTestId('viewport-host').click();await expect(page.getByTestId('viewport-host-hits')).toHaveText('1');await expect(page.getByTestId('viewport-host')).toBeFocused();await expect(content).toBeVisible();
    await page.getByTestId('viewport-a-popover-input').focus();await page.keyboard.press('Shift+Tab');await expect(page.getByTestId('viewport-a-popover-content').getByRole('button',{name:'Close options'})).toBeFocused();
});

test.describe('touch supplemental entry',()=>{
    test.use({hasTouch:true,isMobile:true,viewport:{width:390,height:844}});
    test('tap opens the editing panel while supplemental links retain their destination',async({page})=>{
        await page.getByTestId('popover-trigger').tap();await expect(page.getByTestId('popover-content')).toBeVisible();await page.getByTestId('popover-input').fill('Touch schedule');await page.getByTestId('popover-close').tap();await expect(page.getByTestId('popover-content')).toBeHidden();
        await page.getByTestId('tooltip-one').tap();await expect(page.getByTestId('tooltip-one-content')).toBeHidden();
        await page.getByTestId('hover-card-trigger').tap();await expect(page).toHaveURL(/#panel-parts$/);await expect(page.getByTestId('hover-card-content')).toHaveCount(0);
    });
});

test('all themed panel sizes consume their measured padding and caller utilities override it',async({page},info)=>{
    test.skip(info.project.name==='dom-primitives','Themed geometry is a separate declared surface.');
    for(const [trigger,content,control,paddings] of [
        ['parts-dialog-trigger','parts-dialog-content','parts-dialog-size',[24,32,12,16]],
        ['popover-trigger','popover-content','popover-size',[16,24,32,12]],
        ['hover-card-trigger','hover-card-content','hover-size',[16,24,12]],
    ] as const){
        await page.getByTestId(trigger).focus();if(trigger!=='hover-card-trigger')await page.getByTestId(trigger).click();
        const surface=page.getByTestId(content);await expect(surface).toBeVisible();
        for(const padding of paddings){await expect.poll(()=>surface.evaluate(element=>parseFloat(getComputedStyle(element).paddingTop))).toBe(padding);await page.getByTestId(control).dispatchEvent('click');}
        await surface.evaluate(element=>element.classList.add('p-0'));await expect.poll(()=>surface.evaluate(element=>getComputedStyle(element).paddingTop)).toBe('0px');
        await surface.evaluate(element=>element.classList.remove('p-0'));await page.keyboard.press('Escape');await expect(surface).toBeHidden();
    }
});

test('long task names wrap at tablet width and controls remain reachable in a reduced viewport',async({page},info)=>{
    await page.emulateMedia({reducedMotion:'reduce'});await page.setViewportSize({width:768,height:900});
    await page.getByTestId('stage-edit').click();const name='https://example.test/release/'+'checklist-owner-review-'.repeat(12);await page.getByTestId('stage-draft-name').fill(name);await page.getByTestId('stage-save').click();
    await expect(page.getByTestId('stage-name')).toHaveText(name);expect(await page.getByTestId('stage-name').evaluate(element=>element.scrollWidth-element.clientWidth)).toBeLessThanOrEqual(1);
    // Half of the desktop CSS viewport simulates 200% layout density; it is not browser UI zoom.
    await page.setViewportSize({width:640,height:450});await page.getByTestId('stage-delete').click();await expect(page.getByTestId('stage-keep')).toBeFocused();
    await expect.poll(()=>page.getByTestId('stage-confirmation').evaluate(element=>{const r=element.getBoundingClientRect();return Math.max(-r.top,r.bottom-innerHeight,-r.left,r.right-innerWidth);})).toBeLessThanOrEqual(1);
    await page.screenshot({path:`docs/validation/dom-components-p3/m4-task-${info.project.name}-zoom-simulation.png`});await page.getByTestId('stage-keep').click();await expect(page.getByTestId('stage-name')).toHaveText(name);
});

test('refusing tooltip close cannot retain work after its live trigger disappears',async({page})=>{
    const trigger=page.getByTestId('tooltip-one'),tip=page.getByTestId('tooltip-one-content');
    await trigger.focus();await expect(tip).toBeVisible();await page.getByTestId('hover-accept').dispatchEvent('click');await trigger.evaluate(element=>element.remove());
    await expect(tip).toBeHidden();await expect(page.getByTestId('hover-requests')).toHaveText('2');
    await page.getByTestId('hover-outside').click();await expect(page.getByTestId('hover-outside')).toBeFocused();await expect(page.getByTestId('hover-requests')).toHaveText('2');
});

test('delayed application decisions never become temporary displayed open state',async({page})=>{
    await page.getByTestId('popover-delay').click();await page.getByTestId('popover-trigger').click();await expect(page.getByTestId('popover-requests')).toHaveText('1');await expect(page.getByTestId('popover-content')).toBeHidden();await expect(page.getByTestId('popover-trigger')).toHaveAttribute('aria-expanded','false');
    await page.getByTestId('popover-apply').click();await expect(page.getByTestId('popover-input')).toBeFocused();await page.getByTestId('popover-input').fill('Accepted later');
    await page.getByTestId('popover-close').click();await expect(page.getByTestId('popover-requests')).toHaveText('2');await expect(page.getByTestId('popover-content')).toBeVisible();await expect(page.getByTestId('popover-input')).toHaveValue('Accepted later');
    await page.getByTestId('popover-apply').click();await expect(page.getByTestId('popover-content')).toBeHidden();await expect(page.getByTestId('popover-apply')).toBeFocused();
});

test('logical alignment changes in RTL and a live scope boundary flips and shifts the measured panel',async({page})=>{
    await page.getByTestId('popover-trigger').click();const content=page.getByTestId('popover-content');await expect(content).toBeVisible();
    for(const rtl of [false,true]){
        if(rtl)await page.getByTestId('panel-reading-direction').dispatchEvent('click');
        for(const align of ['start','center','end']){
            await page.getByTestId(`popover-align-${align}`).click();await expect(content).toHaveAttribute('data-align',align);
            await expect.poll(()=>page.evaluate(({rtl,align})=>{const a=document.querySelector('[data-testid=popover-anchor]')!.getBoundingClientRect(),p=document.querySelector('[data-testid=popover-content]')!.getBoundingClientRect();const x=align==='center'?a.left+(a.width-p.width)/2:((align==='start')!==rtl)?a.left:a.right-p.width;return Math.abs(x-p.left);},{rtl,align})).toBeLessThan(1);
        }
    }
    await page.getByTestId('popover-arrow-toggle').click();await page.getByTestId('popover-boundary').click();
    await page.getByTestId('popover-anchor').evaluate(element=>{const b=document.getElementById('panel-boundary')!.getBoundingClientRect();const bottom=Math.min(innerHeight,b.bottom),right=Math.min(document.documentElement.clientWidth,b.right);Object.assign((element as HTMLElement).style,{position:'fixed',left:`${right-80}px`,top:`${bottom-8}px`,width:'80px'});dispatchEvent(new Event('resize'));});
    await expect(content).toHaveAttribute('data-side','top');await expect.poll(()=>content.evaluate(element=>{const b=document.getElementById('panel-boundary')!.getBoundingClientRect(),p=element.getBoundingClientRect();return Math.max(Math.max(0,b.left)-p.left,p.right-Math.min(document.documentElement.clientWidth,b.right),Math.max(0,b.top)-p.top,p.bottom-Math.min(innerHeight,b.bottom));})).toBeLessThanOrEqual(1);
});

test('hoverable Tooltip crosses the real pointer gap and leaves keyboard focus unchanged',async({page})=>{
    const trigger=page.getByTestId('tooltip-one'),tip=page.getByTestId('tooltip-one-content');
    await page.getByTestId('hover-outside').focus();await trigger.hover();await expect(tip).toBeVisible();await expect(page.getByTestId('hover-outside')).toBeFocused();
    const a=(await trigger.boundingBox())!,b=(await tip.boundingBox())!,above=(await tip.getAttribute('data-side'))==='top';
    await page.mouse.move(a.x+a.width/2,above?a.y+1:a.y+a.height-1);
    await page.mouse.move((a.x+a.width/2+b.x+b.width/2)/2,above?(a.y+b.y+b.height)/2:(a.y+a.height+b.y)/2);await expect(tip).toBeVisible();
    await tip.hover();await expect(tip).toBeVisible();await expect(page.getByTestId('hover-outside')).toBeFocused();
    await page.mouse.move(5,5);await expect(tip).toBeHidden();await expect(page.getByTestId('hover-outside')).toBeFocused();
});

test('tooltip blur respects the shared outside cancellation and does not restore page focus',async({page})=>{
    const trigger=page.getByTestId('tooltip-one'),tip=page.getByTestId('tooltip-one-content');
    await page.getByTestId('hover-native').click();await trigger.focus();await expect(tip).toBeVisible();
    await page.getByTestId('hover-outside').focus();await expect(tip).toBeVisible();await expect(page.getByTestId('hover-outside')).toBeFocused();await expect(page.getByTestId('hover-requests')).toHaveText('1');
    await page.getByTestId('hover-native').dispatchEvent('click');await trigger.focus();await expect(tip).toBeVisible();await trigger.evaluate(element=>(element as HTMLElement).blur());
    await expect(tip).toBeHidden();await expect(page.getByTestId('hover-requests')).toHaveText('2');expect(await page.evaluate(()=>document.activeElement===document.body)).toBe(true);
});
