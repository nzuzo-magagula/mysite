import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'

// Built output is committed to public/ and served by the Dioxus app at
// /neuralnet/, mirroring how react_demo builds into public/algovis.
export default defineConfig({
  plugins: [svelte()],
  build: {
    outDir: '../public/neuralnet',
    emptyOutDir: true,
  },
  base: '/neuralnet/',
})
