// The default theme's enhancements. Everything they act on already works
// without them: the mode follows the system, every code block is readable
// and selectable, every heading and page is a link. A failure here hides
// nothing.
(function (enhance) {
  var script = document.currentScript;
  var coordinator = window.__rustify_docs_islands;
  var start = function () { enhance(script); };
  if (document.documentElement.hasAttribute("data-rd-not-found") && coordinator) {
    coordinator.parsed.then(start);
  } else {
    start();
  }
})(function (script) {
  "use strict";
  var root = document.documentElement;
  var base = root.getAttribute("data-rd-base") || "/";
  var status = document.getElementById("rd-status");

  // A landing page names the query parameters it carries (gclid, utm_*…):
  // each one the address has goes into the hidden fields, links and frames
  // the page marks, its hero's actions among them. The address's value
  // wins over a field's own; a link that has the parameter keeps it.
  // Nothing is stored or sent, and without this the page works the same,
  // only its visits are harder to attribute.
  (function carry() {
    var names = (root.getAttribute("data-rd-carry") || "").split(" ").filter(Boolean);
    if (!names.length) return;
    var params = new URLSearchParams(location.search);
    var present = names.filter(function (name) { return params.get(name); });
    if (!present.length) return;
    var fields = document.querySelectorAll('input[type="hidden"][data-rd-carry]');
    for (var i = 0; i < fields.length; i++) {
      if (present.indexOf(fields[i].name) >= 0) fields[i].value = params.get(fields[i].name);
    }
    var targets = document.querySelectorAll("a[data-rd-carry][href], iframe[data-rd-carry][src]");
    for (var j = 0; j < targets.length; j++) {
      var attribute = targets[j].tagName === "IFRAME" ? "src" : "href";
      var url;
      try {
        url = new URL(targets[j].getAttribute(attribute), location.href);
      } catch (invalid) {
        continue;
      }
      if (url.protocol !== "http:" && url.protocol !== "https:") continue;
      var added = present.filter(function (name) { return !url.searchParams.has(name); });
      if (!added.length) continue;
      added.forEach(function (name) { url.searchParams.set(name, params.get(name)); });
      if (attribute === "href") {
        targets[j].setAttribute("href", url.href);
        continue;
      }
      // A frame is replaced by a copy with the new address rather than
      // given it: a changed src adds a history entry once the frame has
      // loaded, and Chromium can drop it while a lazy load is starting.
      // The copy loads once, as its first load, under its own loading.
      var frame = targets[j].cloneNode(true);
      frame.setAttribute("src", url.href);
      targets[j].parentNode.replaceChild(frame, targets[j]);
    }
  })();

  function stored(key) {
    try {
      return localStorage.getItem(key);
    } catch (refused) {
      return null;
    }
  }

  function store(key, value) {
    try {
      localStorage.setItem(key, value);
    } catch (refused) {
      // Private mode or blocked storage: the choice holds for this page.
    }
  }

  function announce(text, lang) {
    if (!status || !text) return;
    status.textContent = "";
    // A changed text node is what a live region reads; clearing first makes
    // the same message announce twice in a row.
    setTimeout(function () {
      if (lang) status.lang = lang;
      status.textContent = text;
    }, 50);
  }

  function label(name, button) {
    return (button && button.getAttribute("data-" + name)) || (status && status.getAttribute("data-" + name)) || "";
  }

  function labelLang(name, button) {
    return (button && button.getAttribute("data-" + name + "-lang")) ||
      (status && status.getAttribute("data-" + name + "-lang")) || root.lang;
  }

  document.querySelectorAll("[data-rd-language-switch]").forEach(function (menu) {
    menu.addEventListener("keydown", function (event) {
      if (event.key !== "Escape" || !menu.open) return;
      event.preventDefault();
      menu.open = false;
      menu.querySelector("summary").focus();
    });
    menu.querySelectorAll("a[hreflang]").forEach(function (link) {
      link.addEventListener("click", function () {
        var anchor = location.hash.slice(1);
        try { anchor = decodeURIComponent(anchor); } catch (malformed) { /* Literal percent in an id. */ }
        var target = new URL(link.href);
        var anchors = JSON.parse(link.getAttribute("data-rd-language-anchors") || "[]");
        target.hash = anchor && anchors.indexOf(anchor) !== -1 ? location.hash : "";
        link.href = target.href;
      });
    });
  });

  // The colour mode switch.
  var modeKey = "rustify-docs:mode:" + base;
  var modeSwitch = document.querySelector("[data-rd-mode-switch]");
  if (modeSwitch) {
    var modeButtons = modeSwitch.querySelectorAll("[data-rd-mode]");
    var markMode = function () {
      var current = root.getAttribute("data-theme") || "system";
      modeButtons.forEach(function (button) {
        button.setAttribute("aria-pressed", String(button.getAttribute("data-rd-mode") === current));
      });
    };
    modeButtons.forEach(function (button) {
      button.addEventListener("click", function () {
        var mode = button.getAttribute("data-rd-mode");
        root.setAttribute("data-theme", mode);
        store(modeKey, mode);
        markMode();
      });
    });
    window.addEventListener("storage", function (event) {
      if (event.key === modeKey && /^(system|light|dark)$/.test(event.newValue || "")) {
        root.setAttribute("data-theme", event.newValue);
        markMode();
      }
    });
    markMode();
    modeSwitch.hidden = false;
  }

  // Wrapped controls change the space section links must leave above them.
  // A CSSOM sheet updates that space without an inline style under strict CSP.
  var headerBar = document.querySelector(".rd-header-bar");
  if (headerBar) {
    var header = headerBar.parentElement;
    var headerSheet = new CSSStyleSheet();
    headerSheet.insertRule(":root {}");
    document.adoptedStyleSheets = document.adoptedStyleSheets.concat(headerSheet);
    function sizeHeader() {
      headerSheet.cssRules[0].style.setProperty("--rd-header", header.getBoundingClientRect().height + "px");
    }
    sizeHeader();
    new ResizeObserver(sizeHeader).observe(header);
  }

  // Copying a code block's raw text, never its rendered lines. Where the
  // browser refuses (an insecure origin, a denied permission), the button
  // says so and the code is selected for copying by hand.
  function selectCode(figure) {
    var code = figure && figure.querySelector(".rd-code-pre code");
    var selection = window.getSelection && window.getSelection();
    if (!code || !selection) return;
    var range = document.createRange();
    range.selectNodeContents(code);
    selection.removeAllRanges();
    selection.addRange(range);
  }

  function showCopy(button, state) {
    var text = button.querySelector(".rd-copy-label");
    if (!button.hasAttribute("data-rd-copy-text") && text) {
      button.setAttribute("data-rd-copy-text", text.textContent);
    }
    clearTimeout(button.rdCopyTimer);
    button.setAttribute("data-state", state);
    var key = state === "copied" ? "copied" : "copy-failed-short";
    if (text) text.textContent = label(key, button);
    button.lang = labelLang(key, button);
    button.rdCopyTimer = setTimeout(function () {
      button.removeAttribute("data-state");
      if (text) text.textContent = button.getAttribute("data-rd-copy-text");
      button.lang = labelLang("copy", button);
    }, state === "copied" ? 2000 : 4000);
  }

  function copy(button, figure) {
    var raw = figure && figure.querySelector("template.rd-code-raw");
    if (!raw) return;
    var failed = function () {
      showCopy(button, "failed");
      announce(label("copy-failed", button), labelLang("copy-failed", button));
      selectCode(figure);
    };
    if (!navigator.clipboard || !window.isSecureContext) {
      failed();
      return;
    }
    navigator.clipboard.writeText(raw.content.textContent).then(function () {
      showCopy(button, "copied");
      announce(label("copied", button), labelLang("copied", button));
    }, failed);
  }

  document.querySelectorAll(".rd-code [data-rd-copy]").forEach(function (button) {
    var figure = button.closest(".rd-code");
    if (!figure || !figure.querySelector("template.rd-code-raw")) return;
    button.hidden = false;
    button.addEventListener("click", function () {
      copy(button, figure);
    });
  });

  // Code groups: one panel under a row of tabs. Choosing a tab switches
  // every group of that name on the page, and is remembered for the next
  // page under this base.
  var groups = document.querySelectorAll(".rd-code-group[data-rd-group]");
  var groupKey = function (name) {
    return "rustify-docs:group:" + base + ":" + name;
  };
  var sequence = 0;

  function selectTab(group, tab, focus) {
    var tabs = group.querySelectorAll('[role="tab"]');
    tabs.forEach(function (other) {
      var selected = other === tab;
      other.setAttribute("aria-selected", String(selected));
      other.tabIndex = selected ? 0 : -1;
      var panel = document.getElementById(other.getAttribute("aria-controls"));
      if (panel) panel.hidden = !selected;
    });
    if (focus) tab.focus();
  }

  function choose(name, text, focus, source) {
    groups.forEach(function (group) {
      if (group.getAttribute("data-rd-group") !== name) return;
      group.querySelectorAll('[role="tab"]').forEach(function (tab) {
        if (tab.textContent === text) selectTab(group, tab, focus && group === source);
      });
    });
  }

  groups.forEach(function (group) {
    // The bar holds the tab list and, beside it rather than in it, the
    // group's copy button: a tab list owns nothing but tabs.
    var bar = group.querySelector(".rd-code-tabs");
    var tablist = group.querySelector('[role="tablist"]');
    var tabs = group.querySelectorAll('[role="tab"]');
    if (!bar || !tablist || !tabs.length) return;
    var name = group.getAttribute("data-rd-group");
    tabs.forEach(function (tab) {
      var panel = group.querySelector('[data-rd-tab-panel="' + tab.getAttribute("data-rd-tab") + '"]');
      if (!panel) return;
      sequence += 1;
      tab.id = "rd-tab-" + sequence;
      panel.id = "rd-panel-" + sequence;
      tab.setAttribute("aria-controls", panel.id);
      panel.setAttribute("role", "tabpanel");
      panel.setAttribute("aria-labelledby", tab.id);
      tab.addEventListener("click", function () {
        choose(name, tab.textContent, false, group);
        store(groupKey(name), tab.textContent);
      });
    });
    tablist.addEventListener("keydown", function (event) {
      var list = Array.prototype.slice.call(tabs);
      var index = list.indexOf(document.activeElement);
      if (index < 0) return;
      var next = { ArrowRight: index + 1, ArrowLeft: index - 1, Home: 0, End: list.length - 1 }[event.key];
      if (next === undefined) return;
      event.preventDefault();
      var tab = list[(next + list.length) % list.length];
      choose(name, tab.textContent, true, group);
      store(groupKey(name), tab.textContent);
    });
    // One copy button for the group, copying the panel on show.
    var template = group.querySelector("[data-rd-copy]");
    if (template) {
      var button = template.cloneNode(true);
      button.hidden = false;
      button.addEventListener("click", function () {
        var shown = group.querySelector(".rd-code:not([hidden])");
        copy(button, shown);
      });
      bar.appendChild(button);
    }
    group.setAttribute("data-rd-enhanced", "");
    bar.hidden = false;
    var remembered = stored(groupKey(name));
    var initial = Array.prototype.filter.call(tabs, function (tab) {
      return tab.textContent === remembered;
    })[0];
    selectTab(group, initial || tabs[0], false);
  });

  window.addEventListener("storage", function (event) {
    var prefix = "rustify-docs:group:" + base + ":";
    if (event.key && event.key.indexOf(prefix) === 0 && event.newValue) {
      choose(event.key.slice(prefix.length), event.newValue, false, null);
    }
  });

  // Static content tabs own only their nearest builtin. Nested code groups
  // and other static tab sets retain their own input and panel selection.
  var staticSequence = 0;
  document.querySelectorAll("[data-rd-static-tabs]").forEach(function (group) {
    var owned = function (selector) {
      return Array.prototype.filter.call(group.querySelectorAll(selector), function (node) {
        return node.closest("[data-rd-static-tabs]") === group;
      });
    };
    var bar = owned("[data-rd-static-tabbar]")[0];
    var tabs = owned("[data-rd-static-tab]");
    var panels = owned("[data-rd-static-panel]");
    if (!bar || !tabs.length || tabs.length !== panels.length) return;
    var byIndex = new Map();
    panels.forEach(function (panel) { byIndex.set(panel.getAttribute("data-rd-static-panel"), panel); });
    var paired = tabs.map(function (tab) { return byIndex.get(tab.getAttribute("data-rd-static-tab")); });
    if (paired.some(function (panel) { return !panel; })) return;

    function select(index, focus) {
      tabs.forEach(function (tab, at) {
        var active = at === index;
        tab.setAttribute("aria-selected", String(active));
        tab.tabIndex = active ? 0 : -1;
        paired[at].hidden = !active;
      });
      if (focus) tabs[index].focus();
    }

    bar.setAttribute("role", "tablist");
    tabs.forEach(function (tab, index) {
      staticSequence += 1;
      tab.id = "rd-static-tab-" + staticSequence;
      paired[index].id = "rd-static-panel-" + staticSequence;
      tab.setAttribute("role", "tab");
      tab.setAttribute("aria-controls", paired[index].id);
      paired[index].setAttribute("role", "tabpanel");
      paired[index].setAttribute("aria-labelledby", tab.id);
      paired[index].tabIndex = 0;
      tab.addEventListener("click", function () { select(index, false); });
    });
    bar.addEventListener("keydown", function (event) {
      var index = tabs.indexOf(document.activeElement);
      if (index < 0) return;
      var next = { ArrowRight: index + 1, ArrowLeft: index - 1, Home: 0, End: tabs.length - 1 }[event.key];
      if (next === undefined) return;
      event.preventDefault();
      select((next + tabs.length) % tabs.length, true);
    });
    group.setAttribute("data-rd-static-enhanced", "");
    bar.hidden = false;
    select(0, false);
  });

  // The table of contents marks the section the reader is in: the last
  // heading above the line under the header, the first one before any has
  // reached it, and the last one once the page can scroll no further. Both
  // copies (the column and the panel) follow.
  var entries = [];
  // The fragment is the id percent-encoded; one written as is (a `%` with
  // no escape after it) is taken whole rather than stopping the script.
  var fragmentId = function (href) {
    var raw = href.slice(1);
    try {
      return decodeURIComponent(raw);
    } catch (malformed) {
      return raw;
    }
  };
  document.querySelectorAll("[data-rd-toc-link]").forEach(function (link) {
    var id = fragmentId(link.getAttribute("href"));
    var entry = entries.filter(function (known) {
      return known.heading.id === id;
    })[0];
    if (!entry) {
      var heading = document.getElementById(id);
      if (!heading) return;
      entry = { heading: heading, links: [] };
      entries.push(entry);
    }
    entry.links.push(link);
  });
  if (entries.length) {
    var current = null;
    var queued = false;
    var mark = function (entry, on) {
      entry.links.forEach(function (link) {
        if (on) link.setAttribute("aria-current", "location");
        else link.removeAttribute("aria-current");
      });
    };
    var spy = function () {
      queued = false;
      var line = 96;
      var active = null;
      var bottom = window.innerHeight + window.scrollY >= document.documentElement.scrollHeight - 2;
      if (bottom && window.scrollY > 0) {
        active = entries[entries.length - 1];
      } else {
        for (var i = 0; i < entries.length; i++) {
          if (entries[i].heading.getBoundingClientRect().top <= line) active = entries[i];
          else break;
        }
        // Above the first heading the reader is about to read it.
        if (!active) active = entries[0];
      }
      if (active === current) return;
      if (current) mark(current, false);
      current = active;
      if (current) mark(current, true);
    };
    var queue = function () {
      if (!queued) {
        queued = true;
        requestAnimationFrame(spy);
      }
    };
    window.addEventListener("scroll", queue, { passive: true });
    window.addEventListener("resize", queue);
    spy();
  }

  // On a narrow screen the menu and the contents are panels in the flow:
  // following a link inside one closes it, and Escape closes it and returns
  // to its toggle.
  document.querySelectorAll("details.rd-sidebar-narrow, details.rd-toc-narrow").forEach(function (panel) {
    var summary = panel.querySelector("summary");
    panel.addEventListener("click", function (event) {
      if (event.target.closest("a")) panel.open = false;
    });
    panel.addEventListener("keydown", function (event) {
      if (event.key === "Escape" && panel.open) {
        panel.open = false;
        summary.focus();
      }
    });
  });

  // The bootstrap stays usable even when the lazy module cannot be imported.
  var searchDialog = document.querySelector("[data-rd-search]");
  var searchOpen = document.querySelector("[data-rd-search-open]");
  if (searchDialog && searchOpen && typeof searchDialog.showModal === "function") {
    var searchInput = searchDialog.querySelector("[data-rd-search-input]");
    var searchStatus = searchDialog.querySelector("[data-rd-search-status]");
    var searchRetry = searchDialog.querySelector("[data-rd-search-retry]");
    var searchLabels = JSON.parse(searchDialog.getAttribute("data-rd-search-labels"));
    var searchModule = null;
    var searchPending = null;
    var searchInstance = 0;
    var searchAttempt = 0;
    var searchTimer;
    var searchTrigger;

    function searchMessage(key) {
      searchStatus.textContent = searchLabels[key].text;
      searchStatus.lang = searchLabels[key].lang;
    }

    function searchLoading() {
      searchDialog.setAttribute("data-state", "loading");
      searchStatus.removeAttribute("role");
      searchMessage("loading");
      searchRetry.hidden = true;
      searchDialog.querySelector("[data-rd-search-more]").hidden = true;
      searchDialog.querySelector("[data-rd-search-results]").replaceChildren();
      searchDialog.querySelector("[data-rd-search-count]").textContent = "";
      searchInput.setAttribute("aria-expanded", "false");
      searchInput.removeAttribute("aria-activedescendant");
    }

    function searchFailure(expected) {
      if (!searchDialog.open || expected !== searchInstance) return;
      clearTimeout(searchTimer);
      searchPending = null;
      if (searchModule) searchModule.cleanup(searchDialog);
      searchLoading();
      searchDialog.setAttribute("data-state", "error");
      searchStatus.setAttribute("role", "alert");
      searchMessage("failed");
      searchRetry.hidden = false;
    }

    function searchStart() {
      var expected = ++searchInstance;
      searchLoading();
      function initialize(module) {
        if (!searchDialog.open || expected !== searchInstance) return;
        try {
          module.initialize(searchDialog, { instance: expected, onError: function () { searchFailure(expected); } });
        } catch (failed) { searchFailure(expected); }
      }
      if (searchModule) { initialize(searchModule); return; }
      var moduleURL = new URL(searchDialog.getAttribute("data-rd-search-module"), location.href);
      if (moduleURL.origin !== location.origin || !moduleURL.pathname.startsWith(base)) { searchFailure(expected); return; }
      if (searchAttempt) moduleURL.searchParams.set("rd_search_attempt", String(searchAttempt));
      var request = { instance: expected, attempt: searchAttempt };
      searchPending = request;
      searchTimer = setTimeout(function () { if (searchPending === request) searchFailure(expected); }, 10000);
      import(moduleURL.href).then(function (module) {
        if (searchPending !== request || request.attempt !== searchAttempt || expected !== searchInstance || !searchDialog.open) return;
        if (typeof module.initialize !== "function" || typeof module.cleanup !== "function") { searchFailure(expected); return; }
        clearTimeout(searchTimer);
        searchPending = null;
        searchModule = module;
        initialize(module);
      }, function () {
        if (searchPending === request) searchFailure(expected);
      });
    }

    function searchShow(trigger) {
      if (searchDialog.open) { searchInput.focus(); return; }
      // Native close events are queued; a new open must settle the old session
      // before that event can accidentally dispose the new Worker or import.
      searchEnd(false);
      searchTrigger = trigger;
      searchDialog.showModal();
      searchInput.focus({ preventScroll: true });
      searchStart();
    }

    searchOpen.disabled = false;
    searchOpen.addEventListener("click", function () { searchShow(searchOpen); });
    document.addEventListener("keydown", function (event) {
      if ((event.metaKey || event.ctrlKey) && !event.altKey && event.key.toLowerCase() === "k") {
        event.preventDefault();
        searchShow(document.activeElement);
      }
    });
    searchRetry.addEventListener("click", function () {
      if (searchDialog.getAttribute("data-state") !== "error") return;
      if (!searchModule) searchAttempt++;
      searchInput.focus();
      searchStart();
    });
    searchDialog.querySelector("[data-rd-search-close]").addEventListener("click", function () { searchDialog.close(); });
    searchInput.addEventListener("compositionstart", function () { searchDialog.setAttribute("data-rd-search-composing", "true"); });
    searchInput.addEventListener("compositionend", function () { searchDialog.setAttribute("data-rd-search-composing", "false"); });
    function searchEnd(restoreFocus) {
      if (searchDialog.getAttribute("data-state") === "closed") return;
      if (searchPending) searchAttempt++;
      searchInstance++;
      searchPending = null;
      clearTimeout(searchTimer);
      if (searchModule) searchModule.cleanup(searchDialog);
      searchDialog.setAttribute("data-state", "closed");
      searchDialog.setAttribute("data-rd-search-composing", "false");
      if (restoreFocus && searchTrigger && searchTrigger.isConnected) searchTrigger.focus({ preventScroll: true });
    }
    searchDialog.addEventListener("close", function () {
      if (searchDialog.open) return;
      searchEnd(true);
    });
  }

  // The sidebar column starts scrolled to the current page.
  var sidebar = document.querySelector(".rd-sidebar-wide");
  var currentItem = sidebar && sidebar.querySelector("[aria-current=page]");
  if (currentItem && sidebar.scrollHeight > sidebar.clientHeight) {
    var offset = currentItem.getBoundingClientRect().top - sidebar.getBoundingClientRect().top;
    if (offset > sidebar.clientHeight - 48) sidebar.scrollTop = offset - sidebar.clientHeight / 3;
  }
});
