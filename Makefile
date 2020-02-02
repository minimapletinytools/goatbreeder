all: animalclub hello_cargo ccall

animalclub:
	$(MAKE) -C animalclub

copyanimalclubdeps: animalclub
	cp animalclub/ctest/libanimalclub.* ./practice_rust_project/ccall

hello_cargo:
	$(MAKE) -C practice_rust_project

ccall: copyanimalclubdeps
	cd practice_rust_project/ccall && cargo build

clean:
	cd animalclub && make clean

.PHONY: animalclub copyanimalclubdeps hello_cargo clean
