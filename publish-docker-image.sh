#!/bin/sh
set -e -u

docker run --rm -v "$(pwd)":/build fredrikfornwall/rust-static-builder-nightly

docker build -t fredrikfornwall/advent-of-code-2018-rs .

docker push fredrikfornwall/advent-of-code-2018-rs
