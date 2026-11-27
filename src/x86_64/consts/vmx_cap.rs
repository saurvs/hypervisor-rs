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

//! VMX capability field values

pub const PIN_BASED_INTR: u64 = 1 << 0;
pub const PIN_BASED_NMI: u64 = 1 << 3;
pub const PIN_BASED_VIRTUAL_NMI: u64 = 1 << 5;
pub const PIN_BASED_PREEMPTION_TIMER: u64 = 1 << 6;
pub const PIN_BASED_POSTED_INTR: u64 = 1 << 7;

pub const CPU_BASED_IRQ_WND: u64 = 1 << 2;
pub const CPU_BASED_TSC_OFFSET: u64 = 1 << 3;
pub const CPU_BASED_HLT: u64 = 1 << 7;
pub const CPU_BASED_INVLPG: u64 = 1 << 9;
pub const CPU_BASED_MWAIT: u64 = 1 << 10;
pub const CPU_BASED_RDPMC: u64 = 1 << 11;
pub const CPU_BASED_RDTSC: u64 = 1 << 12;
pub const CPU_BASED_CR3_LOAD: u64 = 1 << 15;
pub const CPU_BASED_CR3_STORE: u64 = 1 << 16;
pub const CPU_BASED_CR8_LOAD: u64 = 1 << 19;
pub const CPU_BASED_CR8_STORE: u64 = 1 << 20;
pub const CPU_BASED_TPR_SHADOW: u64 = 1 << 21;
pub const CPU_BASED_VIRTUAL_NMI_WND: u64 = 1 << 22;
pub const CPU_BASED_MOV_DR: u64 = 1 << 23;
pub const CPU_BASED_UNCOND_IO: u64 = 1 << 24;
pub const CPU_BASED_IO_BITMAPS: u64 = 1 << 25;
pub const CPU_BASED_MTF: u64 = 1 << 27;
pub const CPU_BASED_MSR_BITMAPS: u64 = 1 << 28;
pub const CPU_BASED_MONITOR: u64 = 1 << 29;
pub const CPU_BASED_PAUSE: u64 = 1 << 30;
pub const CPU_BASED_SECONDARY_CTLS: u64 = 1 << 31;

pub const CPU_BASED2_VIRTUAL_APIC: u64 = 1 << 0;
pub const CPU_BASED2_EPT: u64 = 1 << 1;
pub const CPU_BASED2_DESC_TABLE: u64 = 1 << 2;
pub const CPU_BASED2_RDTSCP: u64 = 1 << 3;
pub const CPU_BASED2_X2APIC: u64 = 1 << 4;
pub const CPU_BASED2_VPID: u64 = 1 << 5;
pub const CPU_BASED2_WBINVD: u64 = 1 << 6;
pub const CPU_BASED2_UNRESTRICTED: u64 = 1 << 7;
pub const CPU_BASED2_APIC_REG_VIRT: u64 = 1 << 8;
pub const CPU_BASED2_VIRT_INTR_DELIVERY: u64 = 1 << 9;
pub const CPU_BASED2_PAUSE_LOOP: u64 = 1 << 10;
pub const CPU_BASED2_RDRAND: u64 = 1 << 11;
pub const CPU_BASED2_INVPCID: u64 = 1 << 12;
pub const CPU_BASED2_VMFUNC: u64 = 1 << 13;
pub const CPU_BASED2_VMCS_SHADOW: u64 = 1 << 14;
pub const CPU_BASED2_RDSEED: u64 = 1 << 16;
pub const CPU_BASED2_EPT_VE: u64 = 1 << 18;
pub const CPU_BASED2_XSAVES_XRSTORS: u64 = 1 << 20;

pub const VMX_EPT_VPID_SUPPORT_AD: u64 = 1 << 21;
pub const VMX_EPT_VPID_SUPPORT_EXONLY: u64 = 1 << 0;

pub const VMEXIT_SAVE_DBG_CONTROLS: u64 = 1 << 2;
pub const VMEXIT_HOST_IA32E: u64 = 1 << 9;
pub const VMEXIT_LOAD_IA32_PERF_GLOBAL_CTRL: u64 = 1 << 12;
pub const VMEXIT_ACK_INTR: u64 = 1 << 15;
pub const VMEXIT_SAVE_IA32_PAT: u64 = 1 << 18;
pub const VMEXIT_LOAD_IA32_PAT: u64 = 1 << 19;
pub const VMEXIT_SAVE_EFER: u64 = 1 << 20;
pub const VMEXIT_LOAD_EFER: u64 = 1 << 21;
pub const VMEXIT_SAVE_VMX_TIMER: u64 = 1 << 22;

pub const VMENTRY_LOAD_DBG_CONTROLS: u64 = 1 << 2;
pub const VMENTRY_GUEST_IA32E: u64 = 1 << 9;
pub const VMENTRY_SMM: u64 = 1 << 10;
pub const VMENTRY_DEACTIVATE_DUAL_MONITOR: u64 = 1 << 11;
pub const VMENTRY_LOAD_IA32_PERF_GLOBAL_CTRL: u64 = 1 << 13;
pub const VMENTRY_LOAD_IA32_PAT: u64 = 1 << 14;
pub const VMENTRY_LOAD_EFER: u64 = 1 << 15;
