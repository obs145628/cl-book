#!/bin/bash

# Basic script to run cargo build and cargo test on all projects

CMD="cargo build && cargo test"
if [[ $* == *--only-build* ]]; then {
  CMD="cargo build"
} fi



check_proj() {
    (
	echo "Testing $1 ..."
	cd ./$1;
	eval $CMD

    )
    if [ $? -ne 0 ]; then {
	echo "Tests failed for project $1";
	exit 1
    } fi
}

check_proj libs/asmparser
check_proj libs/clangutils
check_proj libs/interp_irint3a
check_proj libs/interp_irintsm
check_proj libs/irint3a
check_proj libs/irintsm
check_proj libs/lanexpr
check_proj libs/oblexer
check_proj libs/obparser
check_proj libs/obtests
check_proj libs/obuid

check_proj apps/cl-lanexpr
check_proj apps/irint3a-utils/
check_proj apps/irintsm-utils/
check_proj apps/lexer-list/
check_proj apps/mini-calc-eval/
