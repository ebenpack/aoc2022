.PHONY: run
run:
	AOC_INPUT=~/Desktop/sandbox/aoc-2022/inputs cargo run -- run $(day) $(bench)

.PHONY: bench
bench:
	AOC_INPUT=~/Desktop/sandbox/aoc-2022/inputs cargo run --release -- run $(day) --bench
