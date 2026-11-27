/*
Copyright (c) 2016 Saurav Sachidanand

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
*/

//! Safe API for the Intel (x86_64, VT-x) Hypervisor.framework.

pub mod consts;
pub mod ffi;

use core::ffi::{c_uint, c_void};
use core::fmt;

use self::ffi::*;
use crate::{Error, Result};

fn check(code: hv_return_t) -> Result<()> {
    Error::check(code)
}

/// Creates a VM instance for the current Mach task
pub fn create_vm() -> Result<()> {
    check(unsafe { hv_vm_create(HV_VM_DEFAULT) })
}

/// Destroys the VM instance associated with the current Mach task
pub fn destroy_vm() -> Result<()> {
    check(unsafe { hv_vm_destroy() })
}

/// Guest physical memory region permissions
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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
    ExecAndRead,
}

impl MemPerm {
    fn flags(self) -> hv_memory_flags_t {
        match self {
            MemPerm::Read => HV_MEMORY_READ,
            MemPerm::Write => HV_MEMORY_WRITE | HV_MEMORY_READ,
            MemPerm::Exec => HV_MEMORY_EXEC,
            MemPerm::ExecAndWrite => HV_MEMORY_EXEC | HV_MEMORY_WRITE | HV_MEMORY_READ,
            MemPerm::ExecAndRead => HV_MEMORY_EXEC | HV_MEMORY_READ,
        }
    }
}

/// Maps a region in the virtual address space of the current Mach task into the guest physical
/// address space of the virtual machine
///
/// `mem` must start on a page boundary and its length must be a multiple of the page size
/// (4 KiB), otherwise the framework returns [`Error::BadArgument`]. A `Vec<u8>` or a small array
/// is generally *not* page aligned; allocate with an explicit alignment, for example
/// `std::alloc::alloc_zeroed(Layout::from_size_align(4096, 4096)?)`, or with `mmap`.
///
/// # Safety
///
/// The memory must stay allocated until it is unmapped with [`unmap_mem`] or the VM is
/// destroyed, and the guest may write to it at any time while a vCPU runs (if `mem_perm`
/// allows writes), so no Rust reference to it may be used while that can happen.
pub unsafe fn map_mem(mem: &[u8], gpa: u64, mem_perm: &MemPerm) -> Result<()> {
    check(unsafe {
        hv_vm_map(
            mem.as_ptr() as *const c_void,
            gpa as hv_gpaddr_t,
            mem.len(),
            mem_perm.flags(),
        )
    })
}

/// Unmaps a region in the guest physical address space of the virtual machine
pub fn unmap_mem(gpa: u64, size: usize) -> Result<()> {
    check(unsafe { hv_vm_unmap(gpa as hv_gpaddr_t, size) })
}

/// Modifies the permissions of a region in the guest physical address space of the virtual
/// machine
pub fn protect_mem(gpa: u64, size: usize, mem_perm: &MemPerm) -> Result<()> {
    check(unsafe { hv_vm_protect(gpa as hv_gpaddr_t, size, mem_perm.flags()) })
}

/// Synchronizes the guest Timestamp-Counters (TSC) across all vCPUs
///
/// * `tsc` Guest TSC value
pub fn sync_tsc(tsc: u64) -> Result<()> {
    check(unsafe { hv_vm_sync_tsc(tsc) })
}

/// Forces an immediate VMEXIT of a set of vCPUs
///
/// * `vcpu_ids` Array of vCPU IDs
pub fn interrupt_vcpus(vcpu_ids: &[u32]) -> Result<()> {
    let count = c_uint::try_from(vcpu_ids.len()).map_err(|_| Error::BadArgument)?;
    check(unsafe { hv_vcpu_interrupt(vcpu_ids.as_ptr(), count) })
}

/// Virtual CPU
///
/// A vCPU belongs to the thread that created it: call its methods (other than
/// [`interrupt_vcpus`]) from that thread only.
#[allow(non_camel_case_types)]
pub struct vCPU {
    /// Virtual CPU ID
    pub id: u32,
}

