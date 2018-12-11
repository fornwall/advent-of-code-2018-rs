#!/bin/sh
set -e -u

wasm-pack build --target browser --out-dir node/pkg-browser
cd node/www
webpack --config webpack.config.js
