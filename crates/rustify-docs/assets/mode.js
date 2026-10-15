// Applies the reader's stored colour mode before the first paint. It runs
// synchronously in <head>, from a file, because the policy allows no inline
// script. The key carries the site's base path, so two sites on one origin
// keep separate choices. Storage that refuses access leaves the page on its
// system mode.
(function () {
  var root = document.documentElement;
  try {
    var mode = localStorage.getItem("rustify-docs:mode:" + (root.getAttribute("data-rd-base") || "/"));
    if (mode === "light" || mode === "dark" || mode === "system") {
      root.setAttribute("data-theme", mode);
    }
  } catch (refused) {
    // Private mode or blocked storage: the system mode stands.
  }
})();

// The input gate is installed in the synchronous head script. It remains
// alive when an island module fails to load or its WebAssembly instance dies.
(function () {
  "use strict";
  var phase = "waiting";
  var records = [];
  var failure = null;
  var handle = null;
  var deadline = null;
  var deadlineAt = null;
  var resolveParsed;
  var parsed = new Promise(function (resolve) { resolveParsed = resolve; });
  var notFoundPrepared = false;

  function prepareNotFound() {
    if (notFoundPrepared) return;
    notFoundPrepared = true;
    var root = document.documentElement;
    if (!root.hasAttribute("data-rd-not-found")) return;
    var templates = document.querySelectorAll("template[data-rd-not-found-prefix]");
    var path;
    try { path = decodeURIComponent(location.pathname); } catch (malformed) { path = ""; }
    if (path.indexOf("\0") !== -1 || path.indexOf("\\") !== -1) path = "";
    var base = root.getAttribute("data-rd-base") || "/";
    var rest = path.indexOf(base) === 0 ? path.slice(base.length) : "";
    if (rest.split("/").some(function (segment) { return segment === "." || segment === ".."; })) rest = "";
    var prefix = rest.split("/")[0];
    var selected = null;
    templates.forEach(function (template) {
      if (template.getAttribute("data-rd-not-found-prefix") === prefix) selected = template;
    });
    if (selected) {
      var content = selected.content.cloneNode(true);
      var dev = document.querySelector("script[data-labels]");
      // Keep the original parser-inserted module; template scripts never run.
      Array.prototype.slice.call(document.body.children).forEach(function (node) {
        if (node.tagName === "SCRIPT") content.appendChild(node);
      });
      root.lang = selected.getAttribute("data-rd-not-found-lang");
      document.title = selected.getAttribute("data-rd-not-found-title");
      if (dev) {
        dev.setAttribute("data-lang", root.lang);
        dev.setAttribute("data-labels", selected.getAttribute("data-rd-dev-labels") || "{}");
        dev.setAttribute("data-label-langs", selected.getAttribute("data-rd-dev-label-langs") || "{}");
      }
      document.body.replaceChildren(content);
    } else {
      templates.forEach(function (template) { template.remove(); });
    }
  }

  function blockFor(event) {
    var target = event.target;
    var element = target instanceof Element ? target : target && target.parentElement;
    var canvas = element && element.closest("fieldset[data-rd-island-controls]");
    return canvas && canvas.closest("[data-rustify-island]");
  }

  function guard(event) {
    var block = blockFor(event);
    if (!block || block.getAttribute("data-rd-island-state") === "ready") return;
    if (event.cancelable) event.preventDefault();
    event.stopImmediatePropagation();
  }

  ["click", "auxclick", "dblclick", "pointerdown", "pointerup", "mousedown", "mouseup", "touchstart", "keydown", "keyup", "beforeinput", "input", "change", "submit", "focusin"].forEach(function (name) {
    document.addEventListener(name, guard, { capture: true, passive: false });
  });

  function mark(record, state) {
    record.block.setAttribute("data-rd-island-state", state);
    if (record.canvas) {
      record.canvas.inert = state !== "ready";
      record.canvas.disabled = state === "failed";
      record.canvas.setAttribute("aria-busy", String(state === "loading"));
    }
    var status = record.block.querySelector("[data-rd-island-status]");
    if (!status) return;
    status.setAttribute("role", state === "failed" ? "alert" : "status");
    status.setAttribute("aria-live", state === "failed" ? "assertive" : "polite");
    var loading = status.querySelector("[data-rd-island-loading]");
    var error = status.querySelector("[data-rd-island-error]");
    var reload = status.querySelector("[data-rd-island-reload]");
    if (loading) loading.hidden = state !== "loading";
    if (error) error.setAttribute("aria-hidden", String(state !== "failed"));
    if (reload) reload.setAttribute("aria-hidden", String(state !== "failed"));
  }

  function restore(record) {
    // Native host release has already stopped callbacks and cleared CSSOM.
    // Source and shell nodes are siblings of this canvas and stay untouched.
    if (record.canvas && record.original) record.canvas.replaceChildren(record.original.cloneNode(true));
    mark(record, "failed");
  }

  function release() {
    if (handle) handle.hooks.runtime.release();
  }

  function fail(error) {
    if (phase === "failed") return false;
    failure = error;
    phase = "failed";
    clearTimeout(deadline);
    release();
    records.forEach(restore);
    console.error("Interactive examples failed", {
      keys: records.map(function (record) { return record.key; }),
      error: String(error),
    });
    return true;
  }

  function prepare() {
    if (records.length || phase === "empty" || phase === "loading" || phase === "ready") return;
    prepareNotFound();
    var malformed = false;
    document.querySelectorAll("[data-rustify-island]").forEach(function (block) {
      var canvas = block.querySelector("fieldset[data-rd-island-controls]");
      var root = canvas && canvas.firstElementChild;
      if (!canvas || !root) malformed = true;
      var record = { block: block, canvas: canvas, key: block.getAttribute("data-rustify-island"), original: root && root.cloneNode(true) };
      records.push(record);
      var reload = block.querySelector("[data-rd-island-reload]");
      if (reload) reload.addEventListener("click", function () { window.location.reload(); });
    });
    if (!records.length) {
      if (phase !== "failed") phase = "empty";
      resolveParsed([]);
      return;
    }
    if (malformed && fail(new Error("island static canvas is missing"))) {
      resolveParsed(records.map(function (record) { return record.block; }));
      return;
    }
    if (phase === "failed") {
      records.forEach(restore);
    } else {
      phase = "loading";
      records.forEach(function (record) { mark(record, "loading"); });
      deadlineAt = performance.now() + 30000;
      deadline = setTimeout(function () { fail(new Error("island startup deadline exceeded")); }, 30000);
    }
    resolveParsed(records.map(function (record) { return record.block; }));
  }

  function terminal() {
    if (phase === "loading" && performance.now() >= deadlineAt) fail(new Error("island startup deadline exceeded"));
    return phase === "failed";
  }

  window.__rustify_docs_islands = {
    parsed: parsed,
    state: function () { return phase; },
    terminal: terminal,
    fail: fail,
    bind: function (started) {
      handle = started;
      if (terminal()) { release(); return false; }
      return true;
    },
    complete: function () {
      if (terminal() || phase !== "loading") return false;
      clearTimeout(deadline);
      phase = "ready";
      records.forEach(function (record) { mark(record, "ready"); });
      return true;
    },
    stats: function () { return handle ? handle.hooks.runtime.stats() : null; },
    failure: function () { return failure === null ? null : String(failure); },
  };

  window.addEventListener("error", function (event) {
    var script = event.target;
    if (script instanceof HTMLScriptElement && /(?:^|\/)islands\.js(?:[?#]|$)/.test(script.src)) {
      fail(new Error("island startup module could not load"));
    }
  }, true);

  // Deferred module downloads can delay DOMContentLoaded. The interactive
  // parser boundary gives us complete SSR nodes and an independent deadline;
  // DOMContentLoaded keeps the same path for the normal completed document.
  document.addEventListener("readystatechange", function () {
    if (document.readyState !== "loading") prepare();
  });
  document.addEventListener("DOMContentLoaded", prepare);
  if (document.readyState !== "loading") prepare();
})();
