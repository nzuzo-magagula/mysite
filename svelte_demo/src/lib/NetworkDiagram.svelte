<script>
  /*
   * The network itself, drawn from the live weight matrices. Edge thickness
   * tracks |w| and hue tracks its sign, so you can watch connections
   * strengthen and flip as training runs.
   */
  let { topology = null, training = false } = $props();

  const W = 460;
  const H = 220;
  const PAD_X = 34;
  const PAD_Y = 22;

  let layers = $derived(topology?.sizes ?? []);
  let weights = $derived(topology?.weights ?? []);

  // Node positions, one column per layer.
  let nodes = $derived.by(() => {
    if (!layers.length) return [];
    const cols = layers.length;
    return layers.map((count, li) => {
      const x = cols === 1 ? W / 2 : PAD_X + (li * (W - 2 * PAD_X)) / (cols - 1);
      const usable = H - 2 * PAD_Y;
      return Array.from({ length: count }, (_, ni) => ({
        x,
        y: count === 1 ? H / 2 : PAD_Y + (ni * usable) / (count - 1),
      }));
    });
  });

  // Normalise edge widths against the largest |w| in the whole network so the
  // diagram stays readable whatever scale the weights drift to.
  let maxAbs = $derived.by(() => {
    let m = 0.001;
    for (const layer of weights)
      for (const row of layer)
        for (const w of row) m = Math.max(m, Math.abs(w));
    return m;
  });

  let edges = $derived.by(() => {
    const out = [];
    for (let li = 0; li < weights.length; li++) {
      const from = nodes[li];
      const to = nodes[li + 1];
      if (!from || !to) continue;
      for (let i = 0; i < weights[li].length; i++) {
        for (let j = 0; j < weights[li][i].length; j++) {
          const w = weights[li][i][j];
          const mag = Math.abs(w) / maxAbs;
          if (mag < 0.04) continue; // hide near-dead connections
          out.push({
            x1: from[j].x,
            y1: from[j].y,
            x2: to[i].x,
            y2: to[i].y,
            width: 0.35 + mag * 2.6,
            positive: w >= 0,
            opacity: 0.12 + mag * 0.68,
          });
        }
      }
    }
    return out;
  });

  const labelFor = (li) =>
    li === 0 ? "input" : li === layers.length - 1 ? "output" : `hidden ${li}`;
</script>

<div class="wrap" class:training>
  {#if layers.length}
    <svg viewBox="0 0 {W} {H}" role="img" aria-label="Network architecture">
      {#each edges as e}
        <line
          x1={e.x1}
          y1={e.y1}
          x2={e.x2}
          y2={e.y2}
          stroke={e.positive ? "var(--class-a)" : "var(--class-b)"}
          stroke-width={e.width}
          stroke-opacity={e.opacity}
          stroke-linecap="round"
        />
      {/each}

      {#each nodes as column, li}
        {#each column as n}
          <circle
            cx={n.x}
            cy={n.y}
            r={li === 0 || li === layers.length - 1 ? 6.5 : 5}
            fill="var(--base-100)"
            stroke="var(--content)"
            stroke-opacity="0.5"
            stroke-width="1.4"
          />
        {/each}
        <text x={column[0]?.x ?? 0} y={H - 5} text-anchor="middle" class="lbl">
          {labelFor(li)}
        </text>
      {/each}
    </svg>

    <div class="legend">
      <span><i class="sw pos"></i> positive weight</span>
      <span><i class="sw neg"></i> negative weight</span>
      <span class="dim">thickness = |w|</span>
    </div>
  {:else}
    <div class="empty">Network appears once the runtime starts</div>
  {/if}
</div>

<style>
  .wrap {
    background: var(--sunken);
    border: 1px solid var(--hairline);
    border-radius: 12px;
    padding: 0.5rem 0.5rem 0.35rem;
  }

  svg {
    display: block;
    width: 100%;
    height: auto;
  }

  /* A gentle pulse while epochs are running. */
  .training svg circle {
    animation: breathe 2.4s ease-in-out infinite;
  }

  @keyframes breathe {
    0%,
    100% {
      stroke-opacity: 0.5;
    }
    50% {
      stroke-opacity: 0.85;
    }
  }

  .lbl {
    font-family: "JetBrains Mono", monospace;
    font-size: 8.5px;
    fill: var(--content);
    fill-opacity: 0.45;
    letter-spacing: 0.06em;
  }

  .legend {
    display: flex;
    flex-wrap: wrap;
    gap: 0.85rem;
    padding: 0.35rem 0.4rem 0.15rem;
    font-size: 0.68rem;
    color: color-mix(in oklab, var(--content) 60%, transparent);
  }

  .legend span {
    display: inline-flex;
    align-items: center;
    gap: 0.3rem;
  }

  .sw {
    width: 14px;
    height: 2.5px;
    border-radius: 2px;
    display: inline-block;
  }
  .sw.pos {
    background: var(--class-a);
  }
  .sw.neg {
    background: var(--class-b);
  }
  .dim {
    opacity: 0.7;
    font-family: "JetBrains Mono", monospace;
  }

  .empty {
    padding: 3rem 1rem;
    text-align: center;
    font-size: 0.82rem;
    color: color-mix(in oklab, var(--content) 45%, transparent);
  }
</style>
