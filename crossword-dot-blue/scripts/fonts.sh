#!/bin/sh
set -eu

cd "$(dirname "$0")/.."

UNICODES="U+0020-007E,U+00A0-00FF,U+2013-2014,U+2018-201D,U+2026"
OUT="public/assets/fonts"
mkdir -p "$OUT"

subset() {
    uvx --from fonttools --with brotli pyftsubset "resources/fonts/$1" \
        --unicodes="$UNICODES" \
        --layout-features="kern,liga,calt" \
        --flavor=woff2 \
        --output-file="$OUT/$2.woff2"
}

subset HealTheWebA-Regular.otf heal-the-web-a-regular

ls -la "$OUT"
