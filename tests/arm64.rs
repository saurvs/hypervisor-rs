//! Runtime tests for the Apple silicon API.
//!
//! These run real (tiny) guests, so the test binary must be signed with the
//! `com.apple.security.hypervisor` entitlement. In a checkout, `cargo test`
//! does that through the runner in `.cargo/config.toml`. When the hypervisor
//! is unavailable (unsigned binary, or a CI machine that is itself a VM
//! without nested virtualization) each test prints a note and passes; set
//! `HYPERVISOR_REQUIRE_VM=1` to make that a failure instead.

#![cfg(all(target_os = "macos", target_arch = "aarch64"))]

use std::sync::Mutex;
use std::time::Duration;

use hypervisor::{
    CacheType, Error, ExceptionClass, FeatureReg, GuestMemory, InterruptType, MemPerm, Reg,
    SimdFpReg, SysReg, Vcpu, VcpuConfig, VcpuExit, Vm,
};

/// Guest physical address where test code is loaded.
const CODE: u64 = 0x1_0000;
/// An address with nothing mapped, for MMIO-style data aborts.
const MMIO: u64 = 0x2000_0000;
/// `CPSR` for EL1h with D, A, I and F masked.
const CPSR_EL1H_MASKED: u64 = 0x3c5;

const MOV_X0_42: u32 = 0xd280_0540;
const HVC_0: u32 = 0xd400_0002;
const HVC_1: u32 = 0xd400_0022;
const HVC_0X1234: u32 = 0xd402_4682;
const UMOV_X0_V1_D0: u32 = 0x4e08_3c20;
const UMOV_X1_V1_D1: u32 = 0x4e18_3c21;
const MRS_X0_TPIDR_EL1: u32 = 0xd538_d080;
const STR_X1_X2: u32 = 0xf900_0041;
const STR_W1_X2: u32 = 0xb900_0041;
const LDR_X3_X2: u32 = 0xf940_0043;
const B_SELF: u32 = 0x1400_0000;
const MSR_DAIFCLR_2: u32 = 0xd503_42ff;
const MRS_X0_PMCR_EL0: u32 = 0xd53b_9c00;
const BRK_7: u32 = 0xd420_00e0;
const ADD_X0_X0_X1: u32 = 0x8b01_0000;

/// A process can only have one VM, so tests take turns.
static VM_LOCK: Mutex<()> = Mutex::new(());

fn with_vm(test: impl FnOnce(&Vm)) {
    let _guard = VM_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    match Vm::new() {
        Ok(vm) => test(&vm),
        Err(err) if std::env::var_os("HYPERVISOR_REQUIRE_VM").is_none() => {
            eprintln!("skipping: cannot create a VM here ({err})");
        }
        Err(err) => panic!("Vm::new failed: {err}"),
    }
}

/// Maps one page of code at `CODE` and returns a vCPU about to run it.
fn load<'vm>(vm: &'vm Vm, code: &[u32]) -> (GuestMemory<'vm>, Vcpu<'vm>) {
    let mut ram = GuestMemory::new(vm, CODE, hypervisor::page_size(), MemPerm::RWX).unwrap();
    let bytes: Vec<u8> = code.iter().flat_map(|i| i.to_le_bytes()).collect();
    ram.write(0, &bytes).unwrap();
    let vcpu = Vcpu::new(vm).unwrap();
    vcpu.set_reg(Reg::Pc, CODE).unwrap();
    vcpu.set_reg(Reg::Cpsr, CPSR_EL1H_MASKED).unwrap();
    (ram, vcpu)
}

fn expect_exception(exit: VcpuExit) -> hypervisor::Exception {
    match exit {
        VcpuExit::Exception(e) => e,
        other => panic!("expected an exception exit, got {other:?}"),
    }
}

fn expect_hvc(vcpu: &Vcpu<'_>, imm: u16) {
    let e = expect_exception(vcpu.run().unwrap());
    assert_eq!(e.class(), ExceptionClass::Hvc64, "{e:?}");
    assert_eq!(e.immediate(), Some(imm));
}

#[test]
fn hvc_exit_and_registers() {
    with_vm(|vm| {
        let (_ram, vcpu) = load(vm, &[MOV_X0_42, HVC_0X1234]);
        let e = expect_exception(vcpu.run().unwrap());
        assert_eq!(e.class(), ExceptionClass::Hvc64);
        assert!(e.is_32bit_instruction());
        assert_eq!(e.immediate(), Some(0x1234));
        assert_eq!(vcpu.last_exit(), VcpuExit::Exception(e));
        assert_eq!(vcpu.reg(Reg::X0).unwrap(), 42);
        // The PC is left after the hvc.
        assert_eq!(vcpu.reg(Reg::Pc).unwrap(), CODE + 8);
        assert!(vcpu.exec_time().unwrap() > 0);
    });
}

