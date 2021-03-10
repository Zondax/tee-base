#include <tee_internal_api.h>
#include <tee_internal_api_extensions.h>

#include <librustee_ta.h>
#include <rustee_ta.h>

TEE_Result TA_CreateEntryPoint(void) {
  DMSG("Create entry point\n");

  return RUSTEE_Create();
}

void TA_DestroyEntryPoint(void) {
  DMSG("Destroy entry point\n");

  return RUSTEE_Destroy();
}

TEE_Result TA_OpenSessionEntryPoint(uint32_t param_types, TEE_Param params[4],
                                    void **sess_ctx) {
  DMSG("Open Session entry point\n");

  return RUSTEE_OpenSession(param_types, params, sess_ctx);
}

void TA_CloseSessionEntryPoint(void *sess_ctx) {
  DMSG("Close Session entry point\n");

  return RUSTEE_CloseSession(sess_ctx);
}

TEE_Result TA_InvokeCommandEntryPoint(void *sess_ctx, uint32_t cmd_id,
                                      uint32_t param_types,
                                      TEE_Param params[4]) {
  DMSG("Invoke Command entry point\n");

  return RUSTEE_InvokeCommand(sess_ctx, cmd_id, param_types, params);
}
