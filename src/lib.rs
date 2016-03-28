extern crate libc;
extern crate core;

#[allow(non_camel_case_types)]
pub mod ffi;

/**
Hypervisor API for OSX

To use this library, you need

* OS X Yosemite (10.10) or newer

* an Intel processor with the VT-x feature set that includes Extended Page
Tables (EPT) and Unrestricted Mode. To verify this, run and expect the following
in your Terminal:

  ```shell
  $ sysctl kern.hv_support
  kern.hv_support: 1
  ```
**/

use self::core::fmt;
use libc::*;

use self::ffi::*;
use self::ffi::consts::*;

/// Error returned after every call
pub enum Error {
    /// Success
    Success,
    /// Error
    Error,
    /// Busy
    Busy,
    /// Bad argument
    BadArg,
    /// No resources
    NoRes,
    /// No device
    NoDev,
    /// Unsupported
    Unsupp
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Success => write!(f, "Success"),
            Error::Error => write!(f, "Error"),
            Error::Busy => write!(f, "Busy"),
            Error::BadArg => write!(f, "Bad argument"),
            Error::NoRes => write!(f, "No resources"),
            Error::NoDev => write!(f, "No device"),
            Error::Unsupp => write!(f, "Unsupported"),
        }
    }
}

fn match_error_code(code: hv_return_t) -> Error {
    match code {
        HV_SUCCESS => Error::Success,
        HV_BUSY => Error::Busy,
        HV_BAD_ARGUMENT => Error::BadArg,
        HV_NO_RESOURCES => Error::NoRes,
        HV_NO_DEVICE => Error::NoDev,
        HV_UNSUPPORTED => Error::Unsupp,
        _ => Error::Error
    }
}

/// Creates a VM instance for the current Mach task
pub fn create_vm() -> Error {
    match_error_code(unsafe {
        hv_vm_create(0)
    })
}

/// Destroys the VM instance associated with the current Mach task
pub fn destory_vm() -> Error {
    match_error_code(unsafe {
        hv_vm_destroy()
    })
}

/// Synchronizes the guest Timestamp-Counters (TSC) across all vCPUs
///
/// * `tsc` Guest TSC value
pub fn sync_tsc(tsc: u64) -> Error {
    match_error_code(unsafe {
        hv_vm_sync_tsc(tsc as uint64_t)
    })
}

/// Forces an immediate VMEXIT of a set of vCPUs
///
/// * `vcpu_ids` Array of vCPU IDs
pub fn interrupt_vcpus(vcpu_ids: &[u32]) -> Error {
    match_error_code(unsafe {
        hv_vcpu_interrupt(vcpu_ids.as_ptr(), vcpu_ids.len() as c_uint)
    })
}

/// Virtual CPU
#[allow(non_camel_case_types)]
pub struct vCPU {
    /// Virtual CPU ID
    pub id: u32
}

/// x86 architectural register
#[allow(non_camel_case_types)]
pub enum x86Reg {
	RIP,
	RFLAGS,
	RAX,
	RCX,
	RDX,
	RBX,
	RSI,
	RDI,
	RSP,
	RBP,
	R8,
	R9,
	R10,
	R11,
	R12,
	R13,
	R14,
	R15,
	CS,
	SS,
	DS,
	ES,
	FS,
	GS,
	IDT_BASE,
	IDT_LIMIT,
	GDT_BASE,
	GDT_LIMIT,
	LDTR,
	LDT_BASE,
	LDT_LIMIT,
	LDT_AR,
	TR,
	TSS_BASE,
	TSS_LIMIT,
	TSS_AR,
	CR0,
	CR1,
	CR2,
	CR3,
	CR4,
	DR0,
	DR1,
	DR2,
	DR3,
	DR4,
	DR5,
	DR6,
	DR7,
	TPR,
	XCR0,
	REGISTERS_MAX,
}

