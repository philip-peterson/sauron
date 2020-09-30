#!/bin/bash

set -v

./bootstrap.sh

wasm-pack build --target web --release -- --features "wee_alloc console_error_panic_hook with-request-animation-frame with-storage"

./copy_files_to_benchmark_directory.sh

basic-http-server ./ -a 0.0.0.0:6001
