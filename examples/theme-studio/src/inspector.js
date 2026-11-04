const runtimeAttribute = /\[(?:data-(?:state|highlighted|disabled)|aria-(?:checked|expanded|selected|pressed|disabled|invalid|busy|current))(?:[^\]"']|"[^"]*"|'[^']*')*\]/gi;
const actionPseudo = /:(?:hover|active|focus(?:-visible|-within)?|checked|disabled|enabled|indeterminate|valid|invalid|user-valid|user-invalid|placeholder-shown)\b/gi;

function subjectSelector(selector) {
    // Negating a transient state must not turn it into :not(*) or :not().
    return selector.replace(/:not\(([^()]*)\)/g, (whole, inner) => {
        const stable = inner.replace(runtimeAttribute, "").replace(actionPseudo, "").trim();
        return stable ? `:not(${stable})` : "";
    }).replace(/::[\w-]+(?:\([^)]*\))?/g, "")
        .replace(runtimeAttribute, "").replace(actionPseudo, "")
        .replace(/:(?:is|where)\(\s*\)/g, "").trim() || "*";
}

export function createPartCache() { return { count: -1, entries: new Map() }; }

export function inspectPart(cache, element, allowed) {
    const part = element.closest("[data-rustify-part]");
    if (!part) return JSON.stringify({ tokens: [], steps: [], part: "", color: "" });
    const color = element.closest("[data-color]")?.getAttribute("data-color") || "";
    const sheets = [...document.styleSheets, ...document.adoptedStyleSheets];
    if (cache.count !== sheets.length) {
        cache.count = sheets.length;
        cache.entries.clear();
    }
    const key = [part.getAttribute("data-rustify-part"), part.className, part.getAttribute("data-variant"), color,
        element.className, element.getAttribute("data-variant")].join("\0");
    const previous = cache.entries.get(key);
    if (previous) return previous;
    const valid = new Set(allowed.split(","));
    const tokens = new Set();
    const steps = new Set();
    const visit = rules => {
        for (const rule of rules) {
            if (typeof rule.selectorText === "string") {
                const selector = subjectSelector(rule.selectorText);
                let matches = false;
                try { matches = element.matches(selector) || part.matches(selector); } catch { continue; }
                if (!matches) continue;
                for (const match of rule.cssText.matchAll(/var\(\s*--([\w-]+)/g)) {
                    if (valid.has(match[1])) tokens.add(match[1]);
                    const step = /^rustify-component-color-(a?\d+|contrast)$/.exec(match[1]);
                    if (step) steps.add(step[1]);
                }
            } else if (rule.cssRules) visit(rule.cssRules);
        }
    };
    for (const sheet of sheets) {
        // A cross-origin stylesheet may be rendered but unreadable through CSSOM.
        let rules;
        try { rules = sheet.cssRules; } catch { continue; }
        visit(rules);
    }
    const result = JSON.stringify({ tokens: [...tokens].sort(), steps: [...steps].sort(), part: part.getAttribute("data-rustify-part"), color });
    cache.entries.set(key, result);
    return result;
}
