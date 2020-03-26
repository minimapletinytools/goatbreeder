all: animalclub hello_cargo ccall

animalclub:
	$(MAKE) -C animalclub

copyanimalclubdeps: animalclub
	cp animalclub/ctest/libanimalclub.* ./src/ccall

ccall: copyanimalclubdeps
	cd src/ccall && cargo build

ccall-run: ccall
	cd src/ccall && cargo run

clean:
	cd animalclub && make clean

.PHONY: animalclub copyanimalclubdeps ccall ccall-run clean
