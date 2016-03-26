# hypervisor-rs [![](http://meritbadge.herokuapp.com/hypervisor)](https://crates.io/crates/hypervisor) [![](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/saurvs/hypervisor-rs/blob/master/LICENSE.md)

`hypervisor` is a Rust library that enables hardware-accelerated execution of
virtual machines using OS X as a guest.

It binds to the `Hypervisor` framework available on OS X Yosemite or newer,
and exposes a safe Rust interface through the `hypervisor::osx` module, and an
unsafe foreign function interface through the `hypervisor::osx::ffi` module.

[Documentation](https://saurvs.github.io/hypervisor-rs/)

## Prerequisites

To use this library, you need

* OS X Yosemite (10.10) or newer

* an Intel processor with the VT-x feature set that includes Extended Page
Tables (EPT) and Unrestricted Mode. To verify this, run and expect the following
in your Terminal:
  ```shell
  $ sysctl kern.hv_support
  kern.hv_support: 1
  ```

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

* Create a virtual machine that executes for a finite time (doing nothing useful)
  ```rust
  // create a VM
  create_vm();

  // create a virtual CPU
  let vcpu = vCPU::new().unwrap();

  // run it
  vcpu.run();

  // print time elapsed in nanoseconds
  println!("vcpu execution time: {:?}ns", vcpu.exec_time().unwrap());
  // should print some random finite value every time

  // destroy the virtual CPU
  vcpu.destory();

  // destroy the VM
  destory_vm();
  ```

## Status

*A lot is left to be done.*
