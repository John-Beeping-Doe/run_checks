#!/bin/bash

# Check if a directory is provided as an argument; if not, use the current directory
DIR=${1:-.}

# Use find to list directories up to a depth of 3 and sed to format them in a tree-like structure
find "$DIR" -maxdepth 2 -print | sed -e 's;[^/]*/;|____;g;s;____|; |;g'