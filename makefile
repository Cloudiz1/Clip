run:
	rustc --crate-type=lib src/lib.rs -o library.rlib
	rustc examples/$(src).rs --extern clip=library.rlib -o bin/$(src)
	./bin/$(src)
