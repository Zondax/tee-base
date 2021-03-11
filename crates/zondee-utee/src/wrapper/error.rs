// https://githubcom/mesalock-linux/rust-optee-trustzone-sdk/blob/master/optee-utee/src/errorrs

#[derive(Debug)]
#[repr(u32)]
pub enum TaErrorCode {
    /// Non-specific cause.                                                                                                       
    Generic = 0xFFFF0000,
    /// Access privileges are not sufficient.                                                                                     
    AccessDenied = 0xFFFF0001,
    /// The operation was canceled.                                                                                               
    Cancel = 0xFFFF0002,
    /// Concurrent accesses caused conflict.                                                                                      
    AccessConflict = 0xFFFF0003,
    /// Too much data for the requested operation was passed.                                                                     
    ExcessData = 0xFFFF0004,
    /// Input data was of invalid format.                                                                                         
    BadFormat = 0xFFFF0005,
    /// Input parameters were invalid.                                                                                            
    BadParameters = 0xFFFF0006,
    /// Operation is not valid in the current state.                                                                              
    BadState = 0xFFFF0007,
    /// The requested data item is not found.                                                                                     
    ItemNotFound = 0xFFFF0008,
    /// The requested operation should exist but is not yet implemented.                                                          
    NotImplemented = 0xFFFF0009,
    /// The requested operation is valid but is not supported in this implementation.                                             
    NotSupported = 0xFFFF000A,
    /// Expected data was missing.                                                                                                
    NoData = 0xFFFF000B,
    /// System ran out of resources.                                                                                              
    OutOfMEmory = 0xFFFF000C,
    /// The system is busy working on something else.                                                                             
    Busy = 0xFFFF000D,
    /// Communication with a remote party failed.                                                                                 
    Communication = 0xFFFF000E,
    /// A security fault was detected.                                                                                            
    Security = 0xFFFF000F,
    /// The supplied buffer is too short for the generated output.                                                                
    ShortBuffer = 0xFFFF0010,
    /// Implementation defined error code.                                                                                        
    ExternalCancel = 0xFFFF0011,
    /// Implementation defined error code: trusted Application has panicked during the operation.                                 
    TargetDead = 0xFFFF3024,
    /// Public key type is not supported
    KeyNotSupported = 0xFFFF3025,
    /// Pair not found for public key and KeyTypeId
    PairNotFound = 0xFFFF3026,
    /// Validation error
    ValidationError = 0xFFFF3027,
    /// Keystore unavailable
    Unavailable = 0xFFFF3028,
    /// Unknown error.   
    Unknown = 0xFFFF3029,
}

impl From<u32> for TaErrorCode {
    fn from(code: u32) -> Self {
        match code {
            0xFFFF0001..=0xFFFF0011 => unsafe { core::mem::transmute(code) },
            0xFFFF3024..=0xFFFF3028 => unsafe { core::mem::transmute(code) },
            _ => Self::Unknown,
        }
    }
}

impl TaErrorCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            TaErrorCode::Generic => "Non-specific cause.",
            TaErrorCode::AccessDenied => "Access privileges are not sufficient.",
            TaErrorCode::Cancel => "The operation was canceled.",
            TaErrorCode::AccessConflict => "Concurrent accesses caused conflict.",
            TaErrorCode::ExcessData => "Too much data for the requested operation was passed.",
            TaErrorCode::BadFormat => "Input data was of invalid format.",
            TaErrorCode::BadParameters => "Input parameters were invalid.",
            TaErrorCode::BadState => "Operation is not valid in the current state.",
            TaErrorCode::ItemNotFound => "The requested data item is not found.",
            TaErrorCode::NotImplemented => {
                "The requested operation should exist but is not yet implemented."
            }
            TaErrorCode::NotSupported => {
                "The requested operation is valid but is not supported in this implementation."
            }
            TaErrorCode::NoData => "Expected data was missing.",
            TaErrorCode::OutOfMEmory => "System ran out of resources.",
            TaErrorCode::Busy => "The system is busy working on something else.",
            TaErrorCode::Communication => "Communication with a remote party failed.",
            TaErrorCode::Security => "A security fault was detected.",
            TaErrorCode::ShortBuffer => {
                "The supplied buffer is too short for the generated output."
            }
            TaErrorCode::ExternalCancel => "Undocumented.",
            TaErrorCode::TargetDead => "Trusted Application has panicked during the operation.",

            TaErrorCode::KeyNotSupported => "Key not supported",
            TaErrorCode::PairNotFound => "Pair was not found",
            TaErrorCode::ValidationError => "Validation error",
            TaErrorCode::Unavailable => "Keystore unavailable",
            TaErrorCode::Unknown => "Unknown error.",
        }
    }
}