#[test]
fn guest_sees_host_registers() {
    with_vm(|vm| {
        let (_ram, vcpu) = load(vm, &[ADD_X0_X0_X1, HVC_0]);
        vcpu.set_reg(Reg::X0, 40).unwrap();
        vcpu.set_reg(Reg::X1, 2).unwrap();
        expect_hvc(&vcpu, 0);
        assert_eq!(vcpu.reg(Reg::X0).unwrap(), 42);
    });
}

#[test]
fn general_register_round_trip() {
    with_vm(|vm| {
        let vcpu = Vcpu::new(vm).unwrap();
        for n in 0..31u8 {
            let reg = Reg::x(n).unwrap();
            vcpu.set_reg(reg, 0x1111_0000_0000 + u64::from(n)).unwrap();
        }
        for n in 0..31u8 {
            let reg = Reg::x(n).unwrap();
            assert_eq!(vcpu.reg(reg).unwrap(), 0x1111_0000_0000 + u64::from(n));
        }
        vcpu.set_reg(Reg::Pc, 0x4000).unwrap();
        assert_eq!(vcpu.reg(Reg::Pc).unwrap(), 0x4000);
        vcpu.set_reg(Reg::Fpcr, 1 << 24).unwrap(); // FZ
        assert_eq!(vcpu.reg(Reg::Fpcr).unwrap(), 1 << 24);
    });
}

#[test]
fn simd_register_round_trip_and_guest_view() {
    with_vm(|vm| {
        let (_ram, vcpu) = load(vm, &[UMOV_X0_V1_D0, UMOV_X1_V1_D1, HVC_0]);
        // Let EL1 use SIMD without trapping (CPACR_EL1.FPEN = 0b11).
        vcpu.set_sys_reg(SysReg::CPACR_EL1, 0b11 << 20).unwrap();
        let value: u128 = 0x0011_2233_4455_6677_8899_aabb_ccdd_eeff;
        vcpu.set_simd_fp_reg(SimdFpReg::Q1, value).unwrap();
        assert_eq!(vcpu.simd_fp_reg(SimdFpReg::Q1).unwrap(), value);
        vcpu.set_simd_fp_reg(SimdFpReg::Q31, !value).unwrap();
        assert_eq!(vcpu.simd_fp_reg(SimdFpReg::Q31).unwrap(), !value);
        expect_hvc(&vcpu, 0);
        assert_eq!(vcpu.reg(Reg::X0).unwrap(), value as u64);
        assert_eq!(vcpu.reg(Reg::X1).unwrap(), (value >> 64) as u64);
    });
}

#[test]
fn system_register_round_trip_and_guest_view() {
    with_vm(|vm| {
        let (_ram, vcpu) = load(vm, &[MRS_X0_TPIDR_EL1, HVC_0]);
        vcpu.set_sys_reg(SysReg::TPIDR_EL1, 0xfeed_f00d).unwrap();
        assert_eq!(vcpu.sys_reg(SysReg::TPIDR_EL1).unwrap(), 0xfeed_f00d);
        vcpu.set_sys_reg(SysReg::SP_EL1, 0x8000).unwrap();
        assert_eq!(vcpu.sys_reg(SysReg::SP_EL1).unwrap(), 0x8000);
        expect_hvc(&vcpu, 0);
        assert_eq!(vcpu.reg(Reg::X0).unwrap(), 0xfeed_f00d);
    });
}

#[test]
fn guest_writes_are_visible_to_host() {
    with_vm(|vm| {
        let (ram, vcpu) = load(vm, &[STR_X1_X2, HVC_0]);
        vcpu.set_reg(Reg::X1, 0x0123_4567_89ab_cdef).unwrap();
        vcpu.set_reg(Reg::X2, CODE + 0x100).unwrap();
        expect_hvc(&vcpu, 0);
        let mut buf = [0u8; 8];
        ram.read(0x100, &mut buf).unwrap();
        assert_eq!(u64::from_le_bytes(buf), 0x0123_4567_89ab_cdef);
    });
}

