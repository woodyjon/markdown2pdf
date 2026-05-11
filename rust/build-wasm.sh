#!/usr/bin/env bash
# Build the markdown2pdf WASM module and place output in ../static/wasm/.
# Requires: cargo, wasm32-unknown-unknown target, wasm-bindgen-cli.
# Optional: wasm-opt (binaryen) for size optimisation.

set -euo pipefail
cd "$(dirname "$0")"

cargo build -p markdown2pdf-wasm --target wasm32-unknown-unknown --release

WASM_IN=target/wasm32-unknown-unknown/release/markdown2pdf_wasm.wasm
OUT_DIR=../src/lib/wasm
mkdir -p "$OUT_DIR"

wasm-bindgen \
  --target web \
  --out-dir "$OUT_DIR" \
  --no-typescript \
  "$WASM_IN"

if command -v wasm-opt >/dev/null 2>&1; then
  echo "Optimising with wasm-opt -Oz…"
  wasm-opt -Oz \
    -o "$OUT_DIR/markdown2pdf_wasm_bg.wasm" \
    "$OUT_DIR/markdown2pdf_wasm_bg.wasm"
fi

echo "WASM build complete:"
ls -lh "$OUT_DIR"
