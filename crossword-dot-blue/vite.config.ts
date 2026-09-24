import { defineConfig } from "vite";
import { fileURLToPath } from "node:url";

export default defineConfig({
    publicDir: false,
    resolve: {
        alias: {
            "@bindings": fileURLToPath(new URL("../crossword-tools/bindings", import.meta.url)),
        },
    },
    build: {
        outDir: "public",
        emptyOutDir: false,
        rollupOptions: {
            input: {
                app: "resources/css/app.css",
                site: "resources/js/site.ts",
                crossword: "resources/js/crossword.ts",
                editor: "resources/js/editor.ts",
            },
            output: {
                entryFileNames: "assets/[name].js",
                chunkFileNames: "assets/[name]-[hash].js",
                assetFileNames: "assets/[name].[ext]",
            },
        },
    },
});
