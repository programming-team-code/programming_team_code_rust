#!/bin/bash

# RUSTFLAGS="-C instrument-coverage -C link-dead-code" oj-verify run src/data_structures/seg_tree.rs

cargo profdata -- merge -sparse default_*.profraw -o ptc_rust.profdata
cargo cov -- show -Xdemangler=rustfilt --use-color --ignore-filename-regex='/.cargo/registry' --instr-profile=ptc_rust.profdata $(find target/debug/examples/ -type f -executable | sed 's/^/--object /')
cargo cov -- export -Xdemangler=rustfilt --ignore-filename-regex='/.cargo/registry' --instr-profile=ptc_rust.profdata $(find target/debug/examples/ -type f -executable | sed 's/^/--object /') --summary-only | python .github/workflows/check_code_cov.py
