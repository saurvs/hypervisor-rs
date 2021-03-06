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

use libc::*;

pub const PIN_BASED_INTR                     : uint64_t = 1 <<  0;
pub const PIN_BASED_NMI                      : uint64_t = 1 <<  3;
pub const PIN_BASED_VIRTUAL_NMI              : uint64_t = 1 <<  5;
pub const PIN_BASED_PREEMPTION_TIMER         : uint64_t = 1 <<  6;
pub const PIN_BASED_POSTED_INTR              : uint64_t = 1 <<  7;

pub const CPU_BASED_IRQ_WND                  : uint64_t = 1 <<  2;
pub const CPU_BASED_TSC_OFFSET               : uint64_t = 1 <<  3;
pub const CPU_BASED_HLT                      : uint64_t = 1 <<  7;
pub const CPU_BASED_INVLPG                   : uint64_t = 1 <<  9;
pub const CPU_BASED_MWAIT                    : uint64_t = 1 << 10;
pub const CPU_BASED_RDPMC                    : uint64_t = 1 << 11;
pub const CPU_BASED_RDTSC                    : uint64_t = 1 << 12;
pub const CPU_BASED_CR3_LOAD                 : uint64_t = 1 << 15;
pub const CPU_BASED_CR3_STORE                : uint64_t = 1 << 16;
pub const CPU_BASED_CR8_LOAD                 : uint64_t = 1 << 19;
pub const CPU_BASED_CR8_STORE                : uint64_t = 1 << 20;
pub const CPU_BASED_TPR_SHADOW               : uint64_t = 1 << 21;
pub const CPU_BASED_VIRTUAL_NMI_WND          : uint64_t = 1 << 22;
pub const CPU_BASED_MOV_DR                   : uint64_t = 1 << 23;
pub const CPU_BASED_UNCOND_IO                : uint64_t = 1 << 24;
pub const CPU_BASED_IO_BITMAPS               : uint64_t = 1 << 25;
pub const CPU_BASED_MTF                      : uint64_t = 1 << 27;
pub const CPU_BASED_MSR_BITMAPS              : uint64_t = 1 << 28;
pub const CPU_BASED_MONITOR                  : uint64_t = 1 << 29;
pub const CPU_BASED_PAUSE                    : uint64_t = 1 << 30;
pub const CPU_BASED_SECONDARY_CTLS           : uint64_t = 1 << 31;

pub const CPU_BASED2_VIRTUAL_APIC            : uint64_t = 1 <<  0;
pub const CPU_BASED2_EPT                     : uint64_t = 1 <<  1;
pub const CPU_BASED2_DESC_TABLE              : uint64_t = 1 <<  2;
pub const CPU_BASED2_RDTSCP                  : uint64_t = 1 <<  3;
pub const CPU_BASED2_X2APIC                  : uint64_t = 1 <<  4;
pub const CPU_BASED2_VPID                    : uint64_t = 1 <<  5;
pub const CPU_BASED2_WBINVD                  : uint64_t = 1 <<  6;
pub const CPU_BASED2_UNRESTRICTED            : uint64_t = 1 <<  7;
pub const CPU_BASED2_APIC_REG_VIRT           : uint64_t = 1 <<  8;
pub const CPU_BASED2_VIRT_INTR_DELIVERY      : uint64_t = 1 <<  9;
pub const CPU_BASED2_PAUSE_LOOP              : uint64_t = 1 << 10;
pub const CPU_BASED2_RDRAND                  : uint64_t = 1 << 11;
pub const CPU_BASED2_INVPCID                 : uint64_t = 1 << 12;
pub const CPU_BASED2_VMFUNC                  : uint64_t = 1 << 13;
pub const CPU_BASED2_VMCS_SHADOW             : uint64_t = 1 << 14;
pub const CPU_BASED2_RDSEED                  : uint64_t = 1 << 16;
pub const CPU_BASED2_EPT_VE                  : uint64_t = 1 << 18;
pub const CPU_BASED2_XSAVES_XRSTORS          : uint64_t = 1 << 20;

pub const VMX_EPT_VPID_SUPPORT_AD            : uint64_t = 1 << 21;
pub const VMX_EPT_VPID_SUPPORT_EXONLY        : uint64_t = 1 <<  0;

pub const VMEXIT_SAVE_DBG_CONTROLS           : uint64_t = 1 <<  2;
pub const VMEXIT_HOST_IA32E                  : uint64_t = 1 <<  9;
pub const VMEXIT_LOAD_IA32_PERF_GLOBAL_CTRL  : uint64_t = 1 << 12;
pub const VMEXIT_ACK_INTR                    : uint64_t = 1 << 15;
pub const VMEXIT_SAVE_IA32_PAT               : uint64_t = 1 << 18;
pub const VMEXIT_LOAD_IA32_PAT               : uint64_t = 1 << 19;
pub const VMEXIT_SAVE_EFER                   : uint64_t = 1 << 20;
pub const VMEXIT_LOAD_EFER                   : uint64_t = 1 << 21;
pub const VMEXIT_SAVE_VMX_TIMER              : uint64_t = 1 << 22;

pub const VMENTRY_LOAD_DBG_CONTROLS          : uint64_t = 1 <<  2;
pub const VMENTRY_GUEST_IA32E                : uint64_t = 1 <<  9;
pub const VMENTRY_SMM                        : uint64_t = 1 << 10;
pub const VMENTRY_DEACTIVATE_DUAL_MONITOR    : uint64_t = 1 << 11;
pub const VMENTRY_LOAD_IA32_PERF_GLOBAL_CTRL : uint64_t = 1 << 13;
pub const VMENTRY_LOAD_IA32_PAT              : uint64_t = 1 << 14;
pub const VMENTRY_LOAD_EFER                  : uint64_t = 1 << 15;
