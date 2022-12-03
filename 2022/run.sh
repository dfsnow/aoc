#!/bin/bash

mkdir -p $1/rust $1/r
if [ ! -f $1/input.txt ]; then
    curl -o $1/input.txt -H @cookie.txt \
        "https://adventofcode.com/2022/day/$1/input"
fi
