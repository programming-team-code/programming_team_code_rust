#!/bin/bash
# RUSTFLAGS="-C instrument-coverage -C link-dead-code" oj-verify run --tle 20 --jobs 4 examples/data_structures/rmq.rs

set -o pipefail

executables=$(find target/debug/examples/ -type f -executable | sed 's/^/--object /')
cov_flags=(-Xdemangler=rustfilt --ignore-filename-regex='/.cargo/registry' --instr-profile=ptc_rust.profdata "${executables}")

cargo profdata -- merge -sparse default_*.profraw -o ptc_rust.profdata
cargo cov -- show "${cov_flags[@]}" --use-color
cargo cov -- export "${cov_flags[@]}" --summary-only | python .github/workflows/check_code_cov.py
