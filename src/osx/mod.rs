/*!
Hypervisor API for OSX

To use this library, you need

* OS X Yosemite (10.10) or newer

* an Intel processor with the VT-x feature set that includes Extended Page
Tables (EPT) and Unrestricted Mode. To verify this, run and expect the following
in your Terminal:

  ```shell
  $ sysctl kern.hv_support
  kern.hv_support: 1
  ```
!*/
//extern crate core;

use super::core::fmt;
use libc::*;

#[allow(non_camel_case_types)]
pub mod ffi;

use self::ffi::*;
use self::ffi::consts::*;

/// Error returned after every call
pub enum Error {
    /// Success
    Success,
    /// Error
    Error,
    /// Busy
    Busy,
    /// Bad argument
    BadArg,
    /// No resources
    NoRes,
    /// No device
    NoDev,
    /// Unsupported
    Unsupp
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Success => write!(f, "Success"),
            Error::Error => write!(f, "Error"),
            Error::Busy => write!(f, "Busy"),
            Error::BadArg => write!(f, "Bad argument"),
            Error::NoRes => write!(f, "No resources"),
            Error::NoDev => write!(f, "No device"),
            Error::Unsupp => write!(f, "Unsupported"),
        }
    }
}

fn match_error_code(code: hv_return_t) -> Error {
    match code {
        HV_SUCCESS => Error::Success,
        HV_BUSY => Error::Busy,
        HV_BAD_ARGUMENT => Error::BadArg,
        HV_NO_RESOURCES => Error::NoRes,
        HV_NO_DEVICE => Error::NoDev,
        HV_UNSUPPORTED => Error::Unsupp,
        _ => Error::Error
    }
}

/// Creates a VM instance for the current task
pub fn create_vm() -> Error {
    match_error_code(unsafe {
        hv_vm_create(0)
    })
}

/// Destroys the VM instance associated with the current task
pub fn destory_vm() -> Error {
    match_error_code(unsafe {
        hv_vm_destroy()
    })
}

/// Virtual CPU
#[allow(non_camel_case_types)]
pub struct vCPU {
    /// Virtual CPU ID
    pub id: u32
}

impl vCPU {
    /// Creates a vCPU instance for the current thread
    pub fn new() -> Result<vCPU, Error> {
        let mut vcpuid: hv_vcpuid_t = 0;

        let error = match_error_code(unsafe {
            hv_vcpu_create(&mut vcpuid, 0)
        });

        match error {
            Error::Success => Ok(vCPU {
                id: vcpuid as u32
            }),
            _ => Err(error)
        }
    }

    /// Destroys the vCPU instance associated with the current thread
    pub fn destory(&self) -> Error {
        match_error_code(unsafe {
            hv_vcpu_destroy(self.id as hv_vcpuid_t)
        })
    }

    /// Executes the vCPU
    pub fn run(&self) -> Error {
        match_error_code(unsafe {
            hv_vcpu_run(self.id as hv_vcpuid_t)
        })
    }

    /// Returns the cumulative execution time of the vCPU in nanoseconds
    pub fn exec_time(&self) -> Result<u64, Error> {
        let mut exec_time: uint64_t = 0;

        let error = match_error_code(unsafe {
            hv_vcpu_get_exec_time(self.id, &mut exec_time)
        });

        match error {
            Error::Success => Ok(exec_time as u64),
            _ => Err(error)
        }
    }

}

impl fmt::Debug for vCPU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "vCPU ID: {}", (*self).id)
    }
}
