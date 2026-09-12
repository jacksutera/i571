#!/bin/sh

# No submitted files will be executable; ensure that this script works
# under those conditions.

DIR=$(dirname "$0")
exec python3 "$DIR/bits-571.py"
