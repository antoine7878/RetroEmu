ROM=./rscs/gb-test-roms/cpu_instrs/cpu_instrs.gb

test:
	cargo run -- -t 1 $(ROM)
	# cargo run -- -t 1 -f debug $(ROM)

