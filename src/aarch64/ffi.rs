//! Raw bindings to the Apple silicon (arm64) Hypervisor.framework API.
//!
//! These declarations follow the macOS SDK headers (`hv_vm.h`, `hv_vm_config.h`,
//! `hv_vm_allocate.h`, `hv_vcpu.h`, `hv_vcpu_config.h`, `hv_vcpu_types.h`).
//! Functions newer than macOS 11 are marked with the release that added them;
//! calling one on an older system fails to resolve at load time.
//!
//! Register identifiers are not duplicated here: use the safe types
//! ([`Reg`](crate::Reg), [`SimdFpReg`](crate::SimdFpReg),
//! [`SysReg`](crate::SysReg), [`FeatureReg`](crate::FeatureReg)) and cast them
//! (`Reg::X0 as hv_reg_t`, `SysReg::SCTLR_EL1.0`).
//!
//! The GIC (`hv_gic_*`, macOS 15), SME (macOS 15.2) and macOS 27 additions are
//! not bound yet.

#![allow(non_camel_case_types)]

use core::ffi::c_void;

/// Hypervisor.framework return code (`mach_error_t`).
pub type hv_return_t = i32;

/// Opaque VM configuration object (an `os_object`; release with `os_release`).
pub type hv_vm_config_t = *mut c_void;
/// Opaque vCPU configuration object (an `os_object`; release with `os_release`).
pub type hv_vcpu_config_t = *mut c_void;

/// Intermediate physical address (a guest physical address).
pub type hv_ipa_t = u64;
/// Guest physical memory region permissions for `hv_vm_map()` and `hv_vm_protect()`.
pub type hv_memory_flags_t = u64;
/// Guest memory may be read.
pub const HV_MEMORY_READ: hv_memory_flags_t = 1 << 0;
/// Guest memory may be written.
pub const HV_MEMORY_WRITE: hv_memory_flags_t = 1 << 1;
/// Guest memory may be executed.
pub const HV_MEMORY_EXEC: hv_memory_flags_t = 1 << 2;

/// Flags for `hv_vm_allocate()`.
pub type hv_allocate_flags_t = u64;
/// Default allocation flags.
pub const HV_ALLOCATE_DEFAULT: hv_allocate_flags_t = 0;

/// Intermediate physical address granule (macOS 26).
pub type hv_ipa_granule_t = u32;
/// 4 KiB IPA granule.
pub const HV_IPA_GRANULE_4KB: hv_ipa_granule_t = 0;
/// 16 KiB IPA granule.
pub const HV_IPA_GRANULE_16KB: hv_ipa_granule_t = 1;

/// vCPU instance ID.
pub type hv_vcpu_t = u64;

/// Reason for a vCPU exit.
pub type hv_exit_reason_t = u32;
/// Asynchronous exit requested explicitly by `hv_vcpus_exit()`.
pub const HV_EXIT_REASON_CANCELED: hv_exit_reason_t = 0;
/// Synchronous exception to a higher exception level triggered by the guest.
pub const HV_EXIT_REASON_EXCEPTION: hv_exit_reason_t = 1;
/// The virtual timer became pending; the timer is masked automatically.
pub const HV_EXIT_REASON_VTIMER_ACTIVATED: hv_exit_reason_t = 2;
/// Unable to determine the exit reason.
pub const HV_EXIT_REASON_UNKNOWN: hv_exit_reason_t = 3;

/// Exception syndrome (`ESR_EL2`).
pub type hv_exception_syndrome_t = u64;
/// Exception virtual address (`FAR_EL2`).
pub type hv_exception_address_t = u64;

/// Details of a vCPU exception.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct hv_vcpu_exit_exception_t {
    /// Exception syndrome (`ESR_EL2`).
    pub syndrome: hv_exception_syndrome_t,
    /// Faulting virtual address (`FAR_EL2`).
    pub virtual_address: hv_exception_address_t,
    /// Faulting intermediate physical address (from `HPFAR_EL2`).
    pub physical_address: hv_ipa_t,
}

