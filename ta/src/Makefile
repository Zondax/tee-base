.PHONY: clean new-uuid

include include/uuid.mk

BINARY=$(TA_UUID)

CFG_TEE_TA_LOG_LEVEL ?= 4
CFG_TEE_CORE_LOG_LEVEL ?= 4
# TODO: CFG_ENCRYPT_TA
# TODO: TA_ENC_KEY

CPPFLAGS += -DCFG_TEE_TA_LOG_LEVEL=$(CFG_TEE_TA_LOG_LEVEL) -DCFG_TEE_CORE_LOG_LEVEL=$(CFG_TEE_CORE_LOG_LEVEL)
-include $(TA_DEV_KIT_DIR)/mk/ta_dev_kit.mk

CFLAGS 	+= -I$(SRC_)/$(TA)/lib/include

SIGN_SCRIPT = python3 sign_encrypt.py
DIGEST = $(TA_UUID).dig
STRIPPED = $(TA_UUID).stripped.elf
SIGNATURE = $(TA_UUID).sig

ifeq ($(wildcard $(TA_DEV_KIT_DIR)/mk/ta_dev_kit.mk), )
clean:
	@echo 'Note: $$(TA_DEV_KIT_DIR)/mk/ta_dev_kit.mk not found, cannot clean TA'
endif

sign_ta:
ifdef SIGN_TA
        # Make a copy of the TA
	    cp $(TA_UUID).ta $(TA_UUID).ta.plain
	    $(SIGN_SCRIPT) digest --key $(TA_SIGN_KEY) \
		   --uuid $(TA_UUID) --in $(STRIPPED)
	    base64 -d $(DIGEST) | \
		   openssl pkeyutl -sign -inkey $(TA_SIGN_KEY) \
		           -pkeyopt digest:sha256 -pkeyopt rsa_padding_mode:pss \
		            -pkeyopt rsa_pss_saltlen:digest -pkeyopt rsa_mgf1_md:sha256 | \
		   base64 > $(SIGNATURE)
	    $(SIGN_SCRIPT) stitch --key $(TA_SIGN_KEY) \
		   --uuid $(TA_UUID) --in $(STRIPPED) --algo TEE_ALG_RSASSA_PKCS1_PSS_MGF1_SHA256 \
           --pubkey $(TA_PUB_KEY)
endif

new-uuid:
	python newuuid.py
