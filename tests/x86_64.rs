//! Runtime tests for the Intel API.
//!
//! Like the arm64 tests, these need a signed binary and a host with VT-x
//! (not a VM, not Rosetta). Where the hypervisor is unavailable they print a
//! note and pass, unless `HYPERVISOR_REQUIRE_VM=1` is set.

#![cfg(all(target_os = "macos", target_arch = "x86_64"))]

use std::alloc::{Layout, alloc_zeroed, dealloc};

use hypervisor::consts::{vmcs::*, vmx_cap::*, vmx_exit::*};
use hypervisor::{MemPerm, VMXCap, read_vmx_cap, vCPU, x86Reg};

const PAGE: usize = 4096;

fn cap2ctrl(cap: u64, ctrl: u64) -> u64 {
    (ctrl | (cap & 0xffff_ffff)) & (cap >> 32)
}

/// The program from issue #1: `mov eax, 0x90901234; hlt` with flat 32-bit
/// segments, mapped from page-aligned memory.
#[test]
fn issue_1_program() {
    if let Err(err) = hypervisor::create_vm() {
        if std::env::var_os("HYPERVISOR_REQUIRE_VM").is_some() {
            panic!("create_vm failed: {err}");
        }
        eprintln!("skipping: cannot create a VM here ({err})");
        return;
    }

    let vcpu = vCPU::new().unwrap();
    let w = |field, value| vcpu.write_vmcs(field, value).unwrap();
    let cap = |c| read_vmx_cap(&c).unwrap();

    w(VMCS_CTRL_PIN_BASED, cap2ctrl(cap(VMXCap::PINBASED), 0));
    w(
        VMCS_CTRL_CPU_BASED,
        cap2ctrl(
            cap(VMXCap::PROCBASED),
            CPU_BASED_HLT | CPU_BASED_CR8_LOAD | CPU_BASED_CR8_STORE,
        ),
    );
    w(
        VMCS_CTRL_CPU_BASED2,
        cap2ctrl(cap(VMXCap::PROCBASED2), CPU_BASED2_UNRESTRICTED),
    );
    w(VMCS_CTRL_VMENTRY_CONTROLS, cap2ctrl(cap(VMXCap::ENTRY), 0));
    w(VMCS_CTRL_EXC_BITMAP, 0xffff_ffff);
    w(VMCS_CTRL_CR0_MASK, 0xffff_ffff);
    w(VMCS_CTRL_CR0_SHADOW, 0xffff_ffff);
    w(VMCS_CTRL_CR4_MASK, 0xffff_ffff);
    w(VMCS_CTRL_CR4_SHADOW, 0xffff_ffff);

    w(VMCS_GUEST_CS, 1 << 3);
    w(VMCS_GUEST_CS_AR, 0xc093);
    w(VMCS_GUEST_CS_LIMIT, 0xffff_ffff);
    w(VMCS_GUEST_CS_BASE, 0);
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
        w(sel, 2 << 3);
        w(ar, 0xc093);
        w(limit, 0xffff_ffff);
        w(base, 0);
    }
    w(VMCS_GUEST_LDTR, 0);
    w(VMCS_GUEST_LDTR_LIMIT, 0);
    w(VMCS_GUEST_LDTR_AR, 0x10000);
    w(VMCS_GUEST_LDTR_BASE, 0);
    w(VMCS_GUEST_TR, 0);
    w(VMCS_GUEST_TR_LIMIT, 0);
    w(VMCS_GUEST_TR_AR, 0x83);
    w(VMCS_GUEST_TR_BASE, 0);
    w(VMCS_GUEST_GDTR_LIMIT, 0);
    w(VMCS_GUEST_GDTR_BASE, 0);
    w(VMCS_GUEST_IDTR_LIMIT, 0);
    w(VMCS_GUEST_IDTR_BASE, 0);
    w(VMCS_GUEST_CR0, 0x20);
    w(VMCS_GUEST_CR3, 0);
    w(VMCS_GUEST_CR4, 0x2000);

    let layout = Layout::from_size_align(PAGE, PAGE).unwrap();
    let mem = unsafe { alloc_zeroed(layout) };
    assert!(!mem.is_null());
    let code = [0xb8u8, 0x34, 0x12, 0x90, 0x90, 0xf4];
    unsafe { mem.copy_from_nonoverlapping(code.as_ptr(), code.len()) };
    unsafe {
        hypervisor::map_mem(
            std::slice::from_raw_parts(mem, PAGE),
            0,
            &MemPerm::ExecAndWrite,
        )
        .unwrap()
    };

    vcpu.write_register(&x86Reg::RIP, 0).unwrap();
    vcpu.write_register(&x86Reg::RFLAGS, 0x2).unwrap();
    vcpu.write_register(&x86Reg::RAX, 0).unwrap();
    vcpu.write_register(&x86Reg::RBX, 0x1234).unwrap();
    assert_eq!(vcpu.read_register(&x86Reg::RBX).unwrap(), 0x1234);

    loop {
        vcpu.run().unwrap();
        let reason = vcpu.read_vmcs(VMCS_RO_EXIT_REASON).unwrap();
        if reason == VMX_REASON_EPT_VIOLATION || reason == VMX_REASON_IRQ {
            continue;
        }
        assert_eq!(reason, VMX_REASON_HLT);
        break;
    }
    assert_eq!(vcpu.read_register(&x86Reg::RAX).unwrap(), 0x9090_1234);
    assert_eq!(vcpu.read_register(&x86Reg::RIP).unwrap(), 5);
    assert!(vcpu.exec_time().unwrap() > 0);

    vcpu.destroy().unwrap();
    hypervisor::unmap_mem(0, PAGE).unwrap();
    hypervisor::destroy_vm().unwrap();
    unsafe { dealloc(mem, layout) };
}
