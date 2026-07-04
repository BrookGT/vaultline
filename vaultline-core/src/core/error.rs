//! Shared error type for vaultline parsers and validators.


use core::fmt;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    UnexpectedEof,
    Unexpected { expected: &'static str, at: usize },
    LengthOverflow { field: &'static str },
    DepthLimit,
    OutOfRange { what: &'static str },
    InvalidEncoding { scheme: &'static str },
    StaleHandle,
    Protocol { detail: &'static str },
    CapacityLimit,
    SealFailed { detail: &'static str },
    VerifyDeferred { detail: &'static str },
    XrefBroken { from: u32, to: u32 },
    RotationGap { expected: u64, found: u64 },
}

impl Error {
    pub fn unexpected(expected: &'static str, at: usize) -> Self {
        Error::Unexpected { expected, at }
    }
    pub fn protocol(detail: &'static str) -> Self {
        Error::Protocol { detail }
    }
    pub fn seal(detail: &'static str) -> Self {
        Error::SealFailed { detail }
    }
    pub fn code(&self) -> &'static str {
        match self {
            Error::UnexpectedEof => "eof",
            Error::Unexpected { .. } => "unexpected",
            Error::LengthOverflow { .. } => "length_overflow",
            Error::DepthLimit => "depth_limit",
            Error::OutOfRange { .. } => "out_of_range",
            Error::InvalidEncoding { .. } => "invalid_encoding",
            Error::StaleHandle => "stale_handle",
            Error::Protocol { .. } => "protocol",
            Error::CapacityLimit => "capacity_limit",
            Error::SealFailed { .. } => "seal_failed",
            Error::VerifyDeferred { .. } => "verify_deferred",
            Error::XrefBroken { .. } => "xref_broken",
            Error::RotationGap { .. } => "rotation_gap",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::UnexpectedEof => write!(f, "unexpected end of input"),
            Error::Unexpected { expected, at } => {
                write!(f, "unexpected at {at}: expected {expected}")
            }
            Error::LengthOverflow { field } => write!(f, "length overflow in {field}"),
            Error::DepthLimit => write!(f, "depth limit exceeded"),
            Error::OutOfRange { what } => write!(f, "{what} out of range"),
            Error::InvalidEncoding { scheme } => write!(f, "invalid {scheme}"),
            Error::StaleHandle => write!(f, "stale buffer handle"),
            Error::Protocol { detail } => write!(f, "protocol: {detail}"),
            Error::CapacityLimit => write!(f, "capacity limit"),
            Error::SealFailed { detail } => write!(f, "seal failed: {detail}"),
            Error::VerifyDeferred { detail } => write!(f, "deferred verify: {detail}"),
            Error::XrefBroken { from, to } => write!(f, "xref broken {from}->{to}"),
            Error::RotationGap { expected, found } => {
                write!(f, "rotation gap expected {expected} found {found}")
            }
        }
    }
}

impl From<core::str::Utf8Error> for Error {
    fn from(_: core::str::Utf8Error) -> Self {
        Error::InvalidEncoding { scheme: "utf8" }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn codes() {
        assert_eq!(Error::StaleHandle.code(), "stale_handle");
    }
}