/// Information about an exit from the vCPU to the host.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct hv_vcpu_exit_t {
    /// One of the `HV_EXIT_REASON_*` values.
    pub reason: hv_exit_reason_t,
    /// Valid when `reason` is `HV_EXIT_REASON_EXCEPTION`.
    pub exception: hv_vcpu_exit_exception_t,
}

/// Value of a 128-bit SIMD&FP register (`ext_vector_type(16) uint8_t` in C).
///
/// Rust does not allow SIMD types directly in `extern "C"` signatures yet, so
/// the vector is wrapped in a `#[repr(C)]` struct. A struct with a single
/// 128-bit vector member is a homogeneous short-vector aggregate, which the
/// arm64 calling convention passes in a SIMD register exactly like the bare
/// vector, so the ABI matches the C declaration.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct hv_simd_fp_uchar16_t(pub core::arch::aarch64::uint8x16_t);

/// General purpose register identifier (see [`Reg`](crate::Reg)).
pub type hv_reg_t = u32;
/// SIMD&FP register identifier (see [`SimdFpReg`](crate::SimdFpReg)).
pub type hv_simd_fp_reg_t = u32;
/// System register identifier (see [`SysReg`](crate::SysReg)).
pub type hv_sys_reg_t = u16;
/// Feature register identifier (see [`FeatureReg`](crate::FeatureReg)).
pub type hv_feature_reg_t = u32;

/// Injected interrupt type.
pub type hv_interrupt_type_t = u32;
/// An ARM IRQ.
pub const HV_INTERRUPT_TYPE_IRQ: hv_interrupt_type_t = 0;
/// An ARM FIQ.
pub const HV_INTERRUPT_TYPE_FIQ: hv_interrupt_type_t = 1;

/// Cache type for `hv_vcpu_config_get_ccsidr_el1_sys_reg_values()`.
pub type hv_cache_type_t = u32;
/// Data or unified cache.
pub const HV_CACHE_TYPE_DATA: hv_cache_type_t = 0;
/// Instruction cache.
pub const HV_CACHE_TYPE_INSTRUCTION: hv_cache_type_t = 1;

