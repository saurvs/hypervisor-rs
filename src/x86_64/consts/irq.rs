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

//! Interrupt Request (IRQ) Codes

pub const IRQ_INFO_EXT_IRQ: u32 = 0 << 8;
pub const IRQ_INFO_NMI: u32 = 2 << 8;
pub const IRQ_INFO_HARD_EXC: u32 = 3 << 8;
pub const IRQ_INFO_SOFT_IRQ: u32 = 4 << 8;
pub const IRQ_INFO_PRIV_SOFT_EXC: u32 = 5 << 8;
pub const IRQ_INFO_SOFT_EXC: u32 = 6 << 8;
pub const IRQ_INFO_ERROR_VALID: u32 = 1 << 11;
pub const IRQ_INFO_VALID: u32 = 1 << 31;
