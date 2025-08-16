// @ts-check
import { defineConfig } from 'astro/config';

import svelte from '@astrojs/svelte';

import tailwindcss from '@tailwindcss/vite';



// https://astro.build/config
export default defineConfig({
  integrations: [svelte()],


  vite: {
    // @ts-ignore
    plugins: [tailwindcss()]
  },
  
  // experimental: {
  //   session: true,
  // },
  // adapter: netlify({
  //   cacheOnDemandPages: true,
    
  // }),
});