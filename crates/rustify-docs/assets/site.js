// The default theme's enhancements. Everything they act on already works
// without them: the mode follows the system, code is selectable, every
// heading is a link. A failure here hides nothing.
(function () {
  "use strict";
  var root = document.documentElement;
  var key = "rustify-docs:mode:" + (root.getAttribute("data-rd-base") || "/");
  var status = document.getElementById("rd-status");

  function announce(text) {
    if (!status) return;
    status.textContent = "";
    // A changed text node is what a live region reads; clearing first makes
    // the same message announce twice in a row.
    setTimeout(function () {
      status.textContent = text;
    }, 50);
  }

  // The colour mode switch.
  var modeSwitch = document.querySelector("[data-rd-mode-switch]");
  if (modeSwitch) {
    var buttons = modeSwitch.querySelectorAll("[data-rd-mode]");
    var mark = function () {
      var current = root.getAttribute("data-theme") || "system";
      buttons.forEach(function (button) {
        button.setAttribute("aria-pressed", String(button.getAttribute("data-rd-mode") === current));
      });
    };
    buttons.forEach(function (button) {
      button.addEventListener("click", function () {
        var mode = button.getAttribute("data-rd-mode");
        root.setAttribute("data-theme", mode);
        try {
          localStorage.setItem(key, mode);
        } catch (refused) {
          // The choice holds for this page only.
        }
        mark();
      });
    });
    mark();
    modeSwitch.hidden = false;
  }

  // Copying a code block's raw text, never its rendered lines.
  document.querySelectorAll("[data-rd-copy]").forEach(function (button) {
    var figure = button.closest(".rd-code");
    var raw = figure && figure.querySelector("template.rd-code-raw");
    if (!raw || !navigator.clipboard) return;
    button.hidden = false;
    button.addEventListener("click", function () {
      navigator.clipboard.writeText(raw.content.textContent).then(
        function () {
          announce(status ? status.getAttribute("data-copied") : "");
        },
        function () {
          announce(status ? status.getAttribute("data-copy-failed") : "");
        }
      );
    });
  });

  // The table of contents follows the section at the top of the viewport.
  var links = document.querySelectorAll("[data-rd-toc-link]");
  if (links.length && "IntersectionObserver" in window) {
    var byId = {};
    var headings = [];
    links.forEach(function (link) {
      var id = decodeURIComponent(link.getAttribute("href").slice(1));
      var heading = document.getElementById(id);
      if (heading) {
        byId[id] = link;
        headings.push(heading);
      }
    });
    var visible = {};
    var current = null;
    var update = function () {
      var active = null;
      for (var i = 0; i < headings.length; i++) {
        if (visible[headings[i].id]) {
          active = headings[i].id;
          break;
        }
      }
      if (!active) {
        // Between headings: the last one above the viewport's top.
        for (var j = headings.length - 1; j >= 0; j--) {
          if (headings[j].getBoundingClientRect().top < 96) {
            active = headings[j].id;
            break;
          }
        }
      }
      if (active === current) return;
      if (current && byId[current]) byId[current].removeAttribute("aria-current");
      current = active;
      if (current && byId[current]) byId[current].setAttribute("aria-current", "location");
    };
    var observer = new IntersectionObserver(
      function (entries) {
        entries.forEach(function (entry) {
          visible[entry.target.id] = entry.isIntersecting;
        });
        update();
      },
      { rootMargin: "-56px 0px -66% 0px" }
    );
    headings.forEach(function (heading) {
      observer.observe(heading);
    });
  }

  // The sidebar starts scrolled to the current page.
  var currentItem = document.querySelector(".rd-sidebar [aria-current=page]");
  var sidebar = document.querySelector(".rd-sidebar");
  if (currentItem && sidebar && sidebar.scrollHeight > sidebar.clientHeight) {
    var offset = currentItem.getBoundingClientRect().top - sidebar.getBoundingClientRect().top;
    if (offset > sidebar.clientHeight - 48) sidebar.scrollTop = offset - sidebar.clientHeight / 3;
  }
})();
