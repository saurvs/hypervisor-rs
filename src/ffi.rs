//! Bindings to the Hypervisor Framework

use libc::*;

pub const HV_VM_DEFAULT: uint64_t = 0 << 0;

pub const HV_VCPU_DEFAULT: uint64_t = 0;

// Hypervisor Framework return codes
pub const HV_SUCCESS     : hv_return_t = 0;
pub const HV_ERROR       : hv_return_t = 0xfae94001;
pub const HV_BUSY        : hv_return_t = 0xfae94002;
pub const HV_BAD_ARGUMENT: hv_return_t = 0xfae94003;
pub const HV_NO_RESOURCES: hv_return_t = 0xfae94005;
pub const HV_NO_DEVICE   : hv_return_t = 0xfae94006;
pub const HV_UNSUPPORTED : hv_return_t = 0xfae9400f;

// Guest physical memory region permissions for hv_vm_map() and hv_vm_protect()
pub const HV_MEMORY_READ : uint64_t = 1 << 0;
pub const HV_MEMORY_WRITE: uint64_t = 1 << 1;
pub const HV_MEMORY_EXEC : uint64_t = 1 << 2;

/// Hypervisor Framework return code
pub type hv_return_t = uint32_t;

/// Options for hv_vcpu_create()
pub type hv_vm_options_t = uint64_t;

// Creating and Destroying VM Instances
extern {
    /// Creates a VM instance for the current Mach task
    pub fn hv_vm_create(flags: hv_vm_options_t) -> hv_return_t;

    /// Destroys the VM instance associated with the current Mach task
    pub fn hv_vm_destroy() -> hv_return_t;
}

/// Type of a vCPU ID
pub type hv_vcpuid_t = c_uint;

// Creating and Managing vCPU Instances
extern {
    /// Creates a vCPU instance for the current thread
    pub fn hv_vcpu_create(vcpu: *mut hv_vcpuid_t, flags: hv_vm_options_t) -> hv_return_t;

    /// Executes a vCPU
    pub fn hv_vcpu_run(vcpu: hv_vcpuid_t) -> hv_return_t;

    /// Forces an immediate VMEXIT of a set of vCPUs of the VM
    pub fn hv_vcpu_interrupt(vcpu: *const hv_vcpuid_t, vcpu_count: c_uint) -> hv_return_t;

    /// Returns the cumulative execution time of a vCPU in nanoseconds
    pub fn hv_vcpu_get_exec_time(vcpu: hv_vcpuid_t, time: *mut uint64_t) -> hv_return_t;

    /// Forces flushing of cached vCPU state
    pub fn hv_vcpu_flush(vcpu: hv_vcpuid_t) -> hv_return_t;

    /// Invalidates the TLB of a vCPU
    pub fn hv_vcpu_invalidate_tlb(vcpu: hv_vcpuid_t) -> hv_return_t;

    /// Destroys the vCPU instance associated with the current thread
    pub fn hv_vcpu_destroy(vcpu: hv_vcpuid_t) -> hv_return_t;
}

// Accessing Registers
extern {
    /// Returns the current value of an architectural x86 register
    /// of a vCPU
    pub fn hv_vcpu_read_register(vcpu: hv_vcpuid_t, reg: super::x86Reg, value: *mut uint64_t) -> hv_return_t;

    /// Sets the value of an architectural x86 register of a vCPU
    pub fn hv_vcpu_write_register(vcpu: hv_vcpuid_t, reg: super::x86Reg, value: uint64_t) -> hv_return_t;
}

// Accessing Floating Point (FP) State
extern {
    /// Returns the current architectural x86 floating point and
    /// SIMD state of a vCPU
    pub fn hv_vcpu_read_fpstate(vcpu: hv_vcpuid_t, buffer: *mut c_void, size: size_t) -> hv_return_t;

    /// Sets the architectural x86 floating point and SIMD state of
    /// a vCPU
    pub fn hv_vcpu_write_fpstate(vcpu: hv_vcpuid_t, buffer: *const c_void, size: size_t) -> hv_return_t;
}

// Accessing Machine Specific Registers (MSRs)
extern {
    /// Enables an MSR to be used natively by the VM
    pub fn hv_vcpu_enable_native_msr(vcpu: hv_vcpuid_t, msr: uint32_t, enable: bool) -> hv_return_t;

    /// Returns the current value of an MSR of a vCPU
    pub fn hv_vcpu_read_msr(vcpu: hv_vcpuid_t, msr: uint32_t, value: *mut uint64_t) -> hv_return_t;

    /// Set the value of an MSR of a vCPU
    pub fn hv_vcpu_write_msr(vcpu: hv_vcpuid_t, msr: uint32_t, value: *const uint64_t) -> hv_return_t;
}

// Managing Timestamp-Counters (TSC)
extern {
    /// Synchronizes guest Timestamp-Counters (TSC) across all vCPUs
    pub fn hv_vm_sync_tsc(tsc: uint64_t) -> hv_return_t;
}

/// Type of a user virtual address
pub type hv_uvaddr_t = *const c_void;

/// Guest physical memory region permissions for hv_vm_map()
/// and hv_vm_protect()
pub type hv_memory_flags_t = uint64_t;

/// Type of a guest physical address
pub type hv_gpaddr_t = uint64_t;

// Managing Memory Regions
extern {
    /// Maps a region in the virtual address space of the current
    /// task into the guest physical address space of the VM
    pub fn hv_vm_map(uva: hv_uvaddr_t, gpa: hv_gpaddr_t, size: size_t, flags: hv_memory_flags_t) -> hv_return_t;

    /// Unmaps a region in the guest physical address space of the VM
    pub fn hv_vm_unmap(gpa: hv_gpaddr_t, size: size_t) -> hv_return_t;

    /// Modifies the permissions of a region in the guest physical
    /// address space of the VM
    pub fn hv_vm_protect(gpa: hv_gpaddr_t, size: size_t, flags: hv_memory_flags_t) -> hv_return_t;
}

// Managing Virtual Machine Control Structure (VMCS)
extern {
    /// Returns the current value of a VMCS field of a vCPU
    pub fn hv_vmx_vcpu_read_vmcs(vcpu: hv_vcpuid_t, field: uint32_t, value: *mut uint64_t) -> hv_return_t;

    /// Sets the value of a VMCS field of a vCPU
    pub fn hv_vmx_vcpu_write_vmcs(vcpu: hv_vcpuid_t, field: uint32_t, value: uint64_t) -> hv_return_t;

    /// Returns the VMX capabilities of the host processor
    pub fn hv_vmx_read_capability(field: super::VMXCap, value: *mut uint64_t) -> hv_return_t;

    /// Sets the address of the guest APIC for a vCPU in the
    /// guest physical address space of the VM
    pub fn hv_vmx_vcpu_set_apic_address(vcpu: hv_vcpuid_t, gpa: hv_gpaddr_t) -> hv_return_t;
}
