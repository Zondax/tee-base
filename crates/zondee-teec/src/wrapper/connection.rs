use crate::wrapper::{
    self,
    raw::{
        TEEC_CloseSession, TEEC_Context, TEEC_FinalizeContext, TEEC_InitializeContext,
        TEEC_InvokeCommand, TEEC_OpenSession, TEEC_Operation, TEEC_Session, TEEC_LOGIN_PUBLIC,
        TEEC_SUCCESS,
    },
    Operation,
};
use core::ptr::null_mut;

#[derive(Debug)]
pub struct Connection {
    ctx: TEEC_Context,
    sess: TEEC_Session,
    return_origin: u32,
}

impl Connection {
    pub fn new(mut ctx: TEEC_Context, name: &str, sess: TEEC_Session) -> crate::Result<Self> {
        Self::initialize_context(name, &mut ctx)?;
        Ok(Self {
            ctx,
            return_origin: u32::default(),
            sess,
        })
    }

    pub fn open_session(
        &mut self,
        destination: wrapper::Uuid,
        operation: &mut TEEC_Operation,
    ) -> crate::Result<()> {
        let rslt = unsafe {
            TEEC_OpenSession(
                &mut self.ctx,
                &mut self.sess,
                destination.as_ptr(),
                TEEC_LOGIN_PUBLIC,
                null_mut(),
                operation,
                &mut self.return_origin,
            )
        };
        if rslt == TEEC_SUCCESS {
            Ok(())
        } else {
            Err(crate::Error::ConnectionCodeWithOrigin(
                rslt,
                self.return_origin,
            ))
        }
    }

    pub fn invoke_command<A, B, C, D>(
        &mut self,
        id: u32,
        op: &mut Operation<A, B, C, D>,
    ) -> crate::Result<()> {
        let rslt = unsafe {
            TEEC_InvokeCommand(&mut self.sess, id, op.as_mut_ptr(), &mut self.return_origin)
        };
        if rslt == TEEC_SUCCESS {
            Ok(())
        } else {
            Err(crate::Error::ConnectionCodeWithOrigin(
                rslt,
                self.return_origin,
            ))
        }
    }

    fn initialize_context(name: &str, ctx: &mut TEEC_Context) -> crate::Result<()> {
        let rslt = unsafe { TEEC_InitializeContext(name.as_ptr() as *const _, ctx as *mut _) };
        if rslt == TEEC_SUCCESS {
            Ok(())
        } else {
            Err(crate::Error::ConnectionCode(rslt))
        }
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        unsafe {
            TEEC_CloseSession(&mut self.sess as *mut _);
            TEEC_FinalizeContext(&mut self.ctx as *mut _);
        }
    }
}
