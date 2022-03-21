./target/debug/chibiri9cc: src/*.rs
	cargo build

test: ./target/debug/chibiri9cc
	./test.sh

clean:
	cargo clean
	rm -f *.o *~ tmp*

.PHONY: test clean