<script>
  /*
   * Draws what the network currently believes: the decision surface as a
   * low-res probability field blown up with smoothing, the training points
   * on top, and a contour at p = 0.5.
   */
  let { grid = [], points = [], extent = 2.0, res = 42 } = $props();

  let canvas = $state(null);
  let field = null; // offscreen canvas holding the res×res probability field

  function readColors() {
    const cs = getComputedStyle(document.documentElement);
    return {
      a: cs.getPropertyValue("--class-a").trim() || "#3b5bdb",
      b: cs.getPropertyValue("--class-b").trim() || "#e8a33d",
      line: cs.getPropertyValue("--content").trim() || "#222",
      base: cs.getPropertyValue("--base-100").trim() || "#fff",
    };
  }

  // Resolve an oklch()/hex colour to [r,g,b] by letting the browser do it.
  const rgbCache = new Map();
  function toRgb(color) {
    if (rgbCache.has(color)) return rgbCache.get(color);
    const probe = document.createElement("canvas").getContext("2d");
    probe.fillStyle = color;
    probe.fillRect(0, 0, 1, 1);
    const d = probe.getImageData(0, 0, 1, 1).data;
    const rgb = [d[0], d[1], d[2]];
    rgbCache.set(color, rgb);
    return rgb;
  }

  function draw() {
    if (!canvas) return;
    const dpr = Math.min(window.devicePixelRatio || 1, 2);
    const size = canvas.clientWidth;
    if (!size) return;

    canvas.width = size * dpr;
    canvas.height = size * dpr;

    const ctx = canvas.getContext("2d");
    ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
    ctx.clearRect(0, 0, size, size);

    const colors = readColors();

    // ---- probability field ----
    if (grid && grid.length === res * res) {
      if (!field) field = document.createElement("canvas");
      field.width = res;
      field.height = res;
      const fctx = field.getContext("2d");
      const img = fctx.createImageData(res, res);

      const ca = toRgb(colors.a);
      const cb = toRgb(colors.b);

      for (let i = 0; i < grid.length; i++) {
        const p = grid[i];
        // Confidence drives alpha, so the undecided band fades out.
        const conf = Math.abs(p - 0.5) * 2;
        const c = p >= 0.5 ? cb : ca;
        const o = i * 4;
        img.data[o] = c[0];
        img.data[o + 1] = c[1];
        img.data[o + 2] = c[2];
        img.data[o + 3] = Math.round(28 + conf * 132);
      }
      fctx.putImageData(img, 0, 0);

      ctx.imageSmoothingEnabled = true;
      ctx.imageSmoothingQuality = "high";
      // The grid is row-major starting at y = -extent, which is the bottom of
      // the plot, so flip vertically on the way out.
      ctx.save();
      ctx.translate(0, size);
      ctx.scale(1, -1);
      ctx.drawImage(field, 0, 0, size, size);
      ctx.restore();
    }

    // ---- axes ----
    const toPx = (v) => ((v + extent) / (2 * extent)) * size;
    ctx.strokeStyle = colors.line;
    ctx.globalAlpha = 0.12;
    ctx.lineWidth = 1;
    ctx.beginPath();
    ctx.moveTo(toPx(0), 0);
    ctx.lineTo(toPx(0), size);
    ctx.moveTo(0, size - toPx(0));
    ctx.lineTo(size, size - toPx(0));
    ctx.stroke();
    ctx.globalAlpha = 1;

    // ---- training points ----
    const r = Math.max(2.5, size * 0.011);
    for (const [x, y, label] of points) {
      const px = toPx(x);
      const py = size - toPx(y);
      ctx.beginPath();
      ctx.arc(px, py, r, 0, Math.PI * 2);
      ctx.fillStyle = label === 1 ? colors.b : colors.a;
      ctx.fill();
      ctx.lineWidth = 1.25;
      ctx.strokeStyle = colors.base;
      ctx.stroke();
    }
  }

  $effect(() => {
    // Touch the reactive inputs so the effect re-runs when they change.
    grid;
    points;
    draw();
  });

  $effect(() => {
    const onResize = () => draw();
    window.addEventListener("resize", onResize);

    // The parent can swap the palette at any time.
    const mo = new MutationObserver(() => {
      rgbCache.clear();
      draw();
    });
    mo.observe(document.documentElement, {
      attributes: true,
      attributeFilter: ["data-theme"],
    });

    return () => {
      window.removeEventListener("resize", onResize);
      mo.disconnect();
    };
  });
</script>

<div class="wrap">
  <canvas bind:this={canvas} aria-label="Decision boundary of the network"></canvas>
</div>

<style>
  .wrap {
    position: relative;
    width: 100%;
    aspect-ratio: 1;
    border-radius: 12px;
    overflow: hidden;
    background: var(--sunken);
    border: 1px solid var(--hairline);
  }

  canvas {
    display: block;
    width: 100%;
    height: 100%;
  }
</style>
