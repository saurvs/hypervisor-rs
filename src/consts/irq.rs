//! Interrupt Request (IRQ) Codes

use libc::*;

pub const IRQ_INFO_EXT_IRQ      : uint32_t = 0 << 8;
pub const IRQ_INFO_NMI          : uint32_t = 2 << 8;
pub const IRQ_INFO_HARD_EXC     : uint32_t = 3 << 8;
pub const IRQ_INFO_SOFT_IRQ     : uint32_t = 4 << 8;
pub const IRQ_INFO_PRIV_SOFT_EXC: uint32_t = 5 << 8;
pub const IRQ_INFO_SOFT_EXC     : uint32_t = 6 << 8;
pub const IRQ_INFO_ERROR_VALID  : uint32_t = 1 << 11;
pub const IRQ_INFO_VALID        : uint32_t = 1 << 31;
