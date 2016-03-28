# hypervisor-rs
[![](http://meritbadge.herokuapp.com/hypervisor)]
(https://crates.io/crates/hypervisor)
[![](https://img.shields.io/badge/license-MIT-blue.svg)]
(https://github.com/saurvs/hypervisor-rs/blob/master/LICENSE.md)

`hypervisor` is a Rust library that enables hardware-accelerated execution of
virtual machines on OS X.

It binds to the `Hypervisor` framework on OS X, and exposes a safe Rust
interface through the `hypervisor` module, and an unsafe foreign function
interface through the `hypervisor::ffi` module.

[Documentation](https://saurvs.github.io/hypervisor-rs/)

## Prerequisites

To use this library, you need

* OS X Yosemite (10.10), or newer

* an Intel processor with the VT-x feature set that includes Extended Page
Tables (EPT) and the Unrestricted Mode. To verify this, run and expect the
following in your Terminal:
  ```shell
  $ sysctl kern.hv_support
  kern.hv_support: 1
  ```

## Status
- [x] Accessing x86 registers
- [x] Accessing model-specific registers (MSRs)
- [ ] Mapping segments of guest physical memory into guest physical address space of the VM
- [x] Virtual CPUs
  - [x] Executing and interrupting
  - [x] Force flushing of cached state
  - [x] Invalidating translation lookaside buffer (TLB)
  - [x] Obtaining cumulative execution time
  - [x] Synchronizing guest timestamp-counters (TSC)
- [x] Accessing Virtual Machine Control Structures (VMCS)
- [ ] Accessing Floating Point (FP) state

## Usage

* Add the dependency ```hypervisor``` in your ```Cargo.toml```
  ```toml
  [dependencies]
  hypervisor = "0.0.1"
  ```

* Include the crate ```hypervisor``` in your code
  ```rust
  extern crate hypervisor;

  use hypervisor::osx::*;
  ```

* Create a virtual machine that executes for a finite time
(doing nothing useful)
  ```rust
  // create a VM
  create_vm();

  // create a virtual CPU
  let vcpu = vCPU::new().unwrap();

  // run it
  vcpu.run();

  // print time elapsed in nanoseconds
  println!("vcpu execution time: {:?}ns", vcpu.exec_time().unwrap());

  // destroy the virtual CPU
  vcpu.destory();

  // destroy the VM
  destory_vm();
  ```
