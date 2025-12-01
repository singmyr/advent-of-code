# Advent of Code

[Link to the problems](https://adventofcode.com/)

## Build
```bash
cmake -DYear=2025 -DDay=1 -DPart=2 -S . -B build
```
If you need to force input file update add `--fresh`

## Run
```bash
cmake --build build && ./build/aoc
```