#[allow(non_snake_case)]
fn match_x86Reg(reg_id: &x86Reg) -> hv_x86_reg_t {
    match reg_id {
        &x86Reg::RIP           => hv_x86_reg_t::HV_X86_RIP,
        &x86Reg::RFLAGS        => hv_x86_reg_t::HV_X86_RFLAGS,
        &x86Reg::RAX           => hv_x86_reg_t::HV_X86_RAX,
        &x86Reg::RCX           => hv_x86_reg_t::HV_X86_RCX,
        &x86Reg::RDX           => hv_x86_reg_t::HV_X86_RDX,
        &x86Reg::RBX           => hv_x86_reg_t::HV_X86_RBX,
        &x86Reg::RSI           => hv_x86_reg_t::HV_X86_RSI,
        &x86Reg::RDI           => hv_x86_reg_t::HV_X86_RDI,
        &x86Reg::RSP           => hv_x86_reg_t::HV_X86_RSP,
        &x86Reg::RBP           => hv_x86_reg_t::HV_X86_RBP,
        &x86Reg::R8            => hv_x86_reg_t::HV_X86_R8,
        &x86Reg::R9            => hv_x86_reg_t::HV_X86_R9,
        &x86Reg::R10           => hv_x86_reg_t::HV_X86_R10,
        &x86Reg::R11           => hv_x86_reg_t::HV_X86_R11,
        &x86Reg::R12           => hv_x86_reg_t::HV_X86_R12,
        &x86Reg::R13           => hv_x86_reg_t::HV_X86_R13,
        &x86Reg::R14           => hv_x86_reg_t::HV_X86_R14,
        &x86Reg::R15           => hv_x86_reg_t::HV_X86_R15,
        &x86Reg::CS            => hv_x86_reg_t::HV_X86_CS,
        &x86Reg::SS            => hv_x86_reg_t::HV_X86_SS,
        &x86Reg::DS            => hv_x86_reg_t::HV_X86_DS,
        &x86Reg::ES            => hv_x86_reg_t::HV_X86_ES,
        &x86Reg::FS            => hv_x86_reg_t::HV_X86_FS,
        &x86Reg::GS            => hv_x86_reg_t::HV_X86_GS,
        &x86Reg::IDT_BASE      => hv_x86_reg_t::HV_X86_IDT_BASE,
        &x86Reg::IDT_LIMIT     => hv_x86_reg_t::HV_X86_IDT_LIMIT,
        &x86Reg::GDT_BASE      => hv_x86_reg_t::HV_X86_GDT_BASE,
        &x86Reg::GDT_LIMIT     => hv_x86_reg_t::HV_X86_GDT_LIMIT,
        &x86Reg::LDTR          => hv_x86_reg_t::HV_X86_LDTR,
        &x86Reg::LDT_BASE      => hv_x86_reg_t::HV_X86_LDT_BASE,
        &x86Reg::LDT_LIMIT     => hv_x86_reg_t::HV_X86_LDT_LIMIT,
        &x86Reg::LDT_AR        => hv_x86_reg_t::HV_X86_LDT_AR,
        &x86Reg::TR            => hv_x86_reg_t::HV_X86_TR,
        &x86Reg::TSS_BASE      => hv_x86_reg_t::HV_X86_TSS_BASE,
        &x86Reg::TSS_LIMIT     => hv_x86_reg_t::HV_X86_TSS_LIMIT,
        &x86Reg::TSS_AR        => hv_x86_reg_t::HV_X86_TSS_AR,
        &x86Reg::CR0           => hv_x86_reg_t::HV_X86_CR0,
        &x86Reg::CR1           => hv_x86_reg_t::HV_X86_CR1,
        &x86Reg::CR2           => hv_x86_reg_t::HV_X86_CR2,
        &x86Reg::CR3           => hv_x86_reg_t::HV_X86_CR3,
        &x86Reg::CR4           => hv_x86_reg_t::HV_X86_CR4,
        &x86Reg::DR0           => hv_x86_reg_t::HV_X86_DR0,
        &x86Reg::DR1           => hv_x86_reg_t::HV_X86_DR1,
        &x86Reg::DR2           => hv_x86_reg_t::HV_X86_DR2,
        &x86Reg::DR3           => hv_x86_reg_t::HV_X86_DR3,
        &x86Reg::DR4           => hv_x86_reg_t::HV_X86_DR4,
        &x86Reg::DR5           => hv_x86_reg_t::HV_X86_DR5,
        &x86Reg::DR6           => hv_x86_reg_t::HV_X86_DR6,
        &x86Reg::DR7           => hv_x86_reg_t::HV_X86_DR7,
        &x86Reg::TPR           => hv_x86_reg_t::HV_X86_TPR,
        &x86Reg::XCR0          => hv_x86_reg_t::HV_X86_XCR0,
        &x86Reg::REGISTERS_MAX => hv_x86_reg_t::HV_X86_REGISTERS_MAX,
    }
}

