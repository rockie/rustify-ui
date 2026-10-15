// rustify-docs dev: the page follows the builds of the site being written.
// Served only by `rustify-docs dev`; no production build contains it.
//
// When a build changes this page, the page reloads and returns to where the
// reader was. When a build fails, the page stays the last good version and a
// panel in the corner lists the problems; hidden, it leaves a marker that
// opens it again. The next good build clears both. Nothing here takes focus
// the author did not hand it.
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
  if (!script || !window.EventSource) return;
  var query = new URL(script.src).searchParams;
  // Which run of `dev` served this page, and the build it is a copy of.
  var session = query.get("session") || "";
  var good = query.get("good") || "0";
  var words = {};
  var wordLangs = {};
  try {
    words = JSON.parse(script.getAttribute("data-labels") || "{}");
    wordLangs = JSON.parse(script.getAttribute("data-label-langs") || "{}");
  } catch (malformed) {
    words = {};
  }
  var lang = script.getAttribute("data-lang") || document.documentElement.lang;
  function wordLang(key) { return wordLangs[key] || lang; }
  var scrollKey = "rustify-docs:dev:scroll:" + location.pathname;
  var hiddenKey = "rustify-docs:dev:hidden";

  function stored(key) {
    try {
      return sessionStorage.getItem(key);
    } catch (refused) {
      return null;
    }
  }

  function store(key, value) {
    try {
      if (value === null) sessionStorage.removeItem(key);
      else sessionStorage.setItem(key, value);
    } catch (refused) {
      // Blocked storage: the position and the choice last this page only.
    }
  }

  // Back to where the reader was before the reload, once the page, its
  // images included, has laid out. A page that got shorter scrolls as far
  // as it now can.
  var saved = stored(scrollKey);
  if (saved) {
    store(scrollKey, null);
    if ("scrollRestoration" in history) history.scrollRestoration = "manual";
    var position = saved.split(",").map(Number);
    var restore = function () {
      requestAnimationFrame(function () {
        window.scrollTo(position[0] || 0, position[1] || 0);
      });
    };
    if (document.readyState === "complete") restore();
    else window.addEventListener("load", restore, { once: true });
  }

  function reload() {
    store(scrollKey, Math.round(window.scrollX) + "," + Math.round(window.scrollY));
    if ("scrollRestoration" in history) history.scrollRestoration = "manual";
    location.reload();
  }

  function element(tag, className, text, key) {
    var node = document.createElement(tag);
    if (className) node.className = className;
    if (text !== undefined) node.textContent = text;
    if (key) node.lang = wordLang(key);
    return node;
  }

  function icon(paths) {
    var ns = "http://www.w3.org/2000/svg";
    var svg = document.createElementNS(ns, "svg");
    svg.setAttribute("viewBox", "0 0 24 24");
    svg.setAttribute("fill", "none");
    svg.setAttribute("stroke", "currentColor");
    svg.setAttribute("stroke-width", "2");
    svg.setAttribute("stroke-linecap", "round");
    svg.setAttribute("stroke-linejoin", "round");
    svg.setAttribute("aria-hidden", "true");
    svg.setAttribute("focusable", "false");
    paths.forEach(function (d) {
      var path = document.createElementNS(ns, "path");
      path.setAttribute("d", d);
      svg.appendChild(path);
    });
    return svg;
  }

  function count(n, one, other) {
    return String(n === 1 ? one : other).replace("{n}", String(n));
  }

  // The live region exists from the start: one added with its text is
  // often not read.
  var live = element("p", "rd-dev-sr");
  live.setAttribute("role", "status");
  live.setAttribute("lang", lang);
  document.body.appendChild(live);

  function announce(text, key, includeRevision) {
    live.textContent = "";
    setTimeout(function () {
      if (key) {
        live.lang = wordLang(key);
        live.textContent = text;
      } else {
        live.lang = lang;
        summary.childNodes.forEach(function (node) { live.appendChild(node.cloneNode(true)); });
        if (includeRevision) {
          live.appendChild(document.createTextNode(" · "));
          live.appendChild(revision.cloneNode(true));
        }
      }
    }, 50);
  }

  // While nothing has built, the page is the problems: the list goes in its
  // column, open, rather than over it in a corner.
  var article = Number(good) > 0 ? null : document.querySelector("main .rd-prose");
  var inline = article !== null;
  var root = element("div", inline ? "rd-dev rd-dev-inline" : "rd-dev");
  root.setAttribute("lang", lang);
  root.hidden = true;
  var panel = element("div", "rd-dev-panel");
  panel.id = "rd-dev-panel";
  if (inline) {
    panel.setAttribute("role", "region");
  } else {
    panel.setAttribute("role", "dialog");
    panel.setAttribute("aria-modal", "false");
  }
  panel.setAttribute("aria-labelledby", "rd-dev-summary");
  var head = element("div", "rd-dev-head");
  var summary = element("p", "rd-dev-summary");
  summary.id = "rd-dev-summary";
  var revision = element("span", "rd-dev-revision");
  var hide = element("button", "rd-dev-hide");
  hide.type = "button";
  hide.setAttribute("aria-label", words.hide || "Hide");
  hide.title = words.hide || "Hide";
  hide.lang = wordLang("hide");
  hide.appendChild(icon(["M18 6 6 18", "m6 6 12 12"]));
  head.appendChild(summary);
  head.appendChild(revision);
  if (!inline) head.appendChild(hide);
  var list = element("div", "rd-dev-list");
  // The list scrolls on its own; a keyboard reaches it.
  list.tabIndex = 0;
  list.setAttribute("role", "region");
  list.setAttribute("aria-label", words.problems || "Problems");
  list.lang = wordLang("problems");
  panel.appendChild(head);
  panel.appendChild(list);
  var marker = element("button", "rd-dev-marker");
  marker.type = "button";
  marker.setAttribute("aria-controls", panel.id);
  marker.setAttribute("aria-expanded", "false");
  var markerText = element("span");
  marker.appendChild(icon(["M12 8v4", "M12 16h.01", "M12 3a9 9 0 1 0 0 18 9 9 0 0 0 0-18Z"]));
  marker.appendChild(markerText);
  root.appendChild(panel);
  if (inline) {
    article.parentNode.insertBefore(root, article.nextSibling);
  } else {
    root.appendChild(marker);
    document.body.appendChild(root);
  }

  var open = inline;
  var last = null;

  // "2 errors · 1 warning · showing the last good version", as spans the
  // stylesheet colours by kind. In the panel's head the state of the page
  // takes a line of its own, its separator kept for assistive technology.
  function describe(state, into, stacked) {
    into.textContent = "";
    var parts = [];
    if (state.building) {
      parts.push(["building", words.building || "Building", "building"]);
      if (state.phase && words[state.phase]) parts.push(["building", words[state.phase], state.phase]);
    }
    if (state.errors) parts.push(["error", count(state.errors, words.error_one, words.error_other), state.errors === 1 ? "error_one" : "error_other"]);
    if (state.warnings) parts.push(["warning", count(state.warnings, words.warning_one, words.warning_other), state.warnings === 1 ? "warning_one" : "warning_other"]);
    var page = null;
    var pageKey = "";
    if (Number(good) > 0 && (state.building || !state.ok)) page = words.last_good;
    else if (!state.ok && !state.building) page = words.no_last_good;
    if (page) pageKey = Number(good) > 0 ? "last_good" : "no_last_good";
    parts.forEach(function (part, i) {
      if (i > 0) into.appendChild(document.createTextNode(" · "));
      into.appendChild(element("span", "rd-dev-count-" + part[0], part[1], part[2]));
    });
    if (page && stacked) {
      var line = element("span", "rd-dev-state");
      line.appendChild(element("span", "rd-dev-sr", " · "));
      line.appendChild(element("span", "", page, pageKey));
      into.appendChild(line);
    } else if (page) {
      into.appendChild(document.createTextNode(" · "));
      into.appendChild(element("span", "", page, pageKey));
    }
    var text = parts.map(function (part) {
      return part[1];
    });
    if (page) text.push(page);
    return text.join(" · ");
  }

  // A message as text, with what it quotes in `backticks` set as code, and
  // a break allowed after each "/" of a path.
  function breakable(into, text) {
    text.split("/").forEach(function (piece, i, pieces) {
      into.appendChild(document.createTextNode(piece + (i < pieces.length - 1 ? "/" : "")));
      if (i < pieces.length - 1) into.appendChild(document.createElement("wbr"));
    });
    return into;
  }

  function prose(className, text) {
    var node = element("p", className);
    text = String(text);
    var runs = /`+/g;
    var start = 0;
    var opening;
    while ((opening = runs.exec(text))) {
      var contentStart = runs.lastIndex;
      var closing;
      while ((closing = runs.exec(text))) {
        if (closing[0].length === opening[0].length) break;
      }
      if (!closing) {
        // An unmatched run belongs to the original prose, including compiler
        // and parser messages we do not rewrite. Later runs may still pair.
        runs.lastIndex = contentStart;
        continue;
      }
      node.appendChild(document.createTextNode(text.slice(start, opening.index)));
      var piece = text.slice(contentStart, closing.index);
      if (opening[0].length >= 2 && piece.length >= 2 && piece[0] === " " && piece[piece.length - 1] === " ") {
        piece = piece.slice(1, -1);
      }
      node.appendChild(breakable(element("code", "rd-dev-code"), piece));
      start = runs.lastIndex;
    }
    node.appendChild(document.createTextNode(text.slice(start)));
    return node;
  }

  function fill(diagnostics) {
    list.textContent = "";
    var byFile = [];
    var index = {};
    diagnostics.forEach(function (d) {
      var file = d.file || "";
      if (!(file in index)) {
        index[file] = byFile.length;
        byFile.push({ file: file, items: [] });
      }
      byFile[index[file]].items.push(d);
    });
    // Files with errors first, so the first problem under a red count is
    // an error; otherwise in the order the build reported them.
    var hasError = function (group) {
      return group.items.some(function (d) {
        return d.severity === "error";
      });
    };
    byFile = byFile.filter(hasError).concat(
      byFile.filter(function (group) {
        return !hasError(group);
      })
    );
    byFile.forEach(function (group) {
      var section = element("section");
      if (group.file) section.appendChild(breakable(element("h2", "rd-dev-file"), group.file));
      var items = element("ul", "rd-dev-items");
      group.items.forEach(function (d) {
        var item = element("li", "rd-dev-item");
        item.setAttribute("data-severity", d.severity);
        var where = element("div", "rd-dev-where");
        if (d.line) {
          where.appendChild(element("span", "rd-dev-location", d.line + (d.column ? ":" + d.column : "")));
        }
        var rule = element("span", "rd-dev-rule");
        rule.appendChild(element("span", "rd-dev-sr", (d.severity === "error" ? words.error : words.warning) + ": ", d.severity));
        rule.appendChild(document.createTextNode(d.rule));
        where.appendChild(rule);
        item.appendChild(where);
        item.appendChild(prose("rd-dev-message", d.message));
        if (d.suggestion) item.appendChild(prose("rd-dev-suggestion", d.suggestion));
        items.appendChild(item);
      });
      section.appendChild(items);
      list.appendChild(section);
    });
  }

  function show(state) {
    var problems = state.errors + state.warnings;
    if (problems === 0 && !state.building) {
      // Gone with the problems; a keyboard user who was on it continues
      // from the page's content rather than from the top of the document.
      var focused = root.contains(document.activeElement);
      root.hidden = true;
      open = false;
      var main = document.querySelector("main");
      if (focused && main) {
        if (!main.hasAttribute("tabindex")) main.tabIndex = -1;
        main.focus({ preventScroll: true });
      }
      return "";
    }
    root.hidden = false;
    var text = describe(state, summary, true);
    describe(state, markerText, false);
    revision.textContent = count(state.revision, words.revision, words.revision);
    revision.lang = wordLang("revision");
    panel.hidden = !open;
    marker.hidden = open;
    marker.setAttribute("aria-expanded", String(open));
    fill(state.diagnostics || []);
    return text;
  }

  function apply(state) {
    var hadErrors = last !== null && last.errors > 0;
    // Which build result this page has seen, shown or not.
    root.setAttribute("data-revision", String(state.revision));
    if (inline || (state.errors > 0 && !stored(hiddenKey))) open = true;
    if (state.errors + state.warnings === 0 && !state.building) store(hiddenKey, null);
    var text = show(state);
    if (state.building && (last === null || !last.building || last.phase !== state.phase)) {
      announce(text);
    } else if (state.errors > 0 && (last === null || last.revision !== state.revision)) {
      announce(text, null, true);
    } else if (hadErrors && state.errors === 0) {
      announce(words.fixed || "", "fixed");
    }
    last = state;
  }

  marker.addEventListener("click", function () {
    open = true;
    store(hiddenKey, null);
    if (last) show(last);
    hide.focus();
  });

  function close() {
    open = false;
    store(hiddenKey, "1");
    if (last) show(last);
    marker.focus();
  }

  hide.addEventListener("click", close);
  // The list on a page with nothing built is the page: it does not hide.
  panel.addEventListener("keydown", function (event) {
    if (event.key === "Escape" && !inline) {
      event.preventDefault();
      close();
    }
  });

  function connect() {
    var source = new EventSource(
      "/__rustify-docs/events?path=" +
        encodeURIComponent(location.pathname) +
        "&good=" +
        encodeURIComponent(good) +
        "&session=" +
        encodeURIComponent(session)
    );
    source.addEventListener("state", function (event) {
      var state;
      try {
        state = JSON.parse(event.data);
      } catch (malformed) {
        return;
      }
      // Another run of `dev`, or a build that changed this page.
      if (state.reload || state.session !== session) {
        source.close();
        reload();
        return;
      }
      if (state.good !== null) good = String(state.good);
      apply(state);
    });
    source.addEventListener("error", function () {
      // A dropped stream reconnects by itself; one the server refused is
      // closed, and is opened again here, saying which build this page is.
      if (source.readyState === EventSource.CLOSED) {
        setTimeout(connect, 1000);
      }
    });
  }

  connect();
});
