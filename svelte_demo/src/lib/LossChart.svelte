<script>
  /* Cross-entropy loss over epochs, autoscaled to the window it is showing. */
  let { history = [] } = $props();

  const W = 460;
  const H = 92;

  let bounds = $derived.by(() => {
    if (!history.length) return { lo: 0, hi: 1 };
    const lo = Math.min(...history);
    const hi = Math.max(...history);
    const pad = (hi - lo) * 0.12 || 0.05;
    return { lo: Math.max(0, lo - pad), hi: hi + pad };
  });

  let path = $derived.by(() => {
    if (history.length < 2) return "";
    const { lo, hi } = bounds;
    const span = hi - lo || 1;
    return history
      .map((v, i) => {
        const x = (i / (history.length - 1)) * W;
        const y = H - ((v - lo) / span) * H;
        return `${i === 0 ? "M" : "L"}${x.toFixed(1)},${y.toFixed(1)}`;
      })
      .join(" ");
  });

  let area = $derived(path ? `${path} L${W},${H} L0,${H} Z` : "");
  let latest = $derived(history.length ? history[history.length - 1] : null);
</script>

<div class="wrap">
  <div class="head">
    <span class="ttl">Cross-entropy loss</span>
    {#if latest !== null}
      <span class="val">{latest.toFixed(4)}</span>
    {/if}
  </div>

  <svg viewBox="0 0 {W} {H}" preserveAspectRatio="none" role="img" aria-label="Loss over epochs">
    <defs>
      <linearGradient id="lossFill" x1="0" y1="0" x2="0" y2="1">
        <stop offset="0%" stop-color="var(--primary)" stop-opacity="0.32" />
        <stop offset="100%" stop-color="var(--primary)" stop-opacity="0" />
      </linearGradient>
    </defs>

    {#if path}
      <path d={area} fill="url(#lossFill)" />
      <path
        d={path}
        fill="none"
        stroke="var(--primary)"
        stroke-width="1.6"
        stroke-linejoin="round"
        stroke-linecap="round"
        vector-effect="non-scaling-stroke"
      />
    {:else}
      <text x={W / 2} y={H / 2 + 4} text-anchor="middle" class="ph">
        waiting for the first epoch
      </text>
    {/if}
  </svg>

  <div class="axis">
    <span>{bounds.hi.toFixed(2)}</span>
    <span>{bounds.lo.toFixed(2)}</span>
  </div>
</div>

<style>
  .wrap {
    background: var(--sunken);
    border: 1px solid var(--hairline);
    border-radius: 12px;
    padding: 0.6rem 0.7rem 0.45rem;
    position: relative;
  }

  .head {
    display: flex;
    justify-content: space-between;
    align-items: baseline;
    margin-bottom: 0.35rem;
  }

  .ttl {
    font-family: "JetBrains Mono", monospace;
    font-size: 0.62rem;
    letter-spacing: 0.14em;
    text-transform: uppercase;
    color: color-mix(in oklab, var(--content) 55%, transparent);
  }

  .val {
    font-family: "JetBrains Mono", monospace;
    font-size: 0.8rem;
    font-weight: 600;
    color: var(--primary);
    font-variant-numeric: tabular-nums;
  }

  svg {
    display: block;
    width: 100%;
    height: 92px;
  }

  .ph {
    font-family: "JetBrains Mono", monospace;
    font-size: 9px;
    fill: var(--content);
    fill-opacity: 0.35;
  }

  .axis {
    position: absolute;
    right: 0.7rem;
    top: 2rem;
    bottom: 0.45rem;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    pointer-events: none;
    font-family: "JetBrains Mono", monospace;
    font-size: 0.58rem;
    color: color-mix(in oklab, var(--content) 40%, transparent);
  }
</style>
