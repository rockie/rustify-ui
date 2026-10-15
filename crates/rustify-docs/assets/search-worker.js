// The index stays inside this Worker. Neither queries nor document text leave
// the site's origin, and word boundaries are computed only for matched fields.
const base = new URL("../", self.location.href);
const fold = value => value.replace(/[A-Z]/g, letter => letter.toLowerCase());
const han = /\p{Script=Han}/u;
const segmenters = new Map();
let pages = null;
let instance = 0;
let initialization = 0;

function segmenter(lang) {
    if (!segmenters.has(lang)) segmenters.set(lang, new Intl.Segmenter(lang, { granularity: "word" }));
    return segmenters.get(lang);
}

function localURL(value) {
    const url = new URL(value, base);
    if (url.origin !== base.origin || !url.pathname.startsWith(base.pathname)
        || url.username || url.password || url.hash) throw new Error("url");
    return url;
}

function routeURL(route) {
    if (typeof route !== "string" || !route.startsWith("/") || !route.endsWith("/")
        || route.startsWith("//") || /[\\\u0000-\u001f\u007f]/.test(route)
        || route.split("/").some(part => part === "." || part === "..")) throw new Error("route");
    return localURL(route.slice(1).split("/").map(encodeURIComponent).join("/")).href;
}

function field(text) {
    if (typeof text !== "string") throw new Error("text");
    return { text, lower: fold(text) };
}

function prepare(index) {
    if (index?.schema_version !== 1 || typeof index.ui_lang !== "string" || !index.ui_lang
        || !Array.isArray(index.pages)) throw new Error("schema");
    const seen = new Set();
    return index.pages.map(page => {
        const href = routeURL(page.route);
        if (seen.has(href) || typeof page.source_lang !== "string" || !page.source_lang
            || typeof page.is_fallback !== "boolean" || !Number.isSafeInteger(page.nav_order) || page.nav_order < 0
            || !Array.isArray(page.sections) || !page.sections.length) throw new Error("page");
        seen.add(href);
        return {
            route: page.route, href, title: field(page.title), source_lang: page.source_lang,
            is_fallback: page.is_fallback, nav_order: page.nav_order,
            sections: page.sections.map(section => {
                if (!(section.anchor === null || typeof section.anchor === "string" && section.anchor.length)
                    || !Array.isArray(section.path) || !section.path.length
                    || section.path.some(value => typeof value !== "string")) throw new Error("section");
                return { anchor: section.anchor, path: section.path, heading: field(section.path.slice(1).join(" › ")), body: field(section.text) };
            }),
        };
    });
}

function words(query) {
    return Array.from(segmenter("zh").segment(query))
        .filter(word => word.isWordLike)
        .map(word => ({ text: fold(word.segment), lang: han.test(word.segment) ? "zh" : "en" }));
}

function matches(value, tokens) {
    const segments = new Map();
    return tokens.map(token => {
        let start = value.lower.indexOf(token.text);
        while (start !== -1) {
            if (!segments.has(token.lang)) segments.set(token.lang, segmenter(token.lang).segment(value.text));
            const word = segments.get(token.lang).containing(start);
            if (word?.isWordLike && word.index === start
                && (token.lang === "zh" ? fold(word.segment) === token.text : fold(word.segment).startsWith(token.text))) {
                return [start, start + token.text.length];
            }
            start = value.lower.indexOf(token.text, start + 1);
        }
        return null;
    });
}

function merged(ranges) {
    const result = [];
    for (const range of ranges.filter(Boolean).sort((a, b) => a[0] - b[0] || a[1] - b[1])) {
        const previous = result[result.length - 1];
        if (previous && range[0] <= previous[1]) previous[1] = Math.max(previous[1], range[1]);
        else result.push([...range]);
    }
    return result;
}

