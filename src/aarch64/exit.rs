//! vCPU exit information and exception syndrome decoding.

use core::fmt;

use super::SysReg;
use super::ffi;

/// Why [`Vcpu::run`](crate::Vcpu::run) returned.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum VcpuExit {
    /// The exit was requested with [`VcpuHandle::exit`](crate::VcpuHandle::exit)
    /// or [`exit_vcpus`](crate::exit_vcpus).
    Canceled,
    /// The guest triggered a synchronous exception that the hypervisor traps
    /// (for example `hvc`, a data abort on unmapped memory, or a trapped
    /// system register access).
    Exception(Exception),
    /// The virtual timer fired. The framework masks the timer automatically;
    /// make the timer interrupt pending in the guest and clear the mask with
    /// [`Vcpu::set_vtimer_mask`](crate::Vcpu::set_vtimer_mask) once the guest
    /// has handled it.
    VtimerActivated,
    /// The framework could not determine the exit reason.
    Unknown,
}

impl VcpuExit {
    pub(crate) fn from_raw(raw: &ffi::hv_vcpu_exit_t) -> Self {
        match raw.reason {
            ffi::HV_EXIT_REASON_CANCELED => Self::Canceled,
            ffi::HV_EXIT_REASON_EXCEPTION => Self::Exception(Exception {
                syndrome: raw.exception.syndrome,
                virtual_address: raw.exception.virtual_address,
                physical_address: raw.exception.physical_address,
            }),
            ffi::HV_EXIT_REASON_VTIMER_ACTIVATED => Self::VtimerActivated,
            _ => Self::Unknown,
        }
    }
}

/// A guest exception that caused an exit.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Exception {
    /// Exception syndrome (`ESR_EL2`).
    pub syndrome: u64,
    /// Faulting virtual address (`FAR_EL2`), when applicable.
    pub virtual_address: u64,
    /// Faulting guest physical (intermediate physical) address, when applicable.
    pub physical_address: u64,
}

impl Exception {
    /// The exception class (`ESR_EL2.EC`).
    pub fn class(&self) -> ExceptionClass {
        ExceptionClass::from_ec(((self.syndrome >> 26) & 0x3f) as u8)
    }

    /// The instruction specific syndrome (`ESR_EL2.ISS`).
    pub fn iss(&self) -> u32 {
        (self.syndrome & 0x01ff_ffff) as u32
    }

    /// Whether the trapped instruction was 32 bits long (`ESR_EL2.IL`).
    pub fn is_32bit_instruction(&self) -> bool {
        self.syndrome & (1 << 25) != 0
    }

    /// The 16-bit immediate of a trapped `hvc`, `smc`, `svc` or `brk`.
    pub fn immediate(&self) -> Option<u16> {
        match self.class() {
            ExceptionClass::Svc64
            | ExceptionClass::Hvc64
            | ExceptionClass::Smc64
            | ExceptionClass::Brk64 => Some(self.iss() as u16),
            _ => None,
        }
    }

    /// Decodes a data abort (for example a guest access to unmapped memory,
    /// which is how MMIO is usually emulated).
    pub fn data_abort(&self) -> Option<DataAbort> {
        if self.class() != ExceptionClass::DataAbortLowerEl {
            return None;
        }
        let iss = self.iss();
        Some(DataAbort {
            valid: iss & (1 << 24) != 0,
            size: 1 << ((iss >> 22) & 0b11),
            sign_extend: iss & (1 << 21) != 0,
            register: ((iss >> 16) & 0x1f) as u8,
            sixty_four: iss & (1 << 15) != 0,
            write: iss & (1 << 6) != 0,
            fault_status: (iss & 0x3f) as u8,
            address: self.physical_address,
        })
    }

    /// Decodes a trapped `msr`/`mrs` system register access.
    pub fn sys_reg_access(&self) -> Option<SysRegAccess> {
        if self.class() != ExceptionClass::SysReg64 {
            return None;
        }
        let iss = self.iss();
        let op0 = (iss >> 20) & 0b11;
        let op2 = (iss >> 17) & 0b111;
        let op1 = (iss >> 14) & 0b111;
        let crn = (iss >> 10) & 0b1111;
        let rt = (iss >> 5) & 0x1f;
        let crm = (iss >> 1) & 0b1111;
        let encoding = (op0 << 14) | (op1 << 11) | (crn << 7) | (crm << 3) | op2;
        Some(SysRegAccess {
            reg: SysReg(encoding as u16),
            register: rt as u8,
            read: iss & 1 != 0,
        })
    }
}

impl fmt::Debug for Exception {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Exception")
            .field("class", &self.class())
            .field("syndrome", &format_args!("{:#x}", self.syndrome))
            .field(
                "virtual_address",
                &format_args!("{:#x}", self.virtual_address),
            )
            .field(
                "physical_address",
                &format_args!("{:#x}", self.physical_address),
            )
            .finish()
    }
}

