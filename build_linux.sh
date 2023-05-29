#!/usr/bin/env bash

docker run --rm -it -v "$(pwd)":/app skitsanos/rust-builder-machine cargo build --release