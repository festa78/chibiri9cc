ROOT_DIR:=$(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))

all: clean test

./target/debug/chibiri9cc: src/*.rs
	docker run -v $(ROOT_DIR):/work -w /work --rm -it chibiri9cc_linux /bin/bash -c "/root/.cargo/bin/cargo build"

test: ./target/debug/chibiri9cc
	docker run -v $(ROOT_DIR):/work -w /work --rm -it chibiri9cc_linux /bin/bash -c "./test.sh"

clean:
	docker run -v $(ROOT_DIR):/work -w /work --rm -it chibiri9cc_linux /bin/bash -c "/root/.cargo/bin/cargo clean"
	rm -f *.o *~ tmp*

docker_build:
	docker build -t chibiri9cc_linux .

.PHONY: all test clean