#[test]
fn mmio_store_and_load_are_data_aborts() {
    with_vm(|vm| {
        let (_ram, vcpu) = load(vm, &[STR_W1_X2, LDR_X3_X2, HVC_0]);
        vcpu.set_reg(Reg::X1, 0xdead_beef).unwrap();
        vcpu.set_reg(Reg::X2, MMIO).unwrap();

        // str w1, [x2]
        let e = expect_exception(vcpu.run().unwrap());
        assert_eq!(e.class(), ExceptionClass::DataAbortLowerEl);
        let abort = e.data_abort().unwrap();
        assert!(abort.valid && abort.write && !abort.sixty_four);
        assert_eq!((abort.size, abort.register, abort.address), (4, 1, MMIO));
        let value = vcpu.reg(Reg::x(abort.register).unwrap()).unwrap();
        assert_eq!(value as u32, 0xdead_beef);
        // Emulate the store and skip the instruction.
        vcpu.set_reg(Reg::Pc, vcpu.reg(Reg::Pc).unwrap() + 4)
            .unwrap();

        // ldr x3, [x2]
        let e = expect_exception(vcpu.run().unwrap());
        let abort = e.data_abort().unwrap();
        assert!(abort.valid && !abort.write && abort.sixty_four);
        assert_eq!((abort.size, abort.register, abort.address), (8, 3, MMIO));
        vcpu.set_reg(Reg::X3, 0x5555).unwrap();
        vcpu.set_reg(Reg::Pc, vcpu.reg(Reg::Pc).unwrap() + 4)
            .unwrap();

        expect_hvc(&vcpu, 0);
        assert_eq!(vcpu.reg(Reg::X3).unwrap(), 0x5555);
    });
}

#[test]
fn trapped_system_register_access_is_decoded() {
    with_vm(|vm| {
        // With EL2 disabled (the default), PMU registers trap to the host.
        let (_ram, vcpu) = load(vm, &[MRS_X0_PMCR_EL0, HVC_0]);
        let e = expect_exception(vcpu.run().unwrap());
        assert_eq!(e.class(), ExceptionClass::SysReg64, "{e:?}");
        let access = e.sys_reg_access().unwrap();
        // PMCR_EL0 is op0=3, op1=3, CRn=9, CRm=12, op2=0.
        assert_eq!(access.reg, SysReg(0xdce0));
        assert_eq!(access.register, 0);
        assert!(access.read);
    });
}

#[test]
fn brk_traps_when_debug_exceptions_are_trapped() {
    with_vm(|vm| {
        let (_ram, vcpu) = load(vm, &[BRK_7]);
        vcpu.set_trap_debug_exceptions(true).unwrap();
        assert!(vcpu.trap_debug_exceptions().unwrap());
        vcpu.set_trap_debug_reg_accesses(true).unwrap();
        assert!(vcpu.trap_debug_reg_accesses().unwrap());
        let e = expect_exception(vcpu.run().unwrap());
        assert_eq!(e.class(), ExceptionClass::Brk64);
        assert_eq!(e.immediate(), Some(7));
    });
}

#[test]
fn exit_from_another_thread_cancels_run() {
    with_vm(|vm| {
        let (_ram, vcpu) = load(vm, &[B_SELF]);
        let handle = vcpu.handle();
        assert_eq!(handle.id(), vcpu.id());

        // An exit requested before running makes the next run return at once.
        handle.exit().unwrap();
        assert_eq!(vcpu.run().unwrap(), VcpuExit::Canceled);

        let kicker = std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(20));
            handle.exit().unwrap();
        });
        assert_eq!(vcpu.run().unwrap(), VcpuExit::Canceled);
        kicker.join().unwrap();
        assert_eq!(vcpu.reg(Reg::Pc).unwrap(), CODE);

        hypervisor::exit_vcpus(&[vcpu.handle()]).unwrap();
        assert_eq!(vcpu.run().unwrap(), VcpuExit::Canceled);
    });
}

#[test]
fn pending_irq_is_delivered_to_the_guest() {
    with_vm(|vm| {
        // Vector table at CODE + 0x800; the "current EL with SPx, IRQ" entry
        // is at offset 0x280.
        let mut code = vec![0u32; (0x800 + 0x284) / 4];
        code[0] = MSR_DAIFCLR_2; // unmask IRQs
        code[1] = B_SELF;
        code[(0x800 + 0x280) / 4] = HVC_1;
        let (_ram, vcpu) = load(vm, &code);
        vcpu.set_sys_reg(SysReg::VBAR_EL1, CODE + 0x800).unwrap();

        vcpu.set_pending_interrupt(InterruptType::Irq, true)
            .unwrap();
        assert!(vcpu.pending_interrupt(InterruptType::Irq).unwrap());
        assert!(!vcpu.pending_interrupt(InterruptType::Fiq).unwrap());
        expect_hvc(&vcpu, 1);
        // The guest took the IRQ from the `b .` loop.
        assert_eq!(vcpu.sys_reg(SysReg::ELR_EL1).unwrap(), CODE + 4);
    });
}

