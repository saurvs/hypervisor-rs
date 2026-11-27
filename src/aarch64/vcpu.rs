//! Virtual CPUs.

use core::ffi::c_void;
use core::marker::PhantomData;
use core::ptr::{self, NonNull};

use super::{CacheType, FeatureReg, InterruptType, Reg, SimdFpReg, SysReg, VcpuExit, Vm};
use super::{check, ffi};
use crate::{Error, Result};

/// A virtual CPU.
///
/// Hypervisor.framework binds a vCPU to the thread that created it: every
/// call except [`hv_vcpus_exit`](ffi::hv_vcpus_exit) must be made from that
/// thread, and each thread can own at most one vCPU at a time. `Vcpu` is
/// therefore neither [`Send`] nor [`Sync`]; create it on the thread that will
/// run it. To stop a running vCPU from another thread, use a
/// [`VcpuHandle`] obtained with [`Vcpu::handle`].
///
/// The vCPU borrows the [`Vm`], so it is always destroyed (on drop) before
/// the VM is.
///
/// ```compile_fail
/// fn assert_send<T: Send>() {}
/// assert_send::<hypervisor::Vcpu<'static>>();
/// ```
#[derive(Debug)]
pub struct Vcpu<'vm> {
    id: ffi::hv_vcpu_t,
    exit: *const ffi::hv_vcpu_exit_t,
    _vm: PhantomData<&'vm Vm>,
}

impl<'vm> Vcpu<'vm> {
    /// Creates a vCPU with the default configuration on the current thread.
    pub fn new(vm: &'vm Vm) -> Result<Self> {
        Self::create(vm, ptr::null_mut())
    }

    /// Creates a vCPU with the given configuration on the current thread.
    pub fn with_config(vm: &'vm Vm, config: &VcpuConfig) -> Result<Self> {
        Self::create(vm, config.as_raw())
    }

    fn create(_vm: &'vm Vm, config: ffi::hv_vcpu_config_t) -> Result<Self> {
        let mut id: ffi::hv_vcpu_t = 0;
        let mut exit: *mut ffi::hv_vcpu_exit_t = ptr::null_mut();
        check(unsafe { ffi::hv_vcpu_create(&mut id, &mut exit, config) })?;
        Ok(Self {
            id,
            exit,
            _vm: PhantomData,
        })
    }

    /// The framework's ID for this vCPU.
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Returns a handle that can stop this vCPU from any thread.
    pub fn handle(&self) -> VcpuHandle {
        VcpuHandle(self.id)
    }

    /// Runs the vCPU until the next exit and returns why it exited.
    ///
    /// This blocks the calling thread while the guest runs.
    pub fn run(&self) -> Result<VcpuExit> {
        check(unsafe { ffi::hv_vcpu_run(self.id) })?;
        Ok(self.last_exit())
    }

    /// Returns the exit information from the most recent [`run`](Self::run).
    pub fn last_exit(&self) -> VcpuExit {
        // SAFETY: the framework owns this structure and keeps it valid until
        // the vCPU is destroyed; it is only written during hv_vcpu_run on
        // this (the owning) thread.
        VcpuExit::from_raw(unsafe { &*self.exit })
    }

    /// Returns the value of a general purpose register, `PC`, `FPCR`, `FPSR`
    /// or `CPSR`.
    pub fn reg(&self, reg: Reg) -> Result<u64> {
        let mut value = 0;
        check(unsafe { ffi::hv_vcpu_get_reg(self.id, reg.into(), &mut value) })?;
        Ok(value)
    }

    /// Sets the value of a general purpose register, `PC`, `FPCR`, `FPSR` or
    /// `CPSR`.
    pub fn set_reg(&self, reg: Reg, value: u64) -> Result<()> {
        check(unsafe { ffi::hv_vcpu_set_reg(self.id, reg.into(), value) })
    }

