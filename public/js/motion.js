/*
 * motion.js — the site's ambient motion layer.
 *
 * Dioxus renders and re-renders the DOM from WASM, so nothing here can assume
 * a stable element set. Everything is driven off a single MutationObserver
 * that re-scans for newly mounted nodes, which keeps scroll reveals and
 * pointer effects working across client-side route changes without the
 * router needing to know this file exists.
 */
(function () {
  "use strict";

  var REDUCED = window.matchMedia("(prefers-reduced-motion: reduce)");

  document.documentElement.classList.remove("no-js");

  /* ------------------------------------------------------------------ *
   * 1. Scroll reveals
   * ------------------------------------------------------------------ */

  var revealObserver = null;

  function ensureRevealObserver() {
    if (revealObserver || !("IntersectionObserver" in window)) return;
    revealObserver = new IntersectionObserver(
      function (entries) {
        entries.forEach(function (entry) {
          if (!entry.isIntersecting) return;
          entry.target.classList.add("is-revealed");
          // One-shot: once revealed an element never re-hides.
          revealObserver.unobserve(entry.target);
        });
      },
      { rootMargin: "0px 0px -8% 0px", threshold: 0.08 }
    );
  }

  function scanReveals(root) {
    var nodes = root.querySelectorAll("[data-reveal]:not(.is-revealed)");
    if (!nodes.length) return;

    if (REDUCED.matches || !revealObserver) {
      nodes.forEach(function (n) {
        n.classList.add("is-revealed");
      });
      return;
    }

    nodes.forEach(function (n) {
      if (n.dataset.revealBound === "1") return;
      n.dataset.revealBound = "1";

      // Anything already on screen at mount time is revealed immediately so
      // above-the-fold content never waits for a scroll event.
      var box = n.getBoundingClientRect();
      if (box.top < window.innerHeight * 0.92 && box.bottom > 0) {
        n.classList.add("is-revealed");
        return;
      }
      revealObserver.observe(n);
    });
  }

  /* ------------------------------------------------------------------ *
   * 2. Cursor spotlight on .card-spotlight
   * ------------------------------------------------------------------ */

  function onPointerMove(e) {
    var card = e.target.closest && e.target.closest(".card-spotlight");
    if (!card) return;
    var r = card.getBoundingClientRect();
    card.style.setProperty("--mx", (e.clientX - r.left).toFixed(1) + "px");
    card.style.setProperty("--my", (e.clientY - r.top).toFixed(1) + "px");
  }

  /* ------------------------------------------------------------------ *
   * 3. Reading progress + nav elevation
   * ------------------------------------------------------------------ */

  function scrollContainerOf(el) {
    var node = el;
    while (node && node !== document.body) {
      var style = getComputedStyle(node);
      if (/(auto|scroll)/.test(style.overflowY) && node.scrollHeight > node.clientHeight) {
        return node;
      }
      node = node.parentElement;
    }
    return null;
  }

  var progressRaf = 0;

  function updateProgress() {
    progressRaf = 0;

    var bar = document.querySelector("[data-read-progress]");
    if (bar) {
      // The article shell scrolls an inner element, not the window.
      var scroller =
        document.querySelector("[data-scroll-root]") ||
        scrollContainerOf(bar) ||
        document.scrollingElement;

      if (scroller) {
        var max = scroller.scrollHeight - scroller.clientHeight;
        var ratio = max > 0 ? scroller.scrollTop / max : 0;
        bar.style.setProperty("--progress", Math.min(1, Math.max(0, ratio)).toFixed(4));
      }
    }

    var nav = document.querySelector("[data-nav]");
    if (nav) {
      var root = document.querySelector("[data-scroll-root]") || document.scrollingElement;
      var y = root ? root.scrollTop : window.scrollY;
      nav.classList.toggle("is-scrolled", y > 8);
    }
  }

  function requestProgress() {
    if (progressRaf) return;
    progressRaf = requestAnimationFrame(updateProgress);
  }

  /* ------------------------------------------------------------------ *
   * 4. Wiring
   * ------------------------------------------------------------------ */

  function boot() {
    ensureRevealObserver();
    scanReveals(document);
    updateProgress();
  }

  // Re-scan whenever Dioxus mutates the tree (route change, async data
  // arriving, a list re-rendering). Coalesced to one pass per frame.
  var scanQueued = false;
  var mutationObserver = new MutationObserver(function () {
    if (scanQueued) return;
    scanQueued = true;
    requestAnimationFrame(function () {
      scanQueued = false;
      ensureRevealObserver();
      scanReveals(document);
      updateProgress();
    });
  });

  function observeTree() {
    mutationObserver.observe(document.body, { childList: true, subtree: true });
  }

  document.addEventListener("pointermove", onPointerMove, { passive: true });
  window.addEventListener("scroll", requestProgress, { passive: true, capture: true });
  window.addEventListener("resize", requestProgress, { passive: true });

  REDUCED.addEventListener("change", function () {
    scanReveals(document);
  });

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", function () {
      boot();
      observeTree();
    });
  } else {
    boot();
    observeTree();
  }
})();
