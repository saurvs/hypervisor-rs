//! Runs a few instructions on an Intel Mac and prints the result:
//!
//! ```text
//! b8 34 12 90 90   mov eax, 0x90901234 (32-bit segments)
//! f4               hlt
//! ```
//!
//! This is the C program from issue #1, ported to the current API; it should
//! print `rax = 0x90901234` and `rip = 0x5`. The detail that matters is that
//! `map_mem` needs page-aligned memory whose size is a multiple of the page
//! size. A 6-byte `Vec<u8>` is rejected by the framework, and the guest then
//! only ever sees EPT violations.
//!
//! Run it on an Intel Mac with `cargo run --example x86_64_hello`.

#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::alloc::{Layout, alloc_zeroed, dealloc};

    use hypervisor::consts::{vmcs::*, vmx_cap::*, vmx_exit::*};
    use hypervisor::{MemPerm, VMXCap, read_vmx_cap, vCPU, x86Reg};

    /// Combines the allowed-0 (low 32 bits) and allowed-1 (high 32 bits)
    /// settings of a VMX capability with the controls we want.
    fn cap2ctrl(cap: u64, ctrl: u64) -> u64 {
        (ctrl | (cap & 0xffff_ffff)) & (cap >> 32)
    }

    const PAGE: usize = 4096;
    // mov eax, 0x90901234 (with 32-bit segments); hlt
    const CODE: [u8; 6] = [0xb8, 0x34, 0x12, 0x90, 0x90, 0xf4];

    hypervisor::create_vm()?;
    let vcpu = vCPU::new()?;

    let pinbased = read_vmx_cap(&VMXCap::PINBASED)?;
    let procbased = read_vmx_cap(&VMXCap::PROCBASED)?;
    let procbased2 = read_vmx_cap(&VMXCap::PROCBASED2)?;
    let entry = read_vmx_cap(&VMXCap::ENTRY)?;

    vcpu.write_vmcs(VMCS_CTRL_PIN_BASED, cap2ctrl(pinbased, 0))?;
    vcpu.write_vmcs(
        VMCS_CTRL_CPU_BASED,
        cap2ctrl(
            procbased,
            CPU_BASED_HLT | CPU_BASED_CR8_LOAD | CPU_BASED_CR8_STORE,
        ),
    )?;
    vcpu.write_vmcs(
        VMCS_CTRL_CPU_BASED2,
        cap2ctrl(procbased2, CPU_BASED2_UNRESTRICTED),
    )?;
    vcpu.write_vmcs(VMCS_CTRL_VMENTRY_CONTROLS, cap2ctrl(entry, 0))?;
    vcpu.write_vmcs(VMCS_CTRL_EXC_BITMAP, 0xffff_ffff)?;
    vcpu.write_vmcs(VMCS_CTRL_CR0_MASK, 0xffff_ffff)?;
    vcpu.write_vmcs(VMCS_CTRL_CR0_SHADOW, 0xffff_ffff)?;
    vcpu.write_vmcs(VMCS_CTRL_CR4_MASK, 0xffff_ffff)?;
    vcpu.write_vmcs(VMCS_CTRL_CR4_SHADOW, 0xffff_ffff)?;

    // Flat 4 GiB segments with 32-bit default operand size.
    vcpu.write_vmcs(VMCS_GUEST_CS, 1 << 3)?;
    vcpu.write_vmcs(VMCS_GUEST_CS_AR, 0xc093)?;
    vcpu.write_vmcs(VMCS_GUEST_CS_LIMIT, 0xffff_ffff)?;
    vcpu.write_vmcs(VMCS_GUEST_CS_BASE, 0)?;
    for (sel, ar, limit, base) in [
        (
            VMCS_GUEST_DS,
            VMCS_GUEST_DS_AR,
            VMCS_GUEST_DS_LIMIT,
            VMCS_GUEST_DS_BASE,
        ),
        (
            VMCS_GUEST_ES,
            VMCS_GUEST_ES_AR,
            VMCS_GUEST_ES_LIMIT,
            VMCS_GUEST_ES_BASE,
        ),
        (
            VMCS_GUEST_FS,
            VMCS_GUEST_FS_AR,
            VMCS_GUEST_FS_LIMIT,
            VMCS_GUEST_FS_BASE,
        ),
        (
            VMCS_GUEST_GS,
            VMCS_GUEST_GS_AR,
            VMCS_GUEST_GS_LIMIT,
            VMCS_GUEST_GS_BASE,
        ),
        (
            VMCS_GUEST_SS,
            VMCS_GUEST_SS_AR,
            VMCS_GUEST_SS_LIMIT,
            VMCS_GUEST_SS_BASE,
        ),
    ] {
        vcpu.write_vmcs(sel, 2 << 3)?;
        vcpu.write_vmcs(ar, 0xc093)?;
        vcpu.write_vmcs(limit, 0xffff_ffff)?;
        vcpu.write_vmcs(base, 0)?;
    }

    vcpu.write_vmcs(VMCS_GUEST_LDTR, 0)?;
    vcpu.write_vmcs(VMCS_GUEST_LDTR_LIMIT, 0)?;
    vcpu.write_vmcs(VMCS_GUEST_LDTR_AR, 0x10000)?;
    vcpu.write_vmcs(VMCS_GUEST_LDTR_BASE, 0)?;
    vcpu.write_vmcs(VMCS_GUEST_TR, 0)?;
    vcpu.write_vmcs(VMCS_GUEST_TR_LIMIT, 0)?;
    vcpu.write_vmcs(VMCS_GUEST_TR_AR, 0x83)?;
    vcpu.write_vmcs(VMCS_GUEST_TR_BASE, 0)?;
    vcpu.write_vmcs(VMCS_GUEST_GDTR_LIMIT, 0)?;
    vcpu.write_vmcs(VMCS_GUEST_GDTR_BASE, 0)?;
    vcpu.write_vmcs(VMCS_GUEST_IDTR_LIMIT, 0)?;
    vcpu.write_vmcs(VMCS_GUEST_IDTR_BASE, 0)?;

    // CR0.PE stays clear (the unrestricted guest control allows that); the
    // 32-bit segment attributes above still make `b8` take a 32-bit immediate.
    vcpu.write_vmcs(VMCS_GUEST_CR0, 0x20)?; // NE
    vcpu.write_vmcs(VMCS_GUEST_CR3, 0)?;
    vcpu.write_vmcs(VMCS_GUEST_CR4, 0x2000)?; // VMXE

    // Page-aligned, page-sized guest memory at guest physical address 0.
    let layout = Layout::from_size_align(PAGE, PAGE)?;
    let mem = unsafe { alloc_zeroed(layout) };
    if mem.is_null() {
        return Err("out of memory".into());
    }
    unsafe { mem.copy_from_nonoverlapping(CODE.as_ptr(), CODE.len()) };
    // SAFETY: `mem` stays allocated until after the VM is destroyed below,
    // and the host does not touch it while the guest runs.
    unsafe {
        hypervisor::map_mem(
            core::slice::from_raw_parts(mem, PAGE),
            0,
            &MemPerm::ExecAndWrite,
        )?
    };

    vcpu.write_register(&x86Reg::RIP, 0)?;
    vcpu.write_register(&x86Reg::RFLAGS, 0x2)?;
    vcpu.write_register(&x86Reg::RAX, 0)?;

    loop {
        vcpu.run()?;
        let reason = vcpu.read_vmcs(VMCS_RO_EXIT_REASON)?;
        println!("exit reason: {reason}");
        if reason == VMX_REASON_EPT_VIOLATION || reason == VMX_REASON_IRQ {
            continue;
        }
        break;
    }

    println!("rax = {:#x}", vcpu.read_register(&x86Reg::RAX)?);
    println!("rip = {:#x}", vcpu.read_register(&x86Reg::RIP)?);

    vcpu.destroy()?;
    hypervisor::unmap_mem(0, PAGE)?;
    hypervisor::destroy_vm()?;
    unsafe { dealloc(mem, layout) };
    Ok(())
}

#[cfg(not(all(target_os = "macos", target_arch = "x86_64")))]
fn main() {
    eprintln!("this example needs an Intel Mac");
}
