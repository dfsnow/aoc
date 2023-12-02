#!/bin/bash

day=$(printf %02d "$1")
mkdir -p "$day"
if [ ! -f "$day"/input.txt ]; then
    curl -o "$day"/input.txt -H @cookie.txt \
        "https://adventofcode.com/2023/day/$1/input"
fi
