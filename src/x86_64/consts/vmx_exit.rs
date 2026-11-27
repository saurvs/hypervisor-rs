/*
Copyright (c) 2016 Saurav Sachidanand

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
*/

//! VMX exit reasons

pub const VMX_REASON_EXC_NMI: u64 = 0;
pub const VMX_REASON_IRQ: u64 = 1;
pub const VMX_REASON_TRIPLE_FAULT: u64 = 2;
pub const VMX_REASON_INIT: u64 = 3;
pub const VMX_REASON_SIPI: u64 = 4;
pub const VMX_REASON_IO_SMI: u64 = 5;
pub const VMX_REASON_OTHER_SMI: u64 = 6;
pub const VMX_REASON_IRQ_WND: u64 = 7;
pub const VMX_REASON_VIRTUAL_NMI_WND: u64 = 8;
pub const VMX_REASON_TASK: u64 = 9;
pub const VMX_REASON_CPUID: u64 = 10;
pub const VMX_REASON_GETSEC: u64 = 11;
pub const VMX_REASON_HLT: u64 = 12;
pub const VMX_REASON_INVD: u64 = 13;
pub const VMX_REASON_INVLPG: u64 = 14;
pub const VMX_REASON_RDPMC: u64 = 15;
pub const VMX_REASON_RDTSC: u64 = 16;
pub const VMX_REASON_RSM: u64 = 17;
pub const VMX_REASON_VMCALL: u64 = 18;
pub const VMX_REASON_VMCLEAR: u64 = 19;
pub const VMX_REASON_VMLAUNCH: u64 = 20;
pub const VMX_REASON_VMPTRLD: u64 = 21;
pub const VMX_REASON_VMPTRST: u64 = 22;
pub const VMX_REASON_VMREAD: u64 = 23;
pub const VMX_REASON_VMRESUME: u64 = 24;
pub const VMX_REASON_VMWRITE: u64 = 25;
pub const VMX_REASON_VMOFF: u64 = 26;
pub const VMX_REASON_VMON: u64 = 27;
pub const VMX_REASON_MOV_CR: u64 = 28;
pub const VMX_REASON_MOV_DR: u64 = 29;
pub const VMX_REASON_IO: u64 = 30;
pub const VMX_REASON_RDMSR: u64 = 31;
pub const VMX_REASON_WRMSR: u64 = 32;
pub const VMX_REASON_VMENTRY_GUEST: u64 = 33;
pub const VMX_REASON_VMENTRY_MSR: u64 = 34;
pub const VMX_REASON_MWAIT: u64 = 36;
pub const VMX_REASON_MTF: u64 = 37;
pub const VMX_REASON_MONITOR: u64 = 39;
pub const VMX_REASON_PAUSE: u64 = 40;
pub const VMX_REASON_VMENTRY_MC: u64 = 41;
pub const VMX_REASON_TPR_THRESHOLD: u64 = 43;
pub const VMX_REASON_APIC_ACCESS: u64 = 44;
pub const VMX_REASON_VIRTUALIZED_EOI: u64 = 45;
pub const VMX_REASON_GDTR_IDTR: u64 = 46;
pub const VMX_REASON_LDTR_TR: u64 = 47;
pub const VMX_REASON_EPT_VIOLATION: u64 = 48;
pub const VMX_REASON_EPT_MISCONFIG: u64 = 49;
pub const VMX_REASON_EPT_INVEPT: u64 = 50;
pub const VMX_REASON_RDTSCP: u64 = 51;
pub const VMX_REASON_VMX_TIMER_EXPIRED: u64 = 52;
pub const VMX_REASON_INVVPID: u64 = 53;
pub const VMX_REASON_WBINVD: u64 = 54;
pub const VMX_REASON_XSETBV: u64 = 55;
pub const VMX_REASON_APIC_WRITE: u64 = 56;
pub const VMX_REASON_RDRAND: u64 = 57;
pub const VMX_REASON_INVPCID: u64 = 58;
pub const VMX_REASON_VMFUNC: u64 = 59;
pub const VMX_REASON_RDSEED: u64 = 61;
pub const VMX_REASON_XSAVES: u64 = 63;
pub const VMX_REASON_XRSTORS: u64 = 64;
