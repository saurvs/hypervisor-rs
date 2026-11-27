//! Error type shared by both architectures.

use core::fmt;

/// An error returned by Hypervisor.framework (`hv_return_t`).
///
/// Every safe function in this crate returns [`Result`], where the error
/// carries the decoded framework return code. The raw value is always
/// available through [`Error::code`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Error {
    /// `HV_ERROR`: unspecified failure.
    Generic,
    /// `HV_BUSY`: the resource is busy (for example, a VM already exists in
    /// this process).
    Busy,
    /// `HV_BAD_ARGUMENT`: an argument was invalid (for example, an address or
    /// size that is not page aligned).
    BadArgument,
    /// `HV_ILLEGAL_GUEST_STATE`: the vCPU state is not valid for the
    /// requested operation.
    #[cfg(target_arch = "aarch64")]
    IllegalGuestState,
    /// `HV_NO_RESOURCES`: the system is out of resources.
    NoResources,
    /// `HV_NO_DEVICE`: no hypervisor device is available.
    NoDevice,
    /// `HV_DENIED`: the process is not allowed to use the hypervisor. On
    /// macOS 11 and later this usually means the binary is not signed with
    /// the `com.apple.security.hypervisor` entitlement.
    Denied,
    /// `HV_EXISTS`: the object already exists.
    #[cfg(target_arch = "aarch64")]
    Exists,
    /// `HV_FAULT`: a fault occurred.
    #[cfg(target_arch = "x86_64")]
    Fault,
    /// `HV_UNSUPPORTED`: the operation is not supported on this host (for
    /// example, no hardware virtualization support, or running inside a VM
    /// without nested virtualization).
    Unsupported,
    /// A return code this crate does not know about.
    Unknown(u32),
}

/// Result type used throughout this crate.
pub type Result<T> = core::result::Result<T, Error>;

const HV_SUCCESS: u32 = 0;
const HV_ERROR: u32 = 0xfae9_4001;
const HV_BUSY: u32 = 0xfae9_4002;
const HV_BAD_ARGUMENT: u32 = 0xfae9_4003;
#[cfg(target_arch = "aarch64")]
const HV_ILLEGAL_GUEST_STATE: u32 = 0xfae9_4004;
const HV_NO_RESOURCES: u32 = 0xfae9_4005;
const HV_NO_DEVICE: u32 = 0xfae9_4006;
const HV_DENIED: u32 = 0xfae9_4007;
// The same value is called HV_EXISTS on arm64 and HV_FAULT on x86_64.
const HV_EXISTS_OR_FAULT: u32 = 0xfae9_4008;
const HV_UNSUPPORTED: u32 = 0xfae9_400f;

impl Error {
    /// Converts a raw `hv_return_t` into a `Result`.
    ///
    /// `HV_SUCCESS` maps to `Ok(())`; anything else maps to an `Error`.
    pub fn check(code: u32) -> Result<()> {
        match code {
            HV_SUCCESS => Ok(()),
            other => Err(Self::from_code(other)),
        }
    }

    fn from_code(code: u32) -> Self {
        match code {
            HV_ERROR => Self::Generic,
            HV_BUSY => Self::Busy,
            HV_BAD_ARGUMENT => Self::BadArgument,
            #[cfg(target_arch = "aarch64")]
            HV_ILLEGAL_GUEST_STATE => Self::IllegalGuestState,
            HV_NO_RESOURCES => Self::NoResources,
            HV_NO_DEVICE => Self::NoDevice,
            HV_DENIED => Self::Denied,
            #[cfg(target_arch = "aarch64")]
            HV_EXISTS_OR_FAULT => Self::Exists,
            #[cfg(target_arch = "x86_64")]
            HV_EXISTS_OR_FAULT => Self::Fault,
            HV_UNSUPPORTED => Self::Unsupported,
            other => Self::Unknown(other),
        }
    }

    /// Returns the raw `hv_return_t` value for this error.
    pub fn code(&self) -> u32 {
        match *self {
            Self::Generic => HV_ERROR,
            Self::Busy => HV_BUSY,
            Self::BadArgument => HV_BAD_ARGUMENT,
            #[cfg(target_arch = "aarch64")]
            Self::IllegalGuestState => HV_ILLEGAL_GUEST_STATE,
            Self::NoResources => HV_NO_RESOURCES,
            Self::NoDevice => HV_NO_DEVICE,
            Self::Denied => HV_DENIED,
            #[cfg(target_arch = "aarch64")]
            Self::Exists => HV_EXISTS_OR_FAULT,
            #[cfg(target_arch = "x86_64")]
            Self::Fault => HV_EXISTS_OR_FAULT,
            Self::Unsupported => HV_UNSUPPORTED,
            Self::Unknown(code) => code,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match *self {
            Self::Generic => "error",
            Self::Busy => "busy",
            Self::BadArgument => "bad argument",
            #[cfg(target_arch = "aarch64")]
            Self::IllegalGuestState => "illegal guest state",
            Self::NoResources => "no resources",
            Self::NoDevice => "no device",
            Self::Denied => {
                "denied (is the binary signed with the com.apple.security.hypervisor entitlement?)"
            }
            #[cfg(target_arch = "aarch64")]
            Self::Exists => "already exists",
            #[cfg(target_arch = "x86_64")]
            Self::Fault => "fault",
            Self::Unsupported => "unsupported",
            Self::Unknown(code) => {
                return write!(f, "unknown Hypervisor.framework error {code:#x}");
            }
        };
        write!(f, "hypervisor: {msg}")
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success_is_ok() {
        assert_eq!(Error::check(0), Ok(()));
    }

    #[test]
    fn codes_round_trip() {
        for code in [
            0xfae9_4001,
            0xfae9_4002,
            0xfae9_4003,
            0xfae9_4004,
            0xfae9_4005,
            0xfae9_4006,
            0xfae9_4007,
            0xfae9_4008,
            0xfae9_400f,
            0x1234,
        ] {
            let err = Error::check(code).unwrap_err();
            assert_eq!(err.code(), code);
        }
        assert_eq!(Error::check(0xfae9_4007), Err(Error::Denied));
        assert_eq!(Error::check(0x1234), Err(Error::Unknown(0x1234)));
    }

    #[test]
    fn display_mentions_entitlement_for_denied() {
        assert!(Error::Denied.to_string().contains("entitlement"));
    }
}
