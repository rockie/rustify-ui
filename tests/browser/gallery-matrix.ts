import { expect, Locator, Page } from '@playwright/test';

const floatingParts: Record<string, string> = {
    Dialog: 'dialog-content', AlertDialog: 'alert-dialog-content', Popover: 'popover-content',
    HoverCard: 'hover-card-content', Tooltip: 'tooltip-content', DropdownMenu: 'menu-content',
    ContextMenu: 'menu-content', Menubar: 'menu-content', Select: 'select-content', Toast: 'toast-root',
};

export async function openSurface(page: Page, sample: Locator, name: string) {
    if (name === 'NavigationMenu') {
        const trigger = sample.locator('[data-rustify-part="navigation-menu-trigger"]');
        if (await trigger.getAttribute('aria-expanded') !== 'true') await trigger.press('Enter');
        await expect(sample.locator('[data-rustify-part="navigation-menu-content"]')).toBeVisible();
        await trigger.press('ArrowDown');
        return sample.locator('.gallery-demo');
    }
    if (name === 'ScrollArea') {
        await sample.locator('[data-rustify-part="scroll-area-root"]').hover();
        await expect(sample.locator('[data-rustify-part="scroll-area-scrollbar"]').first()).toBeVisible();
    }
    const part = floatingParts[name];
    if (!part) return sample.locator('.gallery-demo');
    const surface = page.getByTestId('studio-preview').locator(`[data-rustify-part="${part}"]:visible`).last();
    if (await surface.count()) return surface;
    if (name === 'ContextMenu') await sample.locator('[data-rustify-part="context-menu-trigger"]').click({ button: 'right' });
    else if (name === 'HoverCard' || name === 'Tooltip') await sample.locator(`[data-rustify-part="${name === 'HoverCard' ? 'hover-card' : 'tooltip'}-trigger"]`).focus();
    else if (name === 'Toast') await sample.getByRole('button', { name: 'Show notification' }).click();
    else if (name === 'Select') await sample.locator('[data-rustify-part="select-trigger"]').click();
    else await sample.locator('[aria-haspopup]').first().click();
    await expect(surface).toBeVisible();
    return surface;
}

export async function closeSurface(page: Page, name: string) {
    if (name === 'NavigationMenu') {
        await page.keyboard.press('Escape');
        await expect(page.getByTestId('studio-preview').locator('[data-rustify-part="navigation-menu-content"]')).toBeHidden();
        return;
    }
    if (!floatingParts[name]) return;
    if (!await page.getByTestId('studio-preview').locator(`[data-rustify-part="${floatingParts[name]}"]:visible`).count()) return;
    if (name === 'Toast') {
        await page.getByTestId('studio-preview').locator('[data-rustify-part="toast-close"]:visible').click();
    } else if (name === 'AlertDialog') {
        await page.getByTestId('studio-preview').getByRole('button', { name: 'Keep draft', exact: true }).click();
    } else {
        await page.keyboard.press('Escape');
        if (name === 'HoverCard' || name === 'Tooltip') await page.getByTestId('renderer-dom').focus();
    }
    await expect(page.getByTestId('studio-preview').locator(`[data-rustify-part="${floatingParts[name]}"]:visible`)).toHaveCount(0);
}

