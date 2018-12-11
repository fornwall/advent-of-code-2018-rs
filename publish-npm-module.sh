#!/bin/sh
set -e -u

wasm-pack build --target nodejs

cd pkg
npm publish

cd ../advent_of_code_rs_bin
npm publish
