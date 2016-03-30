//! Some useful constants

pub mod vmcs;
pub mod vmx_cap;
pub mod vmx_exit;
pub mod irq;

use libc::*;

pub const VMX_BASIC_TRUE_CTLS: uint64_t = 1 << 55;
