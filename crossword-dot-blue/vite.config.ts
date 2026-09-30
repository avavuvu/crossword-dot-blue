import { defineConfig } from "vite";
import { fileURLToPath } from "node:url";
import { existsSync, globSync } from "node:fs";

const localBoutique = fileURLToPath(new URL("../../boutique", import.meta.url));
const boutique = existsSync(localBoutique)
    ? localBoutique
    : fileURLToPath(new URL("./node_modules/boutique", import.meta.url));


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
            "@bq": boutique,
            "@setups": fileURLToPath(new URL("./bindings/setups.ts", import.meta.url)),
        },
    },
    build: {
        target: ["chrome109", "edge109", "firefox109", "safari16.3"],
        outDir: "public/build",
        emptyOutDir: true,
        manifest: true,
        rollupOptions: {
            input: {
                site: "resources/js/site.ts",
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
