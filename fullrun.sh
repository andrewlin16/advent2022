#!/bin/bash

set -eu

if [[ $# -lt 1 ]] ; then
	echo "Usage: $0 <name>"
	exit 1
fi

./test.sh "$1" && ./run.sh "$1"
