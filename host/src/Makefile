.PHONY: all clean

CC      ?= $(CROSS_COMPILE)gcc
LD      ?= $(CROSS_COMPILE)ld
AR      ?= $(CROSS_COMPILE)ar
NM      ?= $(CROSS_COMPILE)nm
OBJCOPY ?= $(CROSS_COMPILE)objcopy
OBJDUMP ?= $(CROSS_COMPILE)objdump
READELF ?= $(CROSS_COMPILE)readelf

CFLAGS 	+= -Wall -I../../ta/src/include -I$(TEEC_EXPORT)/usr/include -I./include -fPIC
CFLAGS 	+= -I$(SRC_)/$(HOST)/lib/include

LDFLAGS += --hash-style=gnu
LDLIBS 	+= -lteec -L$(TEEC_EXPORT)/lib
LDLIBS 	+= -lrustee_host
LDLIBS 	+= -lpthread
LDLIBS 	+= -lm
LDLIBS 	+= -ldl

LDLIBS 	+= -L../target/${RUST_TARGET}/release

ifdef OVERRIDE_SYSROOT
LDLIBS 	+= --sysroot=$(TEEC_EXPORT)
endif

all: $(PROJ_NAME)

OBJECT_FILES = main.o appmain.o

%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@

$(PROJ_NAME): $(OBJECT_FILES)
	$(CC) -o $@ $(OBJECT_FILES) $(LDLIBS)

clean:
	rm -f $(PROJ_NAME) $(OBJECT_FILES)
