#!/bin/bash

set -eu

if [[ $# -lt 1 ]] ; then
	echo "Usage: $0 <name>"
	exit 1
fi

inputfile="${2:-data/$1_sample-input.txt}"

cargo run --bin "$1" < "${inputfile}"
