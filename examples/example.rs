extern crate hypervisor;

use hypervisor::*;

fn main() {
    // create a VM
    create_vm();

    // create a virtual CPU
    let vcpu = vCPU::new().unwrap();

    // run it
    vcpu.run();

    // print time elapsed in nanoseconds
    println!("vcpu execution time: {:?}ns", vcpu.exec_time().unwrap());

    // destroy the VM
    vcpu.destory();

    // destroy the VM
    destory_vm();
}
