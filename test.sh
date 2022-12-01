#!/bin/bash

set -eu
shopt -s failglob

if [[ $# -lt 1 ]] ; then
	echo "Usage: $0 <name>"
	exit 1
fi

for infile in data/$1_sample-input*.txt; do
	echo "${infile}"
	outfile="${infile/input/output}"
	diff "${outfile}" <(cargo run --bin "$1" < "${infile}") && echo "pass" || echo "fail"
done
