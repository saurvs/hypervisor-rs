/*!
Rust bindings to Apple's [Hypervisor.framework], for hardware-accelerated
virtual machines on macOS.

The framework has a different API on each Mac architecture, and so does this
crate. Both are available at the crate root, selected by the target:

* **Apple silicon (`aarch64`)**: a safe API built around `Vm`, `Vcpu` and
  `GuestMemory`. Requires macOS 11 or newer.
* **Intel (`x86_64`)**: the original VT-x API (`create_vm`, `vCPU`, `map_mem`,
  VMCS access, the `consts` module). Requires macOS 10.10 or newer and a CPU
  with VT-x, EPT and unrestricted mode (`sysctl kern.hv_support` prints `1`).

On other targets the crate is empty, so it can sit in a cross-platform
dependency tree.

The raw C API of the current architecture is in the `ffi` module. On
docs.rs, switch between the `aarch64-apple-darwin` and `x86_64-apple-darwin`
targets to see each API.

# Entitlement

Since macOS 11, a process may only use the hypervisor if it is signed with the
`com.apple.security.hypervisor` entitlement; otherwise VM creation fails with
[`Error::Denied`]. An ad-hoc signature is enough for local development:

```text
codesign --sign - --entitlements entitlements.plist --force path/to/binary
```

The repository's `scripts/run-signed.sh` does this and is configured as the
cargo runner, so `cargo run --example` and `cargo test` work out of the box in
a checkout.

# Example (Apple silicon)

Run two instructions in a guest and read back a register:

```no_run
# #[cfg(target_arch = "aarch64")]
# fn main() -> Result<(), hypervisor::Error> {
use hypervisor::{ExceptionClass, GuestMemory, MemPerm, Reg, Vcpu, VcpuExit, Vm};

const MOV_X0_42: u32 = 0xd280_0540; // mov x0, #42
const HVC_0: u32 = 0xd400_0002; // hvc #0

let vm = Vm::new()?;
let mut ram = GuestMemory::new(&vm, 0x1_0000, 0x4000, MemPerm::RX)?;
ram.write(0, &MOV_X0_42.to_le_bytes())?;
ram.write(4, &HVC_0.to_le_bytes())?;

let vcpu = Vcpu::new(&vm)?;
vcpu.set_reg(Reg::Pc, ram.ipa())?;
vcpu.set_reg(Reg::Cpsr, 0x3c5)?; // EL1h, interrupts masked

match vcpu.run()? {
    VcpuExit::Exception(e) if e.class() == ExceptionClass::Hvc64 => {
        assert_eq!(vcpu.reg(Reg::X0)?, 42);
    }
    other => panic!("unexpected exit: {other:?}"),
}
# Ok(())
# }
# #[cfg(not(target_arch = "aarch64"))]
# fn main() {}
```

[Hypervisor.framework]: https://developer.apple.com/documentation/hypervisor
*/

#![cfg(target_os = "macos")]
#![warn(
    missing_debug_implementations,
    rust_2018_idioms,
    unsafe_op_in_unsafe_fn
)]

mod error;
pub use error::{Error, Result};

#[cfg(target_arch = "aarch64")]
mod aarch64;
#[cfg(target_arch = "aarch64")]
pub use aarch64::*;

#[cfg(target_arch = "x86_64")]
mod x86_64;
#[cfg(target_arch = "x86_64")]
pub use x86_64::*;
