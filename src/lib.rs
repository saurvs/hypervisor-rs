/*!
This is a Rust library that enables hardware-accelerated execution of
virtual machines on OS X.

It binds to the `Hypervisor` framework on OS X, and exposes a safe Rust
interface through the `hypervisor` module, and an unsafe foreign function
interface through the `hypervisor::ffi` module.

To use this library, you need

* OS X Yosemite (10.10), or newer

* an Intel processor with the VT-x feature set that includes Extended Page
Tables (EPT) and Unrestricted Mode. To verify this, run and expect the following
in your Terminal:

  ```shell
  $ sysctl kern.hv_support
  kern.hv_support: 1
  ```
!*/

extern crate libc;
extern crate core;

#[allow(non_camel_case_types)]
pub mod ffi;

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
            Error::Error   => write!(f, "Error"),
            Error::Busy    => write!(f, "Busy"),
            Error::BadArg  => write!(f, "Bad argument"),
            Error::NoRes   => write!(f, "No resources"),
            Error::NoDev   => write!(f, "No device"),
            Error::Unsupp  => write!(f, "Unsupported"),
        }
    }
}

fn match_error_code(code: hv_return_t) -> Error {
    match code {
        HV_SUCCESS      => Error::Success,
        HV_BUSY         => Error::Busy,
        HV_BAD_ARGUMENT => Error::BadArg,
        HV_NO_RESOURCES => Error::NoRes,
        HV_NO_DEVICE    => Error::NoDev,
        HV_UNSUPPORTED  => Error::Unsupp,
        _               => Error::Error
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

/// Guest physical memory region permissions
pub enum MemPerm {
    /// Read
    Read,
    /// Write (implies read)
    Write,
    /// Execute
    Exec,
    /// Execute and write (implies read)
    ExecAndWrite,
    /// Execute and read
    ExecAndRead
}

#[allow(non_snake_case)]
#[inline(always)]
fn match_MemPerm(mem_perm: &MemPerm) -> uint64_t {
    match mem_perm {
        &MemPerm::Read         => HV_MEMORY_READ,
        &MemPerm::Write        => HV_MEMORY_WRITE | HV_MEMORY_READ,
        &MemPerm::Exec         => HV_MEMORY_EXEC,
        &MemPerm::ExecAndWrite => HV_MEMORY_EXEC | HV_MEMORY_WRITE | HV_MEMORY_READ,
        &MemPerm::ExecAndRead  => HV_MEMORY_EXEC | HV_MEMORY_READ,
    }
}

/// Maps a region in the virtual address space of the current Mach task into the guest physical
/// address space of the virutal machine
pub fn map_mem(mem: &[u8], gpa: u64, mem_perm: &MemPerm) -> Error {
    match_error_code(unsafe {
        hv_vm_map(
            mem.as_ptr() as *const c_void, gpa as hv_gpaddr_t, mem.len() as size_t,
            match_MemPerm(mem_perm)
        )
    })
}

/// Unmaps a region in the guest physical address space of the virutal machine
pub fn unmap_mem(gpa: u64, size: usize) -> Error {
    match_error_code(unsafe {
        hv_vm_unmap(gpa as hv_gpaddr_t, size as size_t)
    })
}

/// Modifies the permissions of a region in the guest physical address space of the virtual
/// machine
pub fn protect_mem(gpa: u64, size: usize, mem_perm: &MemPerm) -> Error {
    match_error_code(unsafe {
        hv_vm_protect(gpa as hv_gpaddr_t, size as size_t, match_MemPerm(mem_perm))
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
#[derive(Clone)]
#[repr(C)]
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
            hv_vcpu_interrupt(&(self.id), 1 as c_uint)
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
            hv_vcpu_read_register(self.id as hv_vcpuid_t, (*reg).clone(), &mut value)
        });

        match error {
            Error::Success => Ok(value as u64),
            _ => Err(error)
        }
    }

    /// Sets the value of an architectural x86 register of the vCPU
    pub fn write_register(&self, reg: &x86Reg, value: u64) -> Error {
        match_error_code(unsafe {
            hv_vcpu_write_register(self.id as hv_vcpuid_t, (*reg).clone(), value as uint64_t)
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

    /// Reads the current architectural x86 floating point and SIMD state of the vCPU
    pub fn read_fpstate(&self, buffer: &mut [u8]) -> Error {
        match_error_code(unsafe {
            hv_vcpu_read_fpstate(self.id as hv_vcpuid_t, buffer.as_mut_ptr() as *mut c_void,
            buffer.len() as size_t)
        })
    }

    /// Sets the architectural x86 floating point and SIMD state of the vCPU
    pub fn write_fpstate(&self, buffer: &[u8]) -> Error {
        match_error_code(unsafe {
            hv_vcpu_write_fpstate(self.id as hv_vcpuid_t, buffer.as_ptr() as *const c_void,
            buffer.len() as size_t)
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
#[derive(Clone)]
#[repr(C)]
pub enum VMXCap {
    /// Pin-based VMX capabilities
    PINBASED                                     = 0,
    /// Primary proc-based VMX capabilities
    PROCBASED                                    = 1,
    /// Secondary proc-based VMX capabilities
    PROCBASED2                                   = 2,
    /// VM-entry VMX capabilities
    ENTRY                                        = 3,
    /// VM-exit VMX capabilities
    EXIT                                         = 4,
    /// VMX preemption timer frequency
    PREEMPTION_TIMER                             = 32,
}

/// Reads a VMX capability of the host processor
pub fn read_vmx_cap(vmx_cap: &VMXCap) -> Result<u64, Error> {
    let mut value: uint64_t = 0;

    let error = match_error_code(unsafe {
        hv_vmx_read_capability((*vmx_cap).clone(), &mut value)
    });

    match error {
        Error::Success => Ok(value as u64),
        _ => Err(error)
    }
}
