#!/bin/bash

cd $1
rustc solve.rs && ./solve 1 < input.txt
rustc solve.rs && ./solve 2 < input.txt