/// x86 architectural register
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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
    pub fn new() -> Result<vCPU> {
        let mut vcpuid: hv_vcpuid_t = 0;
        check(unsafe { hv_vcpu_create(&mut vcpuid, HV_VCPU_DEFAULT) })?;
        Ok(vCPU { id: vcpuid })
    }

    /// Destroys the vCPU instance associated with the current thread
    pub fn destroy(&self) -> Result<()> {
        check(unsafe { hv_vcpu_destroy(self.id) })
    }

    /// Executes the vCPU
    pub fn run(&self) -> Result<()> {
        check(unsafe { hv_vcpu_run(self.id) })
    }

    /// Forces an immediate VMEXIT of the vCPU
    pub fn interrupt(&self) -> Result<()> {
        check(unsafe { hv_vcpu_interrupt(&self.id, 1) })
    }

    /// Returns the cumulative execution time of the vCPU in nanoseconds
    pub fn exec_time(&self) -> Result<u64> {
        let mut exec_time: u64 = 0;
        check(unsafe { hv_vcpu_get_exec_time(self.id, &mut exec_time) })?;
        Ok(exec_time)
    }

    /// Forces flushing of cached vCPU state
    ///
    /// Deprecated by Apple since macOS 11: it has no effect and returns
    /// [`Error::Unsupported`].
    pub fn flush(&self) -> Result<()> {
        check(unsafe { hv_vcpu_flush(self.id) })
    }

    /// Invalidates the translation lookaside buffer (TLB) of the vCPU
    pub fn invalidate_tlb(&self) -> Result<()> {
        check(unsafe { hv_vcpu_invalidate_tlb(self.id) })
    }

    /// Enables an MSR to be used natively by the VM
    pub fn enable_native_msr(&self, msr: u32, enable: bool) -> Result<()> {
        check(unsafe { hv_vcpu_enable_native_msr(self.id, msr, enable) })
    }

    /// Returns the current value of an MSR of the vCPU
    pub fn read_msr(&self, msr: u32) -> Result<u64> {
        let mut value: u64 = 0;
        check(unsafe { hv_vcpu_read_msr(self.id, msr, &mut value) })?;
        Ok(value)
    }

    /// Set the value of an MSR of the vCPU
    pub fn write_msr(&self, msr: u32, value: u64) -> Result<()> {
        check(unsafe { hv_vcpu_write_msr(self.id, msr, value) })
    }

    /// Returns the current value of an architectural x86 register
    /// of the vCPU
    pub fn read_register(&self, reg: &x86Reg) -> Result<u64> {
        let mut value: u64 = 0;
        check(unsafe { hv_vcpu_read_register(self.id, *reg, &mut value) })?;
        Ok(value)
    }

    /// Sets the value of an architectural x86 register of the vCPU
    pub fn write_register(&self, reg: &x86Reg, value: u64) -> Result<()> {
        check(unsafe { hv_vcpu_write_register(self.id, *reg, value) })
    }

    /// Returns the current value of a VMCS field of the vCPU
    pub fn read_vmcs(&self, field: u32) -> Result<u64> {
        let mut value: u64 = 0;
        check(unsafe { hv_vmx_vcpu_read_vmcs(self.id, field, &mut value) })?;
        Ok(value)
    }

    /// Sets the value of a VMCS field of the vCPU
    pub fn write_vmcs(&self, field: u32, value: u64) -> Result<()> {
        check(unsafe { hv_vmx_vcpu_write_vmcs(self.id, field, value) })
    }

    /// Sets the address of the guest APIC for the vCPU in the
    /// guest physical address space of the VM
    pub fn set_apic_addr(&self, gpa: u64) -> Result<()> {
        check(unsafe { hv_vmx_vcpu_set_apic_address(self.id, gpa) })
    }

    /// Reads the current architectural x86 floating point and SIMD state of the vCPU
    pub fn read_fpstate(&self, buffer: &mut [u8]) -> Result<()> {
        check(unsafe {
            hv_vcpu_read_fpstate(self.id, buffer.as_mut_ptr() as *mut c_void, buffer.len())
        })
    }

    /// Sets the architectural x86 floating point and SIMD state of the vCPU
    pub fn write_fpstate(&self, buffer: &[u8]) -> Result<()> {
        check(unsafe {
            hv_vcpu_write_fpstate(self.id, buffer.as_ptr() as *const c_void, buffer.len())
        })
    }
}

