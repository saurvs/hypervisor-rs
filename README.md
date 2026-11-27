# hypervisor-rs

[![crates.io](https://img.shields.io/crates/v/hypervisor.svg)](https://crates.io/crates/hypervisor)
[![docs.rs](https://img.shields.io/docsrs/hypervisor)](https://docs.rs/hypervisor)
[![CI](https://github.com/saurvs/hypervisor-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/saurvs/hypervisor-rs/actions/workflows/ci.yml)
[![MSRV 1.85](https://img.shields.io/badge/MSRV-1.85-blue.svg)](#minimum-supported-rust-version)
[![license: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE.md)

`hypervisor` is a Rust library for running hardware-accelerated virtual
machines on macOS through Apple's
[Hypervisor.framework](https://developer.apple.com/documentation/hypervisor).

The framework has a different API on each Mac architecture, and so does this
crate:

| Mac | API | Requirements |
| --- | --- | --- |
| Apple silicon (`aarch64`) | Safe `Vm` / `Vcpu` / `GuestMemory` API | macOS 11+ |
| Intel (`x86_64`) | The original VT-x API: `create_vm`, `vCPU`, `map_mem`, VMCS fields, `consts` | macOS 10.10+, a CPU with VT-x, EPT and unrestricted mode (`sysctl kern.hv_support` prints `1`) |

Both expose the raw C API of their architecture in `hypervisor::ffi`. On
other operating systems the crate compiles to nothing.

**Documentation:** [docs.rs/hypervisor](https://docs.rs/hypervisor) (use the
platform menu to switch between `aarch64-apple-darwin` and
`x86_64-apple-darwin`). The old pages at saurvs.github.io/hypervisor-rs
describe 0.0.x.

## Installation

```shell
cargo add hypervisor
```

## The hypervisor entitlement

Since macOS 11, only processes signed with the
`com.apple.security.hypervisor` entitlement may create a VM; otherwise you get
`Error::Denied`. An ad-hoc signature is enough for local use:

```shell
codesign --sign - --entitlements entitlements.plist --force target/debug/my-vmm
```

with this `entitlements.plist`:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>com.apple.security.hypervisor</key>
    <true/>
</dict>
</plist>
```

[`scripts/run-signed.sh`](scripts/run-signed.sh) signs a binary and runs it.
To get the same in your own project, copy it and `entitlements.plist`, and set
it as the cargo runner in `.cargo/config.toml`:

```toml
[target.aarch64-apple-darwin]
runner = "scripts/run-signed.sh"
```

This repository does exactly that, so `cargo run --example ...` and
`cargo test` just work in a checkout.

## Example (Apple silicon)

```rust,no_run
use hypervisor::{ExceptionClass, GuestMemory, MemPerm, Reg, Vcpu, VcpuExit, Vm};

fn main() -> Result<(), hypervisor::Error> {
    let vm = Vm::new()?;

    // One page of guest memory at guest physical address 0x10000 holding
    //     mov x0, #42
    //     hvc #0
    let mut ram = GuestMemory::new(&vm, 0x1_0000, hypervisor::page_size(), MemPerm::RX)?;
    ram.write(0, &0xd280_0540u32.to_le_bytes())?;
    ram.write(4, &0xd400_0002u32.to_le_bytes())?;

    // A vCPU belongs to the thread that creates it (Vcpu is !Send).
    let vcpu = Vcpu::new(&vm)?;
    vcpu.set_reg(Reg::Pc, 0x1_0000)?;
    vcpu.set_reg(Reg::Cpsr, 0x3c5)?; // EL1h, interrupts masked

    match vcpu.run()? {
        VcpuExit::Exception(e) if e.class() == ExceptionClass::Hvc64 => {
            println!("guest says {}", vcpu.reg(Reg::X0)?); // 42
        }
        other => println!("unexpected exit: {other:?}"),
    }
    Ok(())
}
```

Run it from a checkout with:

```shell
$ cargo run --example arm64_hello
exit: Exception(Exception { class: Hvc64, syndrome: 0x5a000000, virtual_address: 0x0, physical_address: 0x0 })
hvc #0 at pc 0x10008
x0 = 42
```

The [runtime tests](tests/arm64.rs) show more: MMIO emulation through data
aborts, SIMD and system registers, IRQ injection, the virtual timer, stopping
a vCPU from another thread, and one vCPU per thread.

## Example (Intel)

[`examples/x86_64_hello.rs`](examples/x86_64_hello.rs) configures the VMCS
for a flat 32-bit guest, runs `mov eax, 0x90901234; hlt` and prints the
registers (`rax = 0x90901234`, `rip = 0x5`); CI runs it on an Intel runner. Note that `map_mem` needs page-aligned memory whose size is a
multiple of 4 KiB; a plain `Vec<u8>` is not page aligned.

## What is covered

Apple silicon:

- [x] Creating and destroying the VM, mapping, unmapping and protecting guest memory
- [x] vCPUs: create (default or with a `VcpuConfig`), run, exit reasons, cancel from any thread
- [x] General purpose, SIMD&FP and system registers
- [x] Exception syndrome decoding (hvc/brk immediates, data aborts, trapped system registers)
- [x] Pending IRQ/FIQ, debug exception and debug register traps
- [x] Virtual timer mask and offset, execution time
- [ ] GIC (macOS 15), SME (macOS 15.2), VM configuration (IPA size, EL2) in the safe API

Intel:

- [x] Accessing x86 registers and model-specific registers (MSRs)
- [x] Mapping guest physical memory
- [x] vCPUs: run, interrupt, TLB invalidation, FP/SIMD state, execution time
- [x] Synchronizing guest timestamp counters (TSC)
- [x] Reading VMX capabilities and VMCS fields

## Minimum supported Rust version

Rust 1.85 (edition 2024). Raising it is a minor version bump.

## Contributing

`cargo test` runs the unit tests and, on an Apple silicon Mac, real guests.
When the hypervisor is not available (unsigned binary, or inside a VM) the
runtime tests skip themselves; set `HYPERVISOR_REQUIRE_VM=1` to turn that
into a failure. Check the Intel code with
`cargo clippy --all-targets --target x86_64-apple-darwin`.

## License

[MIT](LICENSE.md)