function snippet(value, ranges) {
    const hits = merged(ranges);
    let start = Math.max(0, (hits[0]?.[0] || 0) - 48);
    // Keep a supplementary character intact at both edges of the window.
    if (start && /[\uDC00-\uDFFF]/.test(value.text[start])) start--;
    let end = Math.min(value.text.length, Math.max(start + 180, hits[0]?.[1] || 0));
    if (end < value.text.length && /[\uDC00-\uDFFF]/.test(value.text[end])) end++;
    return {
        snippet: value.text.slice(start, end),
        highlights: hits.filter(range => range[0] < end && range[1] > start)
            .map(range => [Math.max(0, range[0] - start), Math.min(end, range[1]) - start]),
    };
}

function compare(a, b) {
    for (let i = 0; i < a.score.length; i++) {
        if (a.score[i] !== b.score[i]) return b.score[i] - a.score[i];
    }
    return 0;
}

function search(query, offset) {
    const trimmed = query.trim();
    const tokens = words(trimmed);
    if (!tokens.length) return { total: 0, offset: 0, has_more: false, groups: [] };
    const phrase = fold(trimmed);
    const groups = [];
    for (const page of pages) {
        const title = matches(page.title, tokens);
        const titleCount = title.filter(Boolean).length;
        const sections = [];
        page.sections.forEach((section, order) => {
            // Reject non-candidates before asking a segmenter for any boundaries.
            if (tokens.some((token, i) => !title[i] && !section.heading.lower.includes(token.text) && !section.body.lower.includes(token.text))) return;
            const heading = matches(section.heading, tokens);
            const body = matches(section.body, tokens);
            if (tokens.some((_, i) => !title[i] && !heading[i] && !body[i])) return;
            const headingCount = heading.filter(Boolean).length;
            const bodyCount = body.filter(Boolean).length;
            const phraseHit = [page.title, section.heading, section.body].some(value => value.lower.includes(phrase));
            const summary = bodyCount ? snippet(section.body, body) : titleCount ? snippet(page.title, title) : snippet(section.heading, heading);
            sections.push({ ...summary, anchor: section.anchor, path: section.path,
                score: [titleCount, headingCount, bodyCount, Number(phraseHit), -page.nav_order, -order] });
        });
        if (!sections.length) continue;
        sections.sort(compare);
        const best = sections[0];
        groups.push({ route: page.route, href: page.href, title: page.title.text, title_highlights: merged(title),
            source_lang: page.source_lang, is_fallback: page.is_fallback,
            anchor: best.anchor, paths: sections.slice(0, 3).map(section => ({ anchor: section.anchor, path: section.path })),
            snippet: best.snippet, highlights: best.highlights, score: best.score });
    }
    groups.sort((a, b) => compare(a, b) || (a.route < b.route ? -1 : a.route > b.route ? 1 : 0));
    return { total: groups.length, offset, has_more: offset + 20 < groups.length,
        groups: groups.slice(offset, offset + 20).map(({ score, ...group }) => group) };
}

self.onmessage = async ({ data }) => {
    if (data?.type === "init") {
        const epoch = ++initialization;
        instance = data.instance;
        pages = null;
        let code = "load";
        try {
            if (typeof Intl.Segmenter !== "function") { code = "unsupported"; throw new Error(code); }
            const response = await fetch(localURL(data.index_url));
            if (!response.ok) throw new Error("http");
            code = "schema";
            const prepared = prepare(await response.json());
            if (epoch !== initialization) return;
            pages = prepared;
            self.postMessage({ type: "ready", instance });
        } catch {
            if (epoch === initialization) self.postMessage({ type: "error", instance, code });
        }
    } else if (data?.type === "query" && data.instance === instance) {
        try {
            if (!pages || typeof data.query !== "string" || !Number.isSafeInteger(data.id)
                || !Number.isSafeInteger(data.offset) || data.offset < 0) throw new Error("query");
            const result = search(data.query, data.offset);
            self.postMessage({ type: "results", instance, id: data.id, query: data.query, ...result });
        } catch {
            self.postMessage({ type: "error", instance, id: data.id, code: "query" });
        }
    }
};
