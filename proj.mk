#set common environment variables betwen the framework makefiles
export PROJ_NAME ?= rustee_app

#folder inside src that contains the Host source
export HOST ?= REE
#folder inside serc that contains the TA source
export TA ?= TEE

# Set environment according to /optee/build/common.mk & specific

ifdef QEMU_V8
#qemu v8 set
ifdef QEMU_V7
#qemu v7 also set?
$(error Please set EITHER QEMU_V8 OR QEMU_V7 but not both)
endif
endif

ifdef QEMU_V7
export RUST_TARGET ?= armv7-unknown-linux-gnueabihf
export OPTEE ?= $(HOME)/optee
export TEEC_EXPORT ?= $(OPTEE)/out-br/host/arm-buildroot-linux-gnueabihf/sysroot
export HOST_CROSS_COMPILE ?= $(OPTEE)/toolchains/aarch32/bin/arm-linux-gnueabihf-
export TA_CROSS_COMPILE ?= $(HOST_CROSS_COMPILE)
export TA_DEV_KIT_DIR ?= $(OPTEE)/optee_os/out/arm/export-ta_arm32
export OVERRIDE_SYSROOT ?= 1
endif

ifdef QEMU_V8
export RUST_TARGET ?= aarch64-unknown-linux-gnu
export OPTEE ?= $(HOME)/optee
export TEEC_EXPORT ?= $(OPTEE)/out-br/host/aarch64-buildroot-linux-gnu/sysroot
export HOST_CROSS_COMPILE = $(OPTEE)/toolchains/aarch64/bin/aarch64-linux-gnu-
export TA_CROSS_COMPILE ?= $(HOST_CROSS_COMPILE)
export TA_DEV_KIT_DIR ?= $(OPTEE)/optee_os/out/arm/export-ta_arm64
export OVERRIDE_SYSROOT ?= 1
endif


export V?=0
export UTEE_ROOT=$(TA_DEV_KIT_DIR)
export TEEC_ROOT=$(TEEC_EXPORT)/usr

#QEMU CONFIGURATION
export QEMU_VIRTFS_ENABLE = y
export QEMU_VIRTFS_AUTOMOUNT = y
export QEMU_VIRTFS_HOST_DIR = $(SHARED_FOLDER)
export HOSTFWD = ,hostfwd=tcp::10710-:39946