impl vCPU {
    /// Creates a vCPU instance for the current thread
    pub fn new() -> Result<vCPU, Error> {
        let mut vcpuid: hv_vcpuid_t = 0;

        let error = match_error_code(unsafe {
            hv_vcpu_create(&mut vcpuid, 0)
        });

        match error {
            Error::Success => Ok(vCPU {
                id: vcpuid as u32
            }),
            _ => Err(error)
        }
    }

    /// Destroys the vCPU instance associated with the current thread
    pub fn destory(&self) -> Error {
        match_error_code(unsafe {
            hv_vcpu_destroy(self.id as hv_vcpuid_t)
        })
    }

    /// Executes the vCPU
    pub fn run(&self) -> Error {
        match_error_code(unsafe {
            hv_vcpu_run(self.id as hv_vcpuid_t)
        })
    }

    /// Forces an immediate VMEXIT of the vCPU
    pub fn interrupt(&self) -> Error {
        match_error_code(unsafe {
            hv_vcpu_interrupt(&(self.id), 2 as c_uint)
        })
    }

    /// Returns the cumulative execution time of the vCPU in nanoseconds
    pub fn exec_time(&self) -> Result<u64, Error> {
        let mut exec_time: uint64_t = 0;

        let error = match_error_code(unsafe {
            hv_vcpu_get_exec_time(self.id, &mut exec_time)
        });

        match error {
            Error::Success => Ok(exec_time as u64),
            _ => Err(error)
        }
    }

    /// Forces flushing of cached vCPU state
    pub fn flush(&self) -> Error {
        match_error_code(unsafe {
            hv_vcpu_flush(self.id as hv_vcpuid_t)
        })
    }

    /// Invalidates the translation lookaside buffer (TLB) of the vCPU
    pub fn invalidate_tlb(&self) -> Error {
        match_error_code(unsafe {
            hv_vcpu_invalidate_tlb(self.id as hv_vcpuid_t)
        })
    }

    /// Enables an MSR to be used natively by the VM
    pub fn enable_native_msr(&self, msr: u32, enable: bool) -> Error {
        match_error_code(unsafe {
            hv_vcpu_enable_native_msr(self.id as hv_vcpuid_t, msr as uint32_t, enable)
        })
    }

    /// Returns the current value of an MSR of the vCPU
    pub fn read_msr(&self, msr: u32) -> Result<u64, Error> {
        let mut value: uint64_t = 0;

            let error = match_error_code(unsafe {
                hv_vcpu_read_msr(self.id as hv_vcpuid_t, msr as uint32_t, &mut value)
            });

        match error {
            Error::Success => Ok(value as u64),
            _ => Err(error)
        }
    }

    /// Set the value of an MSR of the vCPU
    pub fn write_msr(&self, msr: u32, value: u64) -> Error {
        match_error_code(unsafe {
            hv_vcpu_write_msr(self.id as hv_vcpuid_t, msr as uint32_t, &(value as uint64_t))
        })
    }

