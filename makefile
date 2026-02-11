run:
	rustc --crate-type=lib src/lib.rs -o library.rlib
	rustc main.rs --extern clip=library.rlib
	./main --file test1.txt

clean: 
	rm main 
	rm *.rlib
