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
        outDir: "public/build",
        emptyOutDir: true,
        manifest: true,
        rollupOptions: {
            input: {
                site: "resources/js/site.ts",
                crossword: "resources/js/crossword.ts",
                editor: "resources/js/editor.ts",
            },
            output: {
                entryFileNames: "[name]-[hash].js",
                chunkFileNames: "[name]-[hash].js",
                assetFileNames: "[name]-[hash].[ext]",
            },
        },
    },
});