#[test]
fn vtimer_fires_and_masks_itself() {
    with_vm(|vm| {
        let (_ram, vcpu) = load(vm, &[B_SELF]);
        vcpu.set_vtimer_offset(0).unwrap();
        assert_eq!(vcpu.vtimer_offset().unwrap(), 0);
        vcpu.set_vtimer_mask(false).unwrap();
        assert!(!vcpu.vtimer_mask().unwrap());
        // Enable the virtual timer with a compare value in the past.
        vcpu.set_sys_reg(SysReg::CNTV_CVAL_EL0, 0).unwrap();
        vcpu.set_sys_reg(SysReg::CNTV_CTL_EL0, 1).unwrap();
        assert_eq!(vcpu.run().unwrap(), VcpuExit::VtimerActivated);
        assert!(vcpu.vtimer_mask().unwrap());
    });
}

#[test]
fn one_vcpu_per_thread() {
    with_vm(|vm| {
        let mut ram = GuestMemory::new(vm, CODE, hypervisor::page_size(), MemPerm::RX).unwrap();
        let bytes: Vec<u8> = [ADD_X0_X0_X1, HVC_0]
            .iter()
            .flat_map(|i| i.to_le_bytes())
            .collect();
        ram.write(0, &bytes).unwrap();

        let threads = hypervisor::max_vcpu_count().unwrap().min(4);
        assert!(threads >= 1);
        std::thread::scope(|s| {
            for t in 0..u64::from(threads) {
                s.spawn(move || {
                    let vcpu = Vcpu::new(vm).unwrap();
                    // A thread can only own one vCPU at a time.
                    assert!(Vcpu::new(vm).is_err());
                    vcpu.set_reg(Reg::Pc, CODE).unwrap();
                    vcpu.set_reg(Reg::Cpsr, CPSR_EL1H_MASKED).unwrap();
                    vcpu.set_reg(Reg::X0, 100 * t).unwrap();
                    vcpu.set_reg(Reg::X1, t).unwrap();
                    expect_hvc(&vcpu, 0);
                    assert_eq!(vcpu.reg(Reg::X0).unwrap(), 101 * t);
                });
            }
        });
    });
}

#[test]
fn only_one_vm_per_process() {
    with_vm(|_vm| {
        assert!(Vm::new().is_err());
    });
}

#[test]
fn guest_memory_validation() {
    with_vm(|vm| {
        let page = hypervisor::page_size();
        assert_eq!(
            GuestMemory::new(vm, CODE, 0, MemPerm::RW).unwrap_err(),
            Error::BadArgument
        );
        // Sizes are rounded up to whole pages.
        let mut ram = GuestMemory::new(vm, CODE, 1, MemPerm::RW).unwrap();
        assert_eq!(ram.size(), page);
        assert_eq!(ram.ipa(), CODE);
        assert_eq!(ram.as_ptr() as usize % page, 0);
        // Overlapping mappings are rejected.
        assert!(GuestMemory::new(vm, CODE, page, MemPerm::RW).is_err());
        // Out-of-bounds accesses are rejected.
        assert_eq!(ram.write(page - 1, &[0, 0]), Err(Error::BadArgument));
        assert_eq!(ram.read(usize::MAX, &mut [0]), Err(Error::BadArgument));
        ram.write(page - 1, &[7]).unwrap();
        let mut b = [0];
        ram.read(page - 1, &mut b).unwrap();
        assert_eq!(b, [7]);
        ram.protect(MemPerm::READ).unwrap();
        drop(ram);
        // Once dropped, the range can be mapped again.
        let _again = GuestMemory::new(vm, CODE, page, MemPerm::RW).unwrap();
        // Misaligned guest addresses are rejected by the framework.
        assert!(GuestMemory::new(vm, MMIO + 1, page, MemPerm::RW).is_err());
    });
}

#[test]
fn vcpu_config_feature_registers() {
    with_vm(|vm| {
        let config = VcpuConfig::new().unwrap();
        let pfr0 = config.feature_reg(FeatureReg::IdAa64Pfr0El1).unwrap();
        // ID_AA64PFR0_EL1.EL1 (bits 7:4) is non-zero: AArch64 at EL1.
        assert_ne!((pfr0 >> 4) & 0xf, 0);
        config.feature_reg(FeatureReg::IdAa64Mmfr0El1).unwrap();
        config.ccsidr_el1_values(CacheType::Data).unwrap();
        config.ccsidr_el1_values(CacheType::Instruction).unwrap();
        let vcpu = Vcpu::with_config(vm, &config).unwrap();
        vcpu.set_reg(Reg::X0, 1).unwrap();
    });
}
