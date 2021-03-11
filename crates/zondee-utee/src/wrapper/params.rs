// https://github.com/mesalock-linux/rust-optee-trustzone-sdk/blob/master/optee-utee/src/param.rs

use crate::wrapper::{self, raw, TaErrorCode};
use core::{marker::PhantomData, slice};

pub struct Param {
    pub param_type: ParamType,
    pub raw: *mut raw::TEE_Param,
}

impl Param {
    pub fn from_raw(ptr: *mut raw::TEE_Param, param_type: ParamType) -> Self {
        Self {
            raw: ptr,
            param_type,
        }
    }

    pub unsafe fn as_value(&mut self) -> wrapper::Result<ParamValue> {
        match self.param_type {
            ParamType::ValueInput | ParamType::ValueInout | ParamType::ValueOutput => {
                Ok(ParamValue {
                    raw: &mut (*self.raw).value,
                    param_type: self.param_type,
                    phantom: PhantomData,
                })
            }
            _ => Err(TaErrorCode::BadParameters),
        }
    }

    pub unsafe fn as_memref(&mut self) -> wrapper::Result<ParamMemRef> {
        match self.param_type {
            ParamType::MemRefInout | ParamType::MemRefInput | ParamType::MemRefOutput => {
                Ok(ParamMemRef {
                    raw: &mut (*self.raw).memref,
                    param_type: self.param_type,
                    phantom: PhantomData,
                })
            }
            _ => Err(TaErrorCode::BadParameters),
        }
    }

    pub fn raw(&self) -> *mut raw::TEE_Param {
        self.raw
    }
}

pub struct ParamMemRef<'a> {
    param_type: ParamType,
    phantom: PhantomData<&'a mut ()>,
    raw: *mut raw::MemRef,
}

impl ParamMemRef<'_> {
    pub fn buffer(&mut self) -> &mut [u8] {
        unsafe {
            slice::from_raw_parts_mut((*self.raw).buffer as *mut u8, (*self.raw).size as usize)
        }
    }

    pub fn param_type(&self) -> ParamType {
        self.param_type
    }

    pub fn raw(&mut self) -> *mut raw::MemRef {
        self.raw
    }

    pub fn set_updated_size(&mut self, size: usize) {
        unsafe { (*self.raw).size = size as u32 };
    }
}

pub struct ParamValue<'a> {
    raw: *mut raw::Value,
    param_type: ParamType,
    phantom: PhantomData<&'a mut ()>,
}

impl ParamValue<'_> {
    pub fn a(&self) -> u32 {
        unsafe { (*self.raw).a }
    }

    pub fn b(&self) -> u32 {
        unsafe { (*self.raw).b }
    }

    pub fn param_type(&self) -> ParamType {
        self.param_type
    }

    pub fn set_a(&mut self, a: u32) {
        unsafe {
            (*self.raw).a = a;
        }
    }

    pub fn set_b(&mut self, b: u32) {
        unsafe {
            (*self.raw).b = b;
        }
    }
}

pub struct Parameters(pub Param, pub Param, pub Param, pub Param);

impl Parameters {
    pub fn from_raw(tee_params: &mut [raw::TEE_Param; 4], n: u32) -> Self {
        let [f0, f1, f2, f3] = params_type(n);
        let p0 = Param::from_raw(&mut tee_params[0], f0);
        let p1 = Param::from_raw(&mut tee_params[1], f1);
        let p2 = Param::from_raw(&mut tee_params[2], f2);
        let p3 = Param::from_raw(&mut tee_params[3], f3);
        Parameters(p0, p1, p2, p3)
    }
}

#[derive(Copy, Clone)]
#[repr(u32)]
pub enum ParamType {
    None = 0,
    ValueInput = 1,
    ValueOutput = 2,
    ValueInout = 3,
    MemRefInput = 5,
    MemRefOutput = 6,
    MemRefInout = 7,
}

impl From<u32> for ParamType {
    fn from(value: u32) -> Self {
        match value {
            0 => ParamType::None,
            1 => ParamType::ValueInput,
            2 => ParamType::ValueOutput,
            3 => ParamType::ValueInout,
            5 => ParamType::MemRefInput,
            6 => ParamType::MemRefOutput,
            7 => ParamType::MemRefInout,
            _ => ParamType::None,
        }
    }
}

pub fn params_type(n: u32) -> [ParamType; 4] {
    [
        (0x000fu32 & n).into(),
        ((0x00f0u32 & n) >> 4).into(),
        ((0x0f00u32 & n) >> 8).into(),
        ((0xf000u32 & n) >> 12).into(),
    ]
}
