//! Safe API for the Apple silicon (arm64) Hypervisor.framework.

pub mod ffi;

mod exit;
mod memory;
mod regs;
mod vcpu;

pub use exit::{DataAbort, Exception, ExceptionClass, SysRegAccess, VcpuExit};
pub use memory::{GuestMemory, MemPerm, page_size};
pub use regs::{CacheType, FeatureReg, InterruptType, Reg, SimdFpReg, SysReg};
pub use vcpu::{Vcpu, VcpuConfig, VcpuHandle, exit_vcpus};

use crate::{Error, Result};

pub(crate) fn check(ret: ffi::hv_return_t) -> Result<()> {
    Error::check(ret as u32)
}

/// The virtual machine of the current process.
///
/// A process can have at most one VM at a time; creating a second one while
/// the first exists fails. Dropping the `Vm` destroys it. [`Vcpu`]s and
/// [`GuestMemory`] borrow the `Vm`, so they are always cleaned up first.
///
/// The VM itself (mapping and unmapping memory) may be used from any thread,
/// so `Vm` is [`Send`] and [`Sync`]; share it with `&Vm`, scoped threads or
/// an `Arc` and create one [`Vcpu`] per thread.
///
/// The process must be signed with the `com.apple.security.hypervisor`
/// entitlement, otherwise [`Vm::new`] fails with [`Error::Denied`].
#[derive(Debug)]
pub struct Vm {
    _private: (),
}

impl Vm {
    /// Creates the VM for the current process with the default configuration.
    pub fn new() -> Result<Self> {
        check(unsafe { ffi::hv_vm_create(core::ptr::null_mut()) })?;
        Ok(Self { _private: () })
    }

    /// Maps `size` bytes of host memory at `host` into the guest physical
    /// address space at `ipa`.
    ///
    /// Prefer [`GuestMemory`], which allocates, maps and frees memory safely.
    ///
    /// # Safety
    ///
    /// `host` must point to a page-aligned allocation of at least `size`
    /// bytes that stays valid until the range is unmapped or the VM is
    /// destroyed. The guest can read and write that memory at any time while
    /// a vCPU runs, so the host must not hold Rust references into it that
    /// assume it does not change.
    pub unsafe fn map(&self, host: *mut u8, ipa: u64, size: usize, perm: MemPerm) -> Result<()> {
        check(unsafe { ffi::hv_vm_map(host.cast(), ipa, size, perm.bits()) })
    }

    /// Unmaps a range of the guest physical address space.
    pub fn unmap(&self, ipa: u64, size: usize) -> Result<()> {
        check(unsafe { ffi::hv_vm_unmap(ipa, size) })
    }

    /// Changes the guest permissions of a range of the guest physical
    /// address space.
    pub fn protect(&self, ipa: u64, size: usize, perm: MemPerm) -> Result<()> {
        check(unsafe { ffi::hv_vm_protect(ipa, size, perm.bits()) })
    }
}

impl Drop for Vm {
    fn drop(&mut self) {
        // All vCPUs and GuestMemory borrowing this VM are gone by now.
        unsafe { ffi::hv_vm_destroy() };
    }
}

/// Returns the maximum number of vCPUs a VM supports on this host.
pub fn max_vcpu_count() -> Result<u32> {
    let mut count = 0;
    check(unsafe { ffi::hv_vm_get_max_vcpu_count(&mut count) })?;
    Ok(count)
}
