
new-day:
	cargo new day-$(DAY)
	rm -rf day-$(DAY)/src
	cp -r template_src/src day-$(DAY)/

test:
	cd day-$(DAY) && cargo test

run-part1: 
	cd day-$(DAY) && cargo run --bin part1

run-part2: 
	cd day-$(DAY) && cargo run --bin part2