#[link(name = "Hypervisor", kind = "framework")]
unsafe extern "C" {
    // hv_vm.h (macOS 11)

    /// Returns the maximum number of vCPUs supported.
    pub fn hv_vm_get_max_vcpu_count(max_vcpu_count: *mut u32) -> hv_return_t;
    /// Creates a VM instance for the current process. Pass NULL for the default configuration.
    pub fn hv_vm_create(config: hv_vm_config_t) -> hv_return_t;
    /// Destroys the VM instance associated with the current process. Requires all vCPUs be destroyed.
    pub fn hv_vm_destroy() -> hv_return_t;
    /// Maps a page-aligned region of the current process into the guest physical address space.
    pub fn hv_vm_map(
        addr: *mut c_void,
        ipa: hv_ipa_t,
        size: usize,
        flags: hv_memory_flags_t,
    ) -> hv_return_t;
    /// Unmaps a region in the guest physical address space.
    pub fn hv_vm_unmap(ipa: hv_ipa_t, size: usize) -> hv_return_t;
    /// Modifies the permissions of a region in the guest physical address space.
    pub fn hv_vm_protect(ipa: hv_ipa_t, size: usize, flags: hv_memory_flags_t) -> hv_return_t;

    // hv_vm_allocate.h (macOS 12.1)

    /// Allocates anonymous memory suitable to be mapped as guest memory (macOS 12.1).
    pub fn hv_vm_allocate(
        uvap: *mut *mut c_void,
        size: usize,
        flags: hv_allocate_flags_t,
    ) -> hv_return_t;
    /// Deallocates memory previously allocated by `hv_vm_allocate()` (macOS 12.1).
    pub fn hv_vm_deallocate(uva: *mut c_void, size: usize) -> hv_return_t;

    // hv_vm_config.h

    /// Creates a VM configuration object (macOS 13).
    pub fn hv_vm_config_create() -> hv_vm_config_t;
    /// Returns the maximum IPA bit length (macOS 13).
    pub fn hv_vm_config_get_max_ipa_size(ipa_bit_length: *mut u32) -> hv_return_t;
    /// Returns the default IPA bit length (macOS 13).
    pub fn hv_vm_config_get_default_ipa_size(ipa_bit_length: *mut u32) -> hv_return_t;
    /// Sets the IPA bit length of a VM configuration (macOS 13).
    pub fn hv_vm_config_set_ipa_size(config: hv_vm_config_t, ipa_bit_length: u32) -> hv_return_t;
    /// Returns the IPA bit length of a VM configuration (macOS 13).
    pub fn hv_vm_config_get_ipa_size(
        config: hv_vm_config_t,
        ipa_bit_length: *mut u32,
    ) -> hv_return_t;
    /// Returns whether EL2 is supported on this platform (macOS 15).
    pub fn hv_vm_config_get_el2_supported(el2_supported: *mut bool) -> hv_return_t;
    /// Returns whether EL2 is enabled in a VM configuration (macOS 15).
    pub fn hv_vm_config_get_el2_enabled(
        config: hv_vm_config_t,
        el2_enabled: *mut bool,
    ) -> hv_return_t;
    /// Enables or disables EL2 in a VM configuration (macOS 15).
    pub fn hv_vm_config_set_el2_enabled(config: hv_vm_config_t, el2_enabled: bool) -> hv_return_t;
    /// Returns the default IPA granule (macOS 26).
    pub fn hv_vm_config_get_default_ipa_granule(granule: *mut hv_ipa_granule_t) -> hv_return_t;
    /// Returns the IPA granule of a VM configuration (macOS 26).
    pub fn hv_vm_config_get_ipa_granule(
        config: hv_vm_config_t,
        granule: *mut hv_ipa_granule_t,
    ) -> hv_return_t;
    /// Sets the IPA granule of a VM configuration (macOS 26).
    pub fn hv_vm_config_set_ipa_granule(
        config: hv_vm_config_t,
        granule: hv_ipa_granule_t,
    ) -> hv_return_t;

    // hv_vcpu_config.h (macOS 11)

    /// Creates a vCPU configuration object.
    pub fn hv_vcpu_config_create() -> hv_vcpu_config_t;
    /// Returns the value of a feature register for a vCPU configuration.
    pub fn hv_vcpu_config_get_feature_reg(
        config: hv_vcpu_config_t,
        feature_reg: hv_feature_reg_t,
        value: *mut u64,
    ) -> hv_return_t;
    /// Returns the eight `CCSIDR_EL1` values for a cache type.
    pub fn hv_vcpu_config_get_ccsidr_el1_sys_reg_values(
        config: hv_vcpu_config_t,
        cache_type: hv_cache_type_t,
        values: *mut u64,
    ) -> hv_return_t;

    // hv_vcpu.h (macOS 11)

    /// Creates a vCPU instance for the current thread. Pass NULL for the default configuration.
    pub fn hv_vcpu_create(
        vcpu: *mut hv_vcpu_t,
        exit: *mut *mut hv_vcpu_exit_t,
        config: hv_vcpu_config_t,
    ) -> hv_return_t;
    /// Destroys a vCPU. Must be called by the owning thread.
    pub fn hv_vcpu_destroy(vcpu: hv_vcpu_t) -> hv_return_t;
    /// Returns the value of a general purpose register. Must be called by the owning thread.
    pub fn hv_vcpu_get_reg(vcpu: hv_vcpu_t, reg: hv_reg_t, value: *mut u64) -> hv_return_t;
    /// Sets the value of a general purpose register. Must be called by the owning thread.
    pub fn hv_vcpu_set_reg(vcpu: hv_vcpu_t, reg: hv_reg_t, value: u64) -> hv_return_t;
    /// Returns the value of a SIMD&FP register. Must be called by the owning thread.
    // The wrapper struct lowers to `<16 x i8>`, the same as clang's lowering of
    // the C vector type; see `hv_simd_fp_uchar16_t`.
    #[allow(improper_ctypes)]
    pub fn hv_vcpu_get_simd_fp_reg(
        vcpu: hv_vcpu_t,
        reg: hv_simd_fp_reg_t,
        value: *mut hv_simd_fp_uchar16_t,
    ) -> hv_return_t;
    /// Sets the value of a SIMD&FP register. Must be called by the owning thread.
    // The wrapper struct lowers to `<16 x i8>`, the same as clang's lowering of
    // the C vector type; see `hv_simd_fp_uchar16_t`.
    #[allow(improper_ctypes)]
    pub fn hv_vcpu_set_simd_fp_reg(
        vcpu: hv_vcpu_t,
        reg: hv_simd_fp_reg_t,
        value: hv_simd_fp_uchar16_t,
    ) -> hv_return_t;
    /// Returns the value of a system register. Must be called by the owning thread.
    pub fn hv_vcpu_get_sys_reg(vcpu: hv_vcpu_t, reg: hv_sys_reg_t, value: *mut u64) -> hv_return_t;
    /// Sets the value of a system register. Must be called by the owning thread.
    pub fn hv_vcpu_set_sys_reg(vcpu: hv_vcpu_t, reg: hv_sys_reg_t, value: u64) -> hv_return_t;
    /// Returns whether an interrupt is pending. Must be called by the owning thread.
    pub fn hv_vcpu_get_pending_interrupt(
        vcpu: hv_vcpu_t,
        interrupt_type: hv_interrupt_type_t,
        pending: *mut bool,
    ) -> hv_return_t;
    /// Sets a pending interrupt for the next run. Must be called by the owning thread.
    pub fn hv_vcpu_set_pending_interrupt(
        vcpu: hv_vcpu_t,
        interrupt_type: hv_interrupt_type_t,
        pending: bool,
    ) -> hv_return_t;
    /// Returns whether guest debug exceptions are trapped to the host.
    pub fn hv_vcpu_get_trap_debug_exceptions(vcpu: hv_vcpu_t, value: *mut bool) -> hv_return_t;
    /// Sets whether guest debug exceptions are trapped to the host.
    pub fn hv_vcpu_set_trap_debug_exceptions(vcpu: hv_vcpu_t, value: bool) -> hv_return_t;
    /// Returns whether guest debug register accesses are trapped to the host.
    pub fn hv_vcpu_get_trap_debug_reg_accesses(vcpu: hv_vcpu_t, value: *mut bool) -> hv_return_t;
    /// Sets whether guest debug register accesses are trapped to the host.
    pub fn hv_vcpu_set_trap_debug_reg_accesses(vcpu: hv_vcpu_t, value: bool) -> hv_return_t;
    /// Runs a vCPU until the next exit. Must be called by the owning thread.
    pub fn hv_vcpu_run(vcpu: hv_vcpu_t) -> hv_return_t;
    /// Forces an immediate exit of a set of vCPUs. May be called from any thread.
    pub fn hv_vcpus_exit(vcpus: *mut hv_vcpu_t, vcpu_count: u32) -> hv_return_t;
    /// Returns the cumulative execution time of a vCPU in `mach_absolute_time()` units.
    pub fn hv_vcpu_get_exec_time(vcpu: hv_vcpu_t, time: *mut u64) -> hv_return_t;
    /// Returns whether the virtual timer is masked.
    pub fn hv_vcpu_get_vtimer_mask(vcpu: hv_vcpu_t, vtimer_is_masked: *mut bool) -> hv_return_t;
    /// Masks or unmasks the virtual timer.
    pub fn hv_vcpu_set_vtimer_mask(vcpu: hv_vcpu_t, vtimer_is_masked: bool) -> hv_return_t;
    /// Returns the virtual timer offset.
    pub fn hv_vcpu_get_vtimer_offset(vcpu: hv_vcpu_t, vtimer_offset: *mut u64) -> hv_return_t;
    /// Sets the virtual timer offset (`CNTVCT_EL0 = mach_absolute_time() - vtimer_offset`).
    pub fn hv_vcpu_set_vtimer_offset(vcpu: hv_vcpu_t, vtimer_offset: u64) -> hv_return_t;
}

unsafe extern "C" {
    /// Releases an `os_object` such as `hv_vm_config_t` or `hv_vcpu_config_t` (libSystem).
    pub fn os_release(object: *mut c_void);
}
