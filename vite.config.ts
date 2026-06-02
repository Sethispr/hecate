import { svelte } from '@sveltejs/vite-plugin-svelte';
import path from 'path';
import { defineConfig } from 'vite';

export default defineConfig(() => {
  return {
    // Dynamically uses the repository name prefix if set of defaults to relative paths
    base: process.env.BASE_URL || './',
    plugins: [svelte()],
    resolve: {
      alias: {
        '@': path.resolve(__dirname, '.')
      }
    },
    build: {
      target: 'esnext',
      minify: 'esbuild' as const,
      cssMinify: 'esbuild' as const,
      modulePreload: { polyfill: false }
    },
    server: {
      hmr: process.env.DISABLE_HMR !== 'true',
      watch: process.env.DISABLE_HMR === 'true' ? null : {}
    }
  };
});
