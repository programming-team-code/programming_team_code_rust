#!/bin/bash
# RUSTFLAGS="-C instrument-coverage -C link-dead-code -Z coverage-options=branch" oj-verify run --tle 20 --jobs 4 examples/data_structures/rmq.rs

set -eo pipefail

executables=$(find target/debug/examples/ -type f -executable | sed 's/^/--object /')
cov_flags=(-Xdemangler=rustfilt --ignore-filename-regex='/.cargo/registry' --instr-profile=ptc_rust.profdata "${executables}")

cargo profdata -- merge -sparse default_*.profraw -o ptc_rust.profdata
# shellcheck disable=SC2068 # double quotes cause command to fail, not sure why
cargo cov -- show ${cov_flags[@]} --use-color
# shellcheck disable=SC2068
cargo cov -- export ${cov_flags[@]} --summary-only | python .github/workflows/check_code_cov.py