    /// Returns the value of a 128-bit SIMD&FP register. Byte 0 of the
    /// register is the least significant byte of the result.
    pub fn simd_fp_reg(&self, reg: SimdFpReg) -> Result<u128> {
        let mut value: ffi::hv_simd_fp_uchar16_t = unsafe { core::mem::zeroed() };
        check(unsafe { ffi::hv_vcpu_get_simd_fp_reg(self.id, reg.into(), &mut value) })?;
        // SAFETY: both types are 16 plain bytes; aarch64 is little endian.
        Ok(unsafe { core::mem::transmute::<ffi::hv_simd_fp_uchar16_t, u128>(value) })
    }

    /// Sets the value of a 128-bit SIMD&FP register.
    pub fn set_simd_fp_reg(&self, reg: SimdFpReg, value: u128) -> Result<()> {
        // SAFETY: both types are 16 plain bytes.
        let value = unsafe { core::mem::transmute::<u128, ffi::hv_simd_fp_uchar16_t>(value) };
        check(unsafe { ffi::hv_vcpu_set_simd_fp_reg(self.id, reg.into(), value) })
    }

    /// Returns the value of a system register.
    pub fn sys_reg(&self, reg: SysReg) -> Result<u64> {
        let mut value = 0;
        check(unsafe { ffi::hv_vcpu_get_sys_reg(self.id, reg.into(), &mut value) })?;
        Ok(value)
    }

    /// Sets the value of a system register.
    pub fn set_sys_reg(&self, reg: SysReg, value: u64) -> Result<()> {
        check(unsafe { ffi::hv_vcpu_set_sys_reg(self.id, reg.into(), value) })
    }

    /// Returns whether a virtual IRQ or FIQ is pending.
    pub fn pending_interrupt(&self, interrupt: InterruptType) -> Result<bool> {
        let mut pending = false;
        check(unsafe {
            ffi::hv_vcpu_get_pending_interrupt(self.id, interrupt as u32, &mut pending)
        })?;
        Ok(pending)
    }

    /// Asserts or clears a virtual IRQ or FIQ.
    ///
    /// Pending interrupts are cleared automatically when [`run`](Self::run)
    /// returns, so set them again before every run.
    pub fn set_pending_interrupt(&self, interrupt: InterruptType, pending: bool) -> Result<()> {
        check(unsafe { ffi::hv_vcpu_set_pending_interrupt(self.id, interrupt as u32, pending) })
    }

    /// Returns whether guest debug exceptions are trapped to the host.
    pub fn trap_debug_exceptions(&self) -> Result<bool> {
        let mut value = false;
        check(unsafe { ffi::hv_vcpu_get_trap_debug_exceptions(self.id, &mut value) })?;
        Ok(value)
    }

    /// Sets whether guest debug exceptions (such as `brk`) are trapped to the
    /// host.
    pub fn set_trap_debug_exceptions(&self, trap: bool) -> Result<()> {
        check(unsafe { ffi::hv_vcpu_set_trap_debug_exceptions(self.id, trap) })
    }

    /// Returns whether guest accesses to debug registers are trapped to the
    /// host.
    pub fn trap_debug_reg_accesses(&self) -> Result<bool> {
        let mut value = false;
        check(unsafe { ffi::hv_vcpu_get_trap_debug_reg_accesses(self.id, &mut value) })?;
        Ok(value)
    }

    /// Sets whether guest accesses to debug registers (`DBGBCR<n>_EL1`,
    /// `DBGBVR<n>_EL1`, `DBGWCR<n>_EL1`, `DBGWVR<n>_EL1`, `MDSCR_EL1`) are
    /// trapped to the host.
    pub fn set_trap_debug_reg_accesses(&self, trap: bool) -> Result<()> {
        check(unsafe { ffi::hv_vcpu_set_trap_debug_reg_accesses(self.id, trap) })
    }

    /// Returns the cumulative execution time of the vCPU, in
    /// `mach_absolute_time()` units (not nanoseconds; see
    /// `mach_timebase_info`).
    pub fn exec_time(&self) -> Result<u64> {
        let mut time = 0;
        check(unsafe { ffi::hv_vcpu_get_exec_time(self.id, &mut time) })?;
        Ok(time)
    }

    /// Returns whether the virtual timer is masked.
    pub fn vtimer_mask(&self) -> Result<bool> {
        let mut masked = false;
        check(unsafe { ffi::hv_vcpu_get_vtimer_mask(self.id, &mut masked) })?;
        Ok(masked)
    }

