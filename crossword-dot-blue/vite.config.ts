import { defineConfig } from "vite";
import { fileURLToPath } from "node:url";
import { globSync } from "node:fs";

const viewStyles = Object.fromEntries(
    globSync("src/views/**/*.css")
        .map((file) => [file.split("/").pop()!.replace(/\.css$/, ""), file])
        .filter(([name]) => !name.startsWith("_")),
);

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
                editor: "resources/js/editor.ts",
                ...viewStyles,
            },
            output: {
                entryFileNames: "[name]-[hash].js",
                chunkFileNames: "[name]-[hash].js",
                assetFileNames: "[name]-[hash].[ext]",
            },
        },
    },
});
