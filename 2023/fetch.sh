#!/bin/bash

day=$(printf %02d "$1")
if [ ! -f input/"$day"-input.txt ]; then
    curl -o input/"$day"-input.txt -H @cookie.txt \
        "https://adventofcode.com/2023/day/$1/input"
fi
