// The stand-in tag's queue, as a vendor's inline snippet would set it up.
(window.fakeTagQueue = window.fakeTagQueue || []).push("page_view");
if (document.querySelector("main [data-conversion]")) {
  window.fakeTagQueue.push("conversion");
}
