run:
	rustc --crate-type=lib src/lib.rs -o library.rlib
	rustc $(src) --extern clip=library.rlib
	./$(src) --file test1.txt

test:
	cargo test -- --test
