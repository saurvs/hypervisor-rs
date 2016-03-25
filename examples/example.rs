extern crate hypervisor;

use hypervisor::osx::*;
use hypervisor::osx::ffi::*;

fn main() {
    // create a VM
    create_vm();

    // create a virtual CPU
    let vcpu = vCPU::new().unwrap();

    // run it
    vcpu.run();

    // print time elapsed
    println!("vcpu execution time: {:?}ns", vcpu.exec_time().unwrap());
    // should print some finite value

    // destroy the VM
    vcpu.destory();

    // destroy the VM
    destory_vm();
}