    /// Returns the current value of an architectural x86 register
    /// of the vCPU
    pub fn read_register(&self, reg: &x86Reg) -> Result<u64, Error> {
        let mut value: uint64_t = 0;

        let error = match_error_code(unsafe {
            hv_vcpu_read_register(self.id as hv_vcpuid_t, match_x86Reg(&reg), &mut value)
        });

        match error {
            Error::Success => Ok(value as u64),
            _ => Err(error)
        }
    }

    /// Sets the value of an architectural x86 register of the vCPU
    pub fn write_register(&self, reg: &x86Reg, value: u64) -> Error {
        match_error_code(unsafe {
            hv_vcpu_write_register(self.id as hv_vcpuid_t, match_x86Reg(&reg), value as uint64_t)
        })
    }

    /// Returns the current value of a VMCS field of the vCPU
    pub fn read_vmcs(&self, field: u32) -> Result<u64, Error> {
        let mut value: uint64_t = 0;

        let error = match_error_code(unsafe {
            hv_vmx_vcpu_read_vmcs(self.id as hv_vcpuid_t, field as uint32_t, &mut value)
        });

        match error {
            Error::Success => Ok(value as u64),
            _ => Err(error)
        }
    }

    /// Sets the value of a VMCS field of the vCPU
    pub fn write_vmcs(&self, field: u32, value: u64) -> Error {
        match_error_code(unsafe {
            hv_vmx_vcpu_write_vmcs(self.id as hv_vcpuid_t, field as uint32_t, value as uint64_t)
        })
    }

    /// Sets the address of the guest APIC for the vCPU in the
    /// guest physical address space of the VM
    pub fn set_apic_addr(&self, gpa: u64) -> Error {
        match_error_code(unsafe {
            hv_vmx_vcpu_set_apic_address(self.id as hv_vcpuid_t, gpa as uint64_t)
        })
    }

}

impl fmt::Debug for vCPU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "vCPU ID: {}", (*self).id)
    }
}

/// VMX cabability
#[allow(non_camel_case_types)]
pub enum VMXCap {
    /// Pin-based VMX capabilities
    PINBASED,
    /// Primary proc-based VMX capabilities
    PROCBASED,
    /// Secondary proc-based VMX capabilities
    PROCBASED2,
    /// VM-entry VMX capabilities
    ENTRY,
    /// VM-exit VMX capabilities
    EXIT,
    /// VMX preemption timer frequency
    PREEMPTION_TIMER,
}

#[allow(non_snake_case)]
fn match_VMXCap(vmx_cap: &VMXCap) -> hv_vmx_capability_t {
    match vmx_cap {
        &VMXCap::PINBASED         => hv_vmx_capability_t::HV_VMX_CAP_PINBASED,
        &VMXCap::PROCBASED        => hv_vmx_capability_t::HV_VMX_CAP_PROCBASED,
        &VMXCap::PROCBASED2       => hv_vmx_capability_t::HV_VMX_CAP_PROCBASED2,
        &VMXCap::ENTRY            => hv_vmx_capability_t::HV_VMX_CAP_ENTRY,
        &VMXCap::EXIT             => hv_vmx_capability_t::HV_VMX_CAP_EXIT,
        &VMXCap::PREEMPTION_TIMER => hv_vmx_capability_t::HV_VMX_CAP_PREEMPTION_TIMER,
    }
}

/// Reads a VMX capability of the host processor
pub fn read_vmx_cap(vmx_cap: &VMXCap) -> Result<u64, Error> {
    let mut value: uint64_t = 0;

    let error = match_error_code(unsafe {
        hv_vmx_read_capability(match_VMXCap(&vmx_cap), &mut value)
    });

    match error {
        Error::Success => Ok(value as u64),
        _ => Err(error)
    }
}
