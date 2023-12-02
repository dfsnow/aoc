#!/bin/bash

cd $1
rustc solve.rs && ./solve $2 < input.txt