/// The class of a guest exception (`ESR_EL2.EC`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ExceptionClass {
    /// `0x00`: unknown reason.
    Unknown,
    /// `0x01`: trapped `wfi` or `wfe`.
    WfiWfe,
    /// `0x07`: trapped SVE, SIMD or floating point access.
    SimdFpAccess,
    /// `0x0e`: illegal execution state.
    IllegalState,
    /// `0x15`: `svc` from AArch64.
    Svc64,
    /// `0x16`: `hvc` from AArch64.
    Hvc64,
    /// `0x17`: `smc` from AArch64.
    Smc64,
    /// `0x18`: trapped `msr`, `mrs` or system instruction from AArch64.
    SysReg64,
    /// `0x20`: instruction abort from a lower exception level.
    InstructionAbortLowerEl,
    /// `0x22`: PC alignment fault.
    PcAlignment,
    /// `0x24`: data abort from a lower exception level.
    DataAbortLowerEl,
    /// `0x26`: SP alignment fault.
    SpAlignment,
    /// `0x30`: breakpoint from a lower exception level.
    BreakpointLowerEl,
    /// `0x32`: software step from a lower exception level.
    SoftwareStepLowerEl,
    /// `0x34`: watchpoint from a lower exception level.
    WatchpointLowerEl,
    /// `0x3c`: `brk` from AArch64.
    Brk64,
    /// Any other exception class.
    Other(u8),
}

impl ExceptionClass {
    /// Decodes a raw `EC` value.
    pub fn from_ec(ec: u8) -> Self {
        match ec {
            0x00 => Self::Unknown,
            0x01 => Self::WfiWfe,
            0x07 => Self::SimdFpAccess,
            0x0e => Self::IllegalState,
            0x15 => Self::Svc64,
            0x16 => Self::Hvc64,
            0x17 => Self::Smc64,
            0x18 => Self::SysReg64,
            0x20 => Self::InstructionAbortLowerEl,
            0x22 => Self::PcAlignment,
            0x24 => Self::DataAbortLowerEl,
            0x26 => Self::SpAlignment,
            0x30 => Self::BreakpointLowerEl,
            0x32 => Self::SoftwareStepLowerEl,
            0x34 => Self::WatchpointLowerEl,
            0x3c => Self::Brk64,
            other => Self::Other(other),
        }
    }

    /// The raw `EC` value.
    pub fn ec(self) -> u8 {
        match self {
            Self::Unknown => 0x00,
            Self::WfiWfe => 0x01,
            Self::SimdFpAccess => 0x07,
            Self::IllegalState => 0x0e,
            Self::Svc64 => 0x15,
            Self::Hvc64 => 0x16,
            Self::Smc64 => 0x17,
            Self::SysReg64 => 0x18,
            Self::InstructionAbortLowerEl => 0x20,
            Self::PcAlignment => 0x22,
            Self::DataAbortLowerEl => 0x24,
            Self::SpAlignment => 0x26,
            Self::BreakpointLowerEl => 0x30,
            Self::SoftwareStepLowerEl => 0x32,
            Self::WatchpointLowerEl => 0x34,
            Self::Brk64 => 0x3c,
            Self::Other(ec) => ec,
        }
    }
}

/// A decoded data abort (see [`Exception::data_abort`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DataAbort {
    /// `ISV`: whether `size`, `sign_extend`, `register` and `sixty_four`
    /// describe the access. Loads/stores of pairs, for example, leave this
    /// clear and must be decoded from the instruction instead.
    pub valid: bool,
    /// Access size in bytes (1, 2, 4 or 8).
    pub size: usize,
    /// Whether a load sign-extends the value.
    pub sign_extend: bool,
    /// The transfer register number (`Xn`/`Wn`; 31 means `XZR`).
    pub register: u8,
    /// Whether the transfer register is 64 bits wide.
    pub sixty_four: bool,
    /// Whether the access was a write.
    pub write: bool,
    /// Data fault status code (`DFSC`).
    pub fault_status: u8,
    /// The faulting guest physical address.
    pub address: u64,
}

/// A decoded trapped system register access (see [`Exception::sys_reg_access`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SysRegAccess {
    /// The system register that was accessed.
    pub reg: SysReg,
    /// The general purpose register number (`Rt`; 31 means `XZR`).
    pub register: u8,
    /// `true` for `mrs` (read), `false` for `msr` (write).
    pub read: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn exc(syndrome: u64) -> Exception {
        Exception {
            syndrome,
            virtual_address: 0,
            physical_address: 0x1000,
        }
    }

    #[test]
    fn hvc_immediate() {
        // EC=0x16, IL=1, imm16=0x1234
        let e = exc((0x16 << 26) | (1 << 25) | 0x1234);
        assert_eq!(e.class(), ExceptionClass::Hvc64);
        assert!(e.is_32bit_instruction());
        assert_eq!(e.immediate(), Some(0x1234));
        assert_eq!(e.data_abort(), None);
    }

    #[test]
    fn data_abort_decode() {
        // str w3, [x?]: ISV=1, SAS=2 (4 bytes), SRT=3, WnR=1, DFSC=0x07
        let iss = (1 << 24) | (2 << 22) | (3 << 16) | (1 << 6) | 0x07;
        let d = exc((0x24 << 26) | (1 << 25) | iss).data_abort().unwrap();
        assert!(d.valid);
        assert_eq!(d.size, 4);
        assert_eq!(d.register, 3);
        assert!(d.write);
        assert!(!d.sixty_four);
        assert_eq!(d.fault_status, 0x07);
        assert_eq!(d.address, 0x1000);
    }

    #[test]
    fn sys_reg_decode() {
        // mrs x5, sctlr_el1: op0=3 op1=0 crn=1 crm=0 op2=0, rt=5, read
        let iss = (3 << 20) | (1 << 10) | (5 << 5) | 1;
        let a = exc((0x18 << 26) | (1 << 25) | iss)
            .sys_reg_access()
            .unwrap();
        assert_eq!(a.reg, SysReg::SCTLR_EL1);
        assert_eq!(a.register, 5);
        assert!(a.read);
    }

    #[test]
    fn class_round_trip() {
        for ec in 0..64u8 {
            assert_eq!(ExceptionClass::from_ec(ec).ec(), ec);
        }
    }
}
