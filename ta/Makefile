 .PHONY: all cclean clean copy
default: all

include ../proj.mk
include ./src/include/uuid.mk

current_dir = $(shell pwd)

all:
	cd $(SRC_)/$(TA) && cargo build --target $(RUST_TARGET) --release --target-dir $(current_dir)/target
	$(MAKE) -C src CROSS_COMPILE="$(TA_CROSS_COMPILE)" LDFLAGS=""

clean: cclean
	cd $(SRC_)/$(TA) && cargo clean --target-dir $(current_dir)/target
	- rm $(SHARED_FOLDER)/$(TA_UUID).ta

cclean:
	$(MAKE) -C src clean

copy: all
	cp src/$(TA_UUID).ta $(SHARED_FOLDER)
