// The default theme's enhancements. Everything they act on already works
// without them: the mode follows the system, every code block is readable
// and selectable, every heading and page is a link. A failure here hides
// nothing.
(function () {
  "use strict";
  var root = document.documentElement;
  var base = root.getAttribute("data-rd-base") || "/";
  var status = document.getElementById("rd-status");

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

  function announce(text) {
    if (!status || !text) return;
    status.textContent = "";
    // A changed text node is what a live region reads; clearing first makes
    // the same message announce twice in a row.
    setTimeout(function () {
      status.textContent = text;
    }, 50);
  }

  function label(name) {
    return status ? status.getAttribute("data-" + name) || "" : "";
  }

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
    if (text) text.textContent = label(state === "copied" ? "copied" : "copy-failed-short");
    button.rdCopyTimer = setTimeout(function () {
      button.removeAttribute("data-state");
      if (text) text.textContent = button.getAttribute("data-rd-copy-text");
    }, state === "copied" ? 2000 : 4000);
  }

  function copy(button, figure) {
    var raw = figure && figure.querySelector("template.rd-code-raw");
    if (!raw) return;
    var failed = function () {
      showCopy(button, "failed");
      announce(label("copy-failed"));
      selectCode(figure);
    };
    if (!navigator.clipboard || !window.isSecureContext) {
      failed();
      return;
    }
    navigator.clipboard.writeText(raw.content.textContent).then(function () {
      showCopy(button, "copied");
      announce(label("copied"));
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

  // The sidebar column starts scrolled to the current page.
  var sidebar = document.querySelector(".rd-sidebar-wide");
  var currentItem = sidebar && sidebar.querySelector("[aria-current=page]");
  if (currentItem && sidebar.scrollHeight > sidebar.clientHeight) {
    var offset = currentItem.getBoundingClientRect().top - sidebar.getBoundingClientRect().top;
    if (offset > sidebar.clientHeight - 48) sidebar.scrollTop = offset - sidebar.clientHeight / 3;
  }
})();
