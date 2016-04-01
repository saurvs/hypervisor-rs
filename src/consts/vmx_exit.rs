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

use libc::*;

pub const VMX_REASON_EXC_NMI           : uint64_t =  0;
pub const VMX_REASON_IRQ               : uint64_t =  1;
pub const VMX_REASON_TRIPLE_FAULT      : uint64_t =  2;
pub const VMX_REASON_INIT              : uint64_t =  3;
pub const VMX_REASON_SIPI              : uint64_t =  4;
pub const VMX_REASON_IO_SMI            : uint64_t =  5;
pub const VMX_REASON_OTHER_SMI         : uint64_t =  6;
pub const VMX_REASON_IRQ_WND           : uint64_t =  7;
pub const VMX_REASON_VIRTUAL_NMI_WND   : uint64_t =  8;
pub const VMX_REASON_TASK              : uint64_t =  9;
pub const VMX_REASON_CPUID             : uint64_t = 10;
pub const VMX_REASON_GETSEC            : uint64_t = 11;
pub const VMX_REASON_HLT               : uint64_t = 12;
pub const VMX_REASON_INVD              : uint64_t = 13;
pub const VMX_REASON_INVLPG            : uint64_t = 14;
pub const VMX_REASON_RDPMC             : uint64_t = 15;
pub const VMX_REASON_RDTSC             : uint64_t = 16;
pub const VMX_REASON_RSM               : uint64_t = 17;
pub const VMX_REASON_VMCALL            : uint64_t = 18;
pub const VMX_REASON_VMCLEAR           : uint64_t = 19;
pub const VMX_REASON_VMLAUNCH          : uint64_t = 20;
pub const VMX_REASON_VMPTRLD           : uint64_t = 21;
pub const VMX_REASON_VMPTRST           : uint64_t = 22;
pub const VMX_REASON_VMREAD            : uint64_t = 23;
pub const VMX_REASON_VMRESUME          : uint64_t = 24;
pub const VMX_REASON_VMWRITE           : uint64_t = 25;
pub const VMX_REASON_VMOFF             : uint64_t = 26;
pub const VMX_REASON_VMON              : uint64_t = 27;
pub const VMX_REASON_MOV_CR            : uint64_t = 28;
pub const VMX_REASON_MOV_DR            : uint64_t = 29;
pub const VMX_REASON_IO                : uint64_t = 30;
pub const VMX_REASON_RDMSR             : uint64_t = 31;
pub const VMX_REASON_WRMSR             : uint64_t = 32;
pub const VMX_REASON_VMENTRY_GUEST     : uint64_t = 33;
pub const VMX_REASON_VMENTRY_MSR       : uint64_t = 34;
pub const VMX_REASON_MWAIT             : uint64_t = 36;
pub const VMX_REASON_MTF               : uint64_t = 37;
pub const VMX_REASON_MONITOR           : uint64_t = 39;
pub const VMX_REASON_PAUSE             : uint64_t = 40;
pub const VMX_REASON_VMENTRY_MC        : uint64_t = 41;
pub const VMX_REASON_TPR_THRESHOLD     : uint64_t = 43;
pub const VMX_REASON_APIC_ACCESS       : uint64_t = 44;
pub const VMX_REASON_VIRTUALIZED_EOI   : uint64_t = 45;
pub const VMX_REASON_GDTR_IDTR         : uint64_t = 46;
pub const VMX_REASON_LDTR_TR           : uint64_t = 47;
pub const VMX_REASON_EPT_VIOLATION     : uint64_t = 48;
pub const VMX_REASON_EPT_MISCONFIG     : uint64_t = 49;
pub const VMX_REASON_EPT_INVEPT        : uint64_t = 50;
pub const VMX_REASON_RDTSCP            : uint64_t = 51;
pub const VMX_REASON_VMX_TIMER_EXPIRED : uint64_t = 52;
pub const VMX_REASON_INVVPID           : uint64_t = 53;
pub const VMX_REASON_WBINVD            : uint64_t = 54;
pub const VMX_REASON_XSETBV            : uint64_t = 55;
pub const VMX_REASON_APIC_WRITE        : uint64_t = 56;
pub const VMX_REASON_RDRAND            : uint64_t = 57;
pub const VMX_REASON_INVPCID           : uint64_t = 58;
pub const VMX_REASON_VMFUNC            : uint64_t = 59;
pub const VMX_REASON_RDSEED            : uint64_t = 61;
pub const VMX_REASON_XSAVES            : uint64_t = 63;
pub const VMX_REASON_XRSTORS           : uint64_t = 64;