export async function appearance(root: Locator) {
    return root.evaluate(async root => {
        await new Promise<void>(resolve => requestAnimationFrame(() => resolve()));
        await Promise.all(root.getAnimations({ subtree: true })
            .filter(animation => animation.effect?.getComputedTiming().iterations !== Infinity)
            .map(animation => animation.finished.catch(() => {})));
        const canvas = document.createElement('canvas'); canvas.width = canvas.height = 1;
        const ctx = canvas.getContext('2d')!;
        const rgba = (value: string) => {
            ctx.clearRect(0, 0, 1, 1); ctx.fillStyle = value; ctx.fillRect(0, 0, 1, 1);
            return [...ctx.getImageData(0, 0, 1, 1).data].map(value => value / 255);
        };
        const over = (a: number[], b: number[]) => [0, 1, 2].map(i => a[i] * a[3] + b[i] * (1 - a[3])).concat(1);
        const background = (element: Element) => {
            const chain: Element[] = [];
            for (let node: Element | null = element; node; node = node.parentElement) chain.unshift(node);
            return chain.reduce((back, node) => {
                const css = getComputedStyle(node);
                let color = over(rgba(css.backgroundColor), back);
                // The component skins use a constant-color top gradient to composite translucent surfaces.
                const stops = css.backgroundImage.match(/rgba?\([^)]+\)|color\([^)]+\)/g);
                if (stops?.length && stops.every(stop => stop === stops[0])) color = over(rgba(stops[0]), color);
                return color;
            }, [1, 1, 1, 1]);
        };
        const luminance = (color: number[]) => color.slice(0, 3).map(c => c <= .04045 ? c / 12.92 : ((c + .055) / 1.055) ** 2.4).reduce((sum, c, i) => sum + c * [.2126, .7152, .0722][i], 0);
        const ratio = (a: number[], b: number[]) => (Math.max(luminance(a), luminance(b)) + .05) / (Math.min(luminance(a), luminance(b)) + .05);
        const nodes = [root, ...root.querySelectorAll('*')].filter(node => {
            const css = getComputedStyle(node), box = node.getBoundingClientRect();
            return box.width > 0 && box.height > 0 && css.display !== 'none' && css.visibility === 'visible'
                && !node.closest('.gallery-controls,.gallery-structure,output,[hidden],[inert]')
                && !node.closest('.gallery-control')?.querySelector('select,input[type=checkbox]');
        });
        const visual: unknown[] = [], geometry: unknown[] = [], text: { part: string, text: string, ratio: number, foreground: string, background: number[], reference?: { rule: string, matches: boolean } }[] = [];
        const edges: { part: string, kind: string, ratio: number }[] = [];
        for (const node of nodes) {
            const css = getComputedStyle(node), part = node.getAttribute('data-rustify-part') || node.tagName.toLowerCase();
            const paint = (style: CSSStyleDeclaration) => [style.backgroundColor, style.backgroundImage, style.borderColor, style.borderStyle, style.borderWidth,
                style.color, style.boxShadow, style.outlineStyle, style.outlineWidth, style.outlineColor, style.opacity, style.textDecoration, style.animationName];
            visual.push([part, paint(css), node instanceof SVGElement ? node.getAttribute("d") : null, ...['::before', '::after'].map(pseudo => {
                const style = getComputedStyle(node, pseudo);
                return style.content === 'none' ? null : paint(style);
            })]);
            geometry.push([part, css.width, css.height, css.maxWidth, css.padding, css.margin, css.gap, css.fontSize, css.lineHeight]);
            if (!node.closest(':disabled,[data-disabled],[aria-disabled="true"],[data-loading="true"]')) {
                const back = background(node.parentElement || node);
                if (node.matches('.rustify-choice-native:not(:checked):not(:indeterminate)') && !node.closest('[data-variant="soft"]')) {
                    edges.push({ part, kind: 'unchecked boundary', ratio: ratio(over(rgba(css.borderTopColor), back), back) });
                }
                if (node.matches('.rustify-field:not([data-variant="soft"])')) {
                    edges.push({ part, kind: 'field boundary', ratio: ratio(over(rgba(css.getPropertyValue('--rustify-field-edge')), back), back) });
                }
                if (node.matches(':focus-visible,:focus-within') && css.outlineStyle !== 'none' && parseFloat(css.outlineWidth) >= 1) {
                    edges.push({ part, kind: 'focus outline', ratio: ratio(over(rgba(css.outlineColor), back), back) });
                } else if (node.matches(':focus-visible')) {
                    const rings = [...css.boxShadow.matchAll(/(rgba?\([^)]+\)|color\([^)]+\)) 0px 0px 0px ([\d.]+)px/g)]
                        .filter(match => Number(match[2]) > 0 && rgba(match[1])[3] > 0)
                        .sort((a, b) => Number(b[2]) - Number(a[2]));
                    if (rings.length) edges.push({ part, kind: 'focus ring', ratio: ratio(over(rgba(rings[0][1]), back), back) });
                }
            }
            const direct = node instanceof HTMLTextAreaElement || (node instanceof HTMLInputElement && !['checkbox', 'radio', 'color', 'hidden'].includes(node.type))
                ? node.value
                : [...node.childNodes].filter(n => n.nodeType === Node.TEXT_NODE).map(n => n.textContent).join('').trim();
            if (!direct || node.closest('[aria-hidden="true"],:disabled,[data-disabled],[aria-disabled="true"],[data-loading="true"]')) continue;
            const back = background(node), front = rgba(css.color);
            front[3] *= Number(css.opacity);
            const component = node.closest('.rustify-button,.rustify-avatar,.rustify-badge,.rustify-callout,.rustify-code,.rustify-menu,.rustify-select-content');
            let reference: { rule: string, matches: boolean } | undefined;
            if (component) {
                const variant = component.getAttribute('data-variant');
                const item = node.closest('[data-rustify-part="menu-item"],[data-rustify-part="select-item"]');
                const high = component.hasAttribute('data-high-contrast') && !item?.hasAttribute('data-color');
                const solid = variant === 'solid' && (component.matches('.rustify-button') || item?.hasAttribute('data-highlighted'));
                const soft = variant === 'soft' && component.matches('.rustify-avatar,.rustify-badge,.rustify-callout,.rustify-code');
                const painted = item || node.closest('[data-rustify-part="avatar-fallback"]') || component;
                const palette = getComputedStyle(painted);
                if (!high && (solid || soft) && palette.getPropertyValue('--rustify-component-color-9').trim()) {
                    const foreground = palette.getPropertyValue(`--rustify-component-color-${solid ? 'contrast' : 'a11'}`);
                    const fill = palette.getPropertyValue(`--rustify-component-color-${solid ? '9' : 'a3'}`);
                    const expectedBack = over(rgba(fill), background(painted.parentElement!));
                    const equal = (a: number[], b: number[]) => a.every((value, index) => Math.abs(value - b[index]) <= 1 / 255);
                    reference = {
                        rule: solid ? 'Radix solid: accent-9 / accent-contrast' : 'Radix soft: accent-a3 / accent-a11',
                        matches: equal(rgba(css.color), rgba(foreground)) && equal(back, expectedBack),
                    };
                }
            }
            text.push({ part, text: direct.slice(0, 70), ratio: ratio(over(front, back), back), foreground: css.color, background: back, reference });
        }
        return { visual: JSON.stringify(visual), geometry: JSON.stringify(geometry), text, edges };
    });
}
