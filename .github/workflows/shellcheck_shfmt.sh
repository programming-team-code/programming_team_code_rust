#!/bin/bash

shellcheck --shell=bash --check-sourced --enable=all .github/workflows/*.sh || exit 1

shfmt -ln=bash -s -d -w .github/workflows/*.sh || exit 1
