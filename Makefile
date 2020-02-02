all: animalclub hello_cargo

animalclub:
	$(MAKE) -C animalclub

copyanimalclubdeps: animalclub
	cp animalclub/csrc/animalclub.h ./
	cp animalclub/ctest/libanimalclub.* ./

hello_cargo: copyanimalclubdeps
	$(MAKE) -C practice_rust_project

clean:
	cd animalclub && make clean

.PHONY: animalclub copyanimalclubdeps hello_cargo clean
