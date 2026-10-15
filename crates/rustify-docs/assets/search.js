// Imported on demand. Importing alone owns no listeners or Worker; the
// bootstrap explicitly starts and cleans up each open dialog session.
const sessions = new WeakMap();

export function cleanup(dialog) {
    const dispose = sessions.get(dialog);
    if (dispose) { sessions.delete(dialog); dispose(); }
}

function marked(parent, text, ranges) {
    let cursor = 0;
    for (const [start, end] of ranges) {
        if (!Number.isSafeInteger(start) || !Number.isSafeInteger(end) || start < cursor || end <= start || end > text.length) throw new Error("highlight");
        parent.append(document.createTextNode(text.slice(cursor, start)));
        const mark = document.createElement("mark");
        mark.textContent = text.slice(start, end);
        parent.append(mark);
        cursor = end;
    }
    parent.append(document.createTextNode(text.slice(cursor)));
}

export function initialize(dialog, { instance, onError }) {
    cleanup(dialog);
    const input = dialog.querySelector("[data-rd-search-input]");
    const list = dialog.querySelector("[data-rd-search-results]");
    const status = dialog.querySelector("[data-rd-search-status]");
    const count = dialog.querySelector("[data-rd-search-count]");
    const more = dialog.querySelector("[data-rd-search-more]");
    const retry = dialog.querySelector("[data-rd-search-retry]");
    const labels = JSON.parse(dialog.dataset.rdSearchLabels);
    const base = new URL(document.documentElement.dataset.rdBase, location.origin);
    const workerURL = new URL(dialog.dataset.rdSearchWorker, location.href);
    if (workerURL.origin !== location.origin || !workerURL.pathname.startsWith(base.pathname)) throw new Error("worker URL");
    const worker = new Worker(workerURL, { type: "module" });
    let active = true;
    let ready = false;
    let id = 0;
    let selected = -1;
    let expectedOffset = 0;
    let watchdog;
    let groups = [];

    function label(node, key, values = {}) {
        const value = labels[key];
        node.lang = value.lang;
        node.textContent = value.text.replace(/\{(\w+)\}/g, (whole, name) => values[name] ?? whole);
    }

    function state(value, key) {
        dialog.dataset.state = value;
        status.removeAttribute("role");
        if (key) label(status, key);
        else status.textContent = "";
        retry.hidden = true;
    }

    function fail() {
        if (!active) return;
        cleanup(dialog);
        onError();
    }

    function arm() {
        clearTimeout(watchdog);
        watchdog = setTimeout(fail, 10_000);
    }

    function select(index, scroll = false) {
        selected = index;
        const options = [...list.children];
        options.forEach((option, i) => option.setAttribute("aria-selected", String(i === selected)));
        if (options[selected]) {
            input.setAttribute("aria-activedescendant", options[selected].id);
            if (scroll) options[selected].scrollIntoView({ block: "nearest", behavior: "instant" });
        } else input.removeAttribute("aria-activedescendant");
    }

    function clear() {
        groups = [];
        list.replaceChildren();
        count.textContent = "";
        more.hidden = true;
        input.setAttribute("aria-expanded", "false");
        select(-1);
    }

    function send(offset) {
        expectedOffset = offset;
        more.disabled = true;
        state("loading", "loading");
        arm();
        worker.postMessage({ type: "query", instance, id, query: input.value, offset });
    }

    function changed() {
        id++;
        clear();
        if (!ready) { state("loading", "loading"); return; }
        clearTimeout(watchdog);
        if (!input.value.trim()) { state("idle", "hint"); return; }
        if (dialog.dataset.rdSearchComposing === "true") return;
        send(0);
    }

    function link(group) {
        const href = new URL(group.href, location.href);
        if (href.origin !== base.origin || !href.pathname.startsWith(base.pathname) || href.username || href.password || href.hash) throw new Error("result URL");
        if (group.anchor !== null) href.hash = encodeURIComponent(group.anchor);
        return href.href;
    }

    function option(group, index) {
        const node = document.createElement("a");
        node.id = `rd-search-option-${instance}-${id}-${index}`;
        node.className = "rd-search-result";
        node.setAttribute("role", "option");
        node.setAttribute("aria-selected", "false");
        node.tabIndex = -1;
        node.href = link(group);
        const title = document.createElement("span");
        title.className = "rd-search-title";
        title.lang = group.source_lang;
        marked(title, group.title, group.title_highlights);
        node.append(title);
        if (group.is_fallback) {
            const badge = document.createElement("span");
            badge.className = "rd-search-badge";
            badge.lang = dialog.dataset.rdSearchLang;
            badge.textContent = dialog.dataset.rdSearchLang;
            node.append(badge);
            const fallback = document.createElement("span");
            fallback.className = "rd-search-source";
            label(fallback, "fallback", { lang: group.source_lang });
            node.append(fallback);
        }
        for (const path of group.paths) {
            if (path.path.length < 2) continue;
            const heading = document.createElement("span");
            heading.className = "rd-search-path";
            heading.lang = group.source_lang;
            heading.textContent = path.path.join(" › ");
            node.append(heading);
        }
        const summary = document.createElement("span");
        summary.className = "rd-search-snippet";
        summary.lang = group.source_lang;
        marked(summary, group.snippet, group.highlights);
        node.append(summary);
        return node;
    }

    function receive({ data }) {
        if (!active || !dialog.open || data?.instance !== instance) return;
        if (data.type === "ready") {
            ready = true;
            clearTimeout(watchdog);
            changed();
        } else if (data.type === "error" && (data.id === undefined || data.id === id)) {
            fail();
        } else if (data.type === "results" && data.id === id && data.query === input.value && data.offset === expectedOffset) {
            clearTimeout(watchdog);
            try {
                const previous = selected;
                const nodes = data.groups.map((group, i) => option(group, groups.length + i));
                groups.push(...data.groups);
                list.append(...nodes);
                input.setAttribute("aria-expanded", String(groups.length > 0));
                state(groups.length ? "results" : "empty", groups.length ? null : "empty");
                label(count, "count", { n: data.total });
                more.hidden = !data.has_more;
                more.disabled = false;
                select(previous >= 0 ? previous : groups.length ? 0 : -1);
                if (document.activeElement === more && more.hidden) input.focus();
                dialog.dataset.rdSearchResultId = String(id);
            } catch { fail(); }
        }
    }

    function keys(event) {
        if (event.isComposing || dialog.dataset.rdSearchComposing === "true") return;
        if (event.key === "ArrowDown" || event.key === "ArrowUp") {
            event.preventDefault();
            if (groups.length) select((selected + (event.key === "ArrowDown" ? 1 : -1) + groups.length) % groups.length, true);
        } else if (event.key === "Enter" && groups[selected]) {
            event.preventDefault();
            const destination = link(groups[selected]);
            dialog.close();
            location.assign(destination);
        }
    }

    function next() { if (ready && !more.disabled) send(groups.length); }
    function pointer(event) { if (event.target.closest("[role=option]")) event.preventDefault(); }
    function activate(event) {
        if (event.button === 0 && !event.metaKey && !event.ctrlKey && !event.shiftKey && !event.altKey
            && event.target.closest("[role=option]")) dialog.close();
    }
    function failedEvent(event) { event.preventDefault(); fail(); }
    sessions.set(dialog, () => {
        active = false;
        clearTimeout(watchdog);
        worker.terminate();
        input.removeEventListener("input", changed);
        input.removeEventListener("compositionend", changed);
        input.removeEventListener("keydown", keys);
        more.removeEventListener("click", next);
        list.removeEventListener("mousedown", pointer);
        list.removeEventListener("click", activate);
        worker.removeEventListener("message", receive);
        worker.removeEventListener("error", failedEvent);
        worker.removeEventListener("messageerror", failedEvent);
    });
    input.addEventListener("input", changed);
    input.addEventListener("compositionend", changed);
    input.addEventListener("keydown", keys);
    more.addEventListener("click", next);
    list.addEventListener("mousedown", pointer);
    list.addEventListener("click", activate);
    worker.addEventListener("message", receive);
    worker.addEventListener("error", failedEvent);
    worker.addEventListener("messageerror", failedEvent);
    clear();
    state("loading", "loading");
    arm();
    try { worker.postMessage({ type: "init", instance, index_url: dialog.dataset.rdSearchIndex }); }
    catch (error) { cleanup(dialog); throw error; }
}