impl fmt::Debug for vCPU {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "vCPU ID: {}", self.id)
    }
}

/// VMX cabability
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub enum VMXCap {
    /// Pin-based VMX capabilities
    PINBASED = 0,
    /// Primary proc-based VMX capabilities
    PROCBASED = 1,
    /// Secondary proc-based VMX capabilities
    PROCBASED2 = 2,
    /// VM-entry VMX capabilities
    ENTRY = 3,
    /// VM-exit VMX capabilities
    EXIT = 4,
    /// Basic VMX capabilities
    BASIC = 5,
    /// Hardware pin-based VMX capabilities
    TRUE_PINBASED = 6,
    /// Hardware primary proc-based VMX capabilities
    TRUE_PROCBASED = 7,
    /// Hardware VM-entry VMX capabilities
    TRUE_ENTRY = 8,
    /// Hardware VM-exit VMX capabilities
    TRUE_EXIT = 9,
    /// Miscellaneous VMX capabilities
    MISC = 10,
    /// CR0 allowed zero bits
    CR0_FIXED0 = 11,
    /// CR0 allowed one bits
    CR0_FIXED1 = 12,
    /// CR4 allowed zero bits
    CR4_FIXED0 = 13,
    /// CR4 allowed one bits
    CR4_FIXED1 = 14,
    /// VMCS enumeration
    VMCS_ENUM = 15,
    /// EPT/VPID capabilities
    EPT_VPID_CAP = 16,
    /// VMX preemption timer frequency
    PREEMPTION_TIMER = 32,
}

/// Reads a VMX capability of the host processor
pub fn read_vmx_cap(vmx_cap: &VMXCap) -> Result<u64> {
    let mut value: u64 = 0;
    check(unsafe { hv_vmx_read_capability(*vmx_cap, &mut value) })?;
    Ok(value)
}

impl fmt::Display for VMXCap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match *self {
            VMXCap::PINBASED => "Pin-based VMX capabilities",
            VMXCap::PROCBASED => "Primary proc-based VMX capabilities",
            VMXCap::PROCBASED2 => "Secondary proc-based VMX capabilities",
            VMXCap::ENTRY => "VM-entry VMX capabilities",
            VMXCap::EXIT => "VM-exit VMX capabilities",
            VMXCap::BASIC => "Basic VMX capabilities",
            VMXCap::TRUE_PINBASED => "Hardware pin-based VMX capabilities",
            VMXCap::TRUE_PROCBASED => "Hardware primary proc-based VMX capabilities",
            VMXCap::TRUE_ENTRY => "Hardware VM-entry VMX capabilities",
            VMXCap::TRUE_EXIT => "Hardware VM-exit VMX capabilities",
            VMXCap::MISC => "Miscellaneous VMX capabilities",
            VMXCap::CR0_FIXED0 => "CR0 allowed zero bits",
            VMXCap::CR0_FIXED1 => "CR0 allowed one bits",
            VMXCap::CR4_FIXED0 => "CR4 allowed zero bits",
            VMXCap::CR4_FIXED1 => "CR4 allowed one bits",
            VMXCap::VMCS_ENUM => "VMCS enumeration",
            VMXCap::EPT_VPID_CAP => "EPT/VPID capabilities",
            VMXCap::PREEMPTION_TIMER => "VMX preemption timer frequency",
        };
        f.write_str(s)
    }
}
