.PHONY: all cclean clean copy
default: all

include ../proj.mk

current_dir = $(shell pwd)

all:
	cd $(SRC_)/$(HOST) && cargo build --target $(RUST_TARGET) --release --target-dir $(current_dir)/target
	$(MAKE) -C src CROSS_COMPILE="$(HOST_CROSS_COMPILE)" --no-builtin-variables

clean: cclean
	cd $(SRC_)/$(HOST) && cargo clean --target-dir $(current_dir)/target
	- rm $(SHARED_FOLDER)/$(PROJ_NAME)

cclean:
	$(MAKE) -C src clean

copy: all
	cp src/$(PROJ_NAME) $(SHARED_FOLDER)
