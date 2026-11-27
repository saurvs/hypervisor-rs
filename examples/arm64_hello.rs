//! Runs a two-instruction guest on Apple silicon:
//!
//! ```text
//! mov x0, #42
//! hvc #0
//! ```
//!
//! and prints why the vCPU exited and what the guest left in `X0`.
//!
//! Run it from a checkout with `cargo run --example arm64_hello`. The cargo
//! runner configured in `.cargo/config.toml` signs the binary with the
//! `com.apple.security.hypervisor` entitlement first (see
//! `scripts/run-signed.sh`).

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use hypervisor::{ExceptionClass, GuestMemory, MemPerm, Reg, Vcpu, VcpuExit, Vm};

    /// Guest physical address of the code page.
    const CODE_IPA: u64 = 0x1_0000;
    const GUEST_CODE: [u32; 2] = [
        0xd280_0540, // mov x0, #42
        0xd400_0002, // hvc #0
    ];

    let vm = Vm::new().inspect_err(|e| {
        eprintln!("could not create a VM: {e}");
        eprintln!("hint: run through scripts/run-signed.sh to add the entitlement");
    })?;

    // One page of guest RAM holding the code; the guest's MMU is off, so
    // guest virtual addresses are guest physical addresses.
    let mut ram = GuestMemory::new(&vm, CODE_IPA, hypervisor::page_size(), MemPerm::RX)?;
    for (i, insn) in GUEST_CODE.iter().enumerate() {
        ram.write(i * 4, &insn.to_le_bytes())?;
    }

    let vcpu = Vcpu::new(&vm)?;
    vcpu.set_reg(Reg::Pc, CODE_IPA)?;
    vcpu.set_reg(Reg::Cpsr, 0x3c5)?; // EL1h with D, A, I and F masked

    loop {
        let exit = vcpu.run()?;
        println!("exit: {exit:?}");
        match exit {
            VcpuExit::Exception(e) if e.class() == ExceptionClass::Hvc64 => {
                println!(
                    "hvc #{} at pc {:#x}",
                    e.immediate().unwrap_or(0),
                    vcpu.reg(Reg::Pc)?
                );
                println!("x0 = {}", vcpu.reg(Reg::X0)?);
                break;
            }
            VcpuExit::Canceled => continue,
            other => return Err(format!("unexpected exit: {other:?}").into()),
        }
    }
    Ok(())
}

#[cfg(not(all(target_os = "macos", target_arch = "aarch64")))]
fn main() {
    eprintln!("this example needs an Apple silicon Mac");
}
