#!/bin/sh

# TODO: set up this file to run your parser assuming that the current
# directory is the one containing this file.


# No submitted files will be executable; ensure that this script works
# under those conditions.

DIR = "$(cd "$(dirname "$0")" && pwd)"
exec python3 "$DIR/bits-571.py"
