#!/bin/bash

set -eu

if [[ $# -lt 1 ]] ; then
	echo "Usage: $0 <name>"
	exit 1
fi

cargo run --bin "$1" < "data/$1_input.txt"