    /// Masks or unmasks the virtual timer. While masked, the vCPU does not
    /// exit with [`VcpuExit::VtimerActivated`].
    pub fn set_vtimer_mask(&self, masked: bool) -> Result<()> {
        check(unsafe { ffi::hv_vcpu_set_vtimer_mask(self.id, masked) })
    }

    /// Returns the virtual timer offset.
    pub fn vtimer_offset(&self) -> Result<u64> {
        let mut offset = 0;
        check(unsafe { ffi::hv_vcpu_get_vtimer_offset(self.id, &mut offset) })?;
        Ok(offset)
    }

    /// Sets the virtual timer offset:
    /// `CNTVCT_EL0 = mach_absolute_time() - offset`.
    pub fn set_vtimer_offset(&self, offset: u64) -> Result<()> {
        check(unsafe { ffi::hv_vcpu_set_vtimer_offset(self.id, offset) })
    }
}

impl Drop for Vcpu<'_> {
    fn drop(&mut self) {
        // Cannot fail for a live vCPU on its owning thread, which the type
        // guarantees.
        unsafe { ffi::hv_vcpu_destroy(self.id) };
    }
}

/// A thread-safe handle to a [`Vcpu`], used to force it to exit.
///
/// The handle is only an ID: using it after the vCPU has been destroyed is
/// harmless but may target a newer vCPU that was given the same ID.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct VcpuHandle(ffi::hv_vcpu_t);

impl VcpuHandle {
    /// The framework's ID for the vCPU.
    pub fn id(&self) -> u64 {
        self.0
    }

    /// Forces the vCPU to exit with [`VcpuExit::Canceled`]. If it is not
    /// running, its next [`Vcpu::run`] returns immediately.
    pub fn exit(&self) -> Result<()> {
        exit_vcpus(&[*self])
    }
}

/// Forces a set of vCPUs to exit with [`VcpuExit::Canceled`].
pub fn exit_vcpus(vcpus: &[VcpuHandle]) -> Result<()> {
    let mut ids: Vec<ffi::hv_vcpu_t> = vcpus.iter().map(|h| h.0).collect();
    let count = u32::try_from(ids.len()).map_err(|_| Error::BadArgument)?;
    check(unsafe { ffi::hv_vcpus_exit(ids.as_mut_ptr(), count) })
}

/// A vCPU configuration (`hv_vcpu_config_t`).
///
/// Mostly useful for querying the feature registers a guest will see.
#[derive(Debug)]
pub struct VcpuConfig {
    raw: NonNull<c_void>,
}

impl VcpuConfig {
    /// Creates a default vCPU configuration.
    pub fn new() -> Result<Self> {
        let raw = unsafe { ffi::hv_vcpu_config_create() };
        NonNull::new(raw)
            .map(|raw| Self { raw })
            .ok_or(Error::NoResources)
    }

    /// Returns the value of a feature (ID) register for this configuration.
    pub fn feature_reg(&self, reg: FeatureReg) -> Result<u64> {
        let mut value = 0;
        check(unsafe {
            ffi::hv_vcpu_config_get_feature_reg(self.as_raw(), reg as u32, &mut value)
        })?;
        Ok(value)
    }

    /// Returns the eight `CCSIDR_EL1` values for the given cache type.
    pub fn ccsidr_el1_values(&self, cache: CacheType) -> Result<[u64; 8]> {
        let mut values = [0u64; 8];
        check(unsafe {
            ffi::hv_vcpu_config_get_ccsidr_el1_sys_reg_values(
                self.as_raw(),
                cache as u32,
                values.as_mut_ptr(),
            )
        })?;
        Ok(values)
    }

    /// The raw configuration object. It stays owned by `self`.
    pub fn as_raw(&self) -> ffi::hv_vcpu_config_t {
        self.raw.as_ptr()
    }
}

impl Drop for VcpuConfig {
    fn drop(&mut self) {
        unsafe { ffi::os_release(self.raw.as_ptr()) };
    }
}
