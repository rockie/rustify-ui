// Installed before any page script (Playwright's init script), this records
// what the page itself observes: largest contentful paint, layout shifts in
// session windows, first contentful paint, every resource the page fetched,
// and every CSP violation. The harness reads `window.__rustifyPerf.snapshot()`;
// nothing here is a Playwright timing.
(() => {
  const state = {
    lcp: null,
    lcpElement: null,
    fcp: null,
    cls: 0,
    session: { value: 0, first: 0, last: 0 },
    shifts: 0,
    violations: [],
    inputSeen: false,
  };
  const observe = (type, callback) => {
    try {
      new PerformanceObserver((list) => list.getEntries().forEach(callback)).observe({ type, buffered: true });
    } catch (unsupported) {
      state[`${type}Unsupported`] = true;
    }
  };
  observe("largest-contentful-paint", (entry) => {
    // LCP stops at the first input; later candidates do not count.
    if (state.inputSeen) return;
    state.lcp = entry.startTime;
    const element = entry.element;
    state.lcpElement = element ? `${element.tagName.toLowerCase()}${element.id ? `#${element.id}` : ""}` : null;
  });
  observe("paint", (entry) => {
    if (entry.name === "first-contentful-paint") state.fcp = entry.startTime;
  });
  observe("layout-shift", (entry) => {
    if (entry.hadRecentInput) return;
    state.shifts += 1;
    // Session windows: shifts less than 1 s apart, a window at most 5 s long;
    // CLS is the largest window.
    const session = state.session;
    if (session.value && entry.startTime - session.last < 1000 && entry.startTime - session.first < 5000) {
      session.value += entry.value;
      session.last = entry.startTime;
    } else {
      session.value = entry.value;
      session.first = entry.startTime;
      session.last = entry.startTime;
    }
    state.cls = Math.max(state.cls, session.value);
  });
  for (const type of ["pointerdown", "keydown"]) {
    addEventListener(type, () => (state.inputSeen = true), { capture: true, once: true });
  }
  addEventListener("securitypolicyviolation", (event) => {
    state.violations.push({ directive: event.effectiveDirective, blocked: event.blockedURI, source: event.sourceFile });
  });
  window.__rustifyPerf = {
    snapshot() {
      const navigation = performance.getEntriesByType("navigation")[0];
      const resources = performance.getEntriesByType("resource").map((entry) => ({
        name: entry.name,
        type: entry.initiatorType,
        transfer: entry.transferSize,
        body: entry.encodedBodySize,
      }));
      return {
        timeOrigin: performance.timeOrigin,
        lcp: state.lcp,
        lcpElement: state.lcpElement,
        fcp: state.fcp,
        cls: state.cls,
        shifts: state.shifts,
        violations: state.violations,
        navigation: navigation && {
          transfer: navigation.transferSize,
          body: navigation.encodedBodySize,
          responseEnd: navigation.responseEnd,
          domContentLoaded: navigation.domContentLoadedEventEnd,
          load: navigation.loadEventEnd,
        },
        resources,
      };
    },
  };
})();
