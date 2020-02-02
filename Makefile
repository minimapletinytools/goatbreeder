
animalclub:
	$(MAKE) -C animalclub
	cp animalclub/csrc/animalclub.h ./
	cp animalclub/ctest/libanimalclub.* ./

clean:
	cd animalclub && make clean

.PHONY: animalclub
