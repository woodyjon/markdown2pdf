#!/usr/bin/env bash
# Build the static site (WASM + SvelteKit) and deploy to Firebase Hosting.
#
# Prereqs (one-time):
#   - npm i -g firebase-tools  (or use `bunx firebase-tools`)
#   - firebase login
#   - Edit .firebaserc to point "default" at your Firebase project ID
set -euo pipefail

cd "$(dirname "$0")"

echo "==> Building Rust → WASM"
bash rust/build-wasm.sh

echo "==> Building static site (SvelteKit + adapter-static)"
bun run build

echo "==> Deploying to Firebase Hosting"
if command -v firebase >/dev/null 2>&1; then
  firebase deploy --only hosting
else
  bunx firebase-tools deploy --only hosting
fi

echo ""
echo "Deploy complete!"
