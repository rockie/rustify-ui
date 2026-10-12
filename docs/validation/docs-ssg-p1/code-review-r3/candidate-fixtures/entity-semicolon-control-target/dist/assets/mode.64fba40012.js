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
