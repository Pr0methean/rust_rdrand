use core::{
    fmt::{self, Display, Formatter},
};

/// Errors in this library
#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum ErrorCode {
    /// The hardware instruction is not supported
    UnsupportedInstruction,
    /// There was a hardware failure
    HardwareFailure,
}

impl ErrorCode {
    const fn as_randcore_code(self) -> core::num::NonZeroU32 {
        /// Arbitrary, off top of head bitmask for error codes that come from rdrand
        const RDRAND_TAG: u32 = rand_core::Error::CUSTOM_START + 0x3D34_7D00;
        unsafe { core::num::NonZeroU32::new_unchecked(RDRAND_TAG + self as u32) }
    }
}

impl core::error::Error for ErrorCode {}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(match self {
            ErrorCode::UnsupportedInstruction => "the hardware instruction is not supported",
            ErrorCode::HardwareFailure => "hardware generator failure",
        })
    }
}

#[cfg(test)]
mod test {
    use super::ErrorCode;
    use core::error::Error;

    #[test]
    fn error_code_send() {
        fn assert_send<T: Send>() {}
        assert_send::<ErrorCode>();
    }

    #[test]
    fn error_code_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<ErrorCode>();
    }

    #[test]
    fn error_code_copy() {
        fn assert_copy<T: Copy>() {}
        assert_copy::<ErrorCode>();
    }

    #[test]
    fn error_code_clone() {
        fn assert_clone<T: Clone>() {}
        assert_clone::<ErrorCode>();
    }

    #[test]
    #[cfg(feature = "std")]
    fn error_code_error() {
        fn assert_error<T: Error>() {}
        assert_error::<ErrorCode>();
    }
}
