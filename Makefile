all: animalclub ccall

animalclub:
	$(MAKE) -C animalclub

copyanimalclubdeps: animalclub
	cp animalclub/ctest/libanimalclub.* ./src/ccall

ccall: copyanimalclubdeps
	cd src/ccall && CXX=$(which clang++) cargo build

run: ccall
	cd src/ccall && CXX=$(which clang++) cargo run

clean:
	cd animalclub && make clean

.PHONY: animalclub copyanimalclubdeps ccall ccall-run clean
