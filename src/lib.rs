/*!
Rust bindings to Apple's [Hypervisor.framework], for hardware-accelerated
virtual machines on macOS.

* **Intel (`x86_64`)**: the original VT-x API (`create_vm`, `vCPU`, `map_mem`,
  VMCS access, the `consts` module). Requires macOS 10.10 or newer and a CPU
  with VT-x, EPT and unrestricted mode (`sysctl kern.hv_support` prints `1`).

On other targets the crate is empty, so it can sit in a cross-platform
dependency tree.

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

#[cfg(target_arch = "x86_64")]
mod x86_64;
#[cfg(target_arch = "x86_64")]
pub use x86_64::*;
