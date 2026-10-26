ROM="./rscs/gb-test-roms/cpu_instrs/individual/06-ld r,r.gb"

test:
	cargo run -- -t 0 -f debug $(ROM)
	# cargo run -- -t 1 -f debug $(ROM)

