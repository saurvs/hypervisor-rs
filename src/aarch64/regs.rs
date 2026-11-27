//! Register identifiers for the arm64 API.

use super::ffi;

/// A general purpose register, the program counter, or a floating point
/// control/status register (`hv_reg_t`).
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Reg {
    /// General purpose register `X0`.
    X0 = 0,
    /// General purpose register `X1`.
    X1 = 1,
    /// General purpose register `X2`.
    X2 = 2,
    /// General purpose register `X3`.
    X3 = 3,
    /// General purpose register `X4`.
    X4 = 4,
    /// General purpose register `X5`.
    X5 = 5,
    /// General purpose register `X6`.
    X6 = 6,
    /// General purpose register `X7`.
    X7 = 7,
    /// General purpose register `X8`.
    X8 = 8,
    /// General purpose register `X9`.
    X9 = 9,
    /// General purpose register `X10`.
    X10 = 10,
    /// General purpose register `X11`.
    X11 = 11,
    /// General purpose register `X12`.
    X12 = 12,
    /// General purpose register `X13`.
    X13 = 13,
    /// General purpose register `X14`.
    X14 = 14,
    /// General purpose register `X15`.
    X15 = 15,
    /// General purpose register `X16`.
    X16 = 16,
    /// General purpose register `X17`.
    X17 = 17,
    /// General purpose register `X18`.
    X18 = 18,
    /// General purpose register `X19`.
    X19 = 19,
    /// General purpose register `X20`.
    X20 = 20,
    /// General purpose register `X21`.
    X21 = 21,
    /// General purpose register `X22`.
    X22 = 22,
    /// General purpose register `X23`.
    X23 = 23,
    /// General purpose register `X24`.
    X24 = 24,
    /// General purpose register `X25`.
    X25 = 25,
    /// General purpose register `X26`.
    X26 = 26,
    /// General purpose register `X27`.
    X27 = 27,
    /// General purpose register `X28`.
    X28 = 28,
    /// General purpose register `X29`.
    X29 = 29,
    /// General purpose register `X30`.
    X30 = 30,
    /// Program counter.
    Pc = 31,
    /// Floating-point Control Register.
    Fpcr = 32,
    /// Floating-point Status Register.
    Fpsr = 33,
    /// Current Program Status Register (`PSTATE`).
    Cpsr = 34,
}

impl Reg {
    /// The frame pointer, an alias of [`Reg::X29`].
    pub const FP: Self = Self::X29;
    /// The link register, an alias of [`Reg::X30`].
    pub const LR: Self = Self::X30;

    const XS: [Reg; 31] = [
        Reg::X0,
        Reg::X1,
        Reg::X2,
        Reg::X3,
        Reg::X4,
        Reg::X5,
        Reg::X6,
        Reg::X7,
        Reg::X8,
        Reg::X9,
        Reg::X10,
        Reg::X11,
        Reg::X12,
        Reg::X13,
        Reg::X14,
        Reg::X15,
        Reg::X16,
        Reg::X17,
        Reg::X18,
        Reg::X19,
        Reg::X20,
        Reg::X21,
        Reg::X22,
        Reg::X23,
        Reg::X24,
        Reg::X25,
        Reg::X26,
        Reg::X27,
        Reg::X28,
        Reg::X29,
        Reg::X30,
    ];

    /// Returns general purpose register `Xn` for `n` in `0..=30`.
    ///
    /// This is handy for decoding the register number in an exception
    /// syndrome. Returns `None` for 31 (which encodes `XZR` or `SP`,
    /// depending on the instruction) and above.
    pub const fn x(n: u8) -> Option<Self> {
        if (n as usize) < Self::XS.len() {
            Some(Self::XS[n as usize])
        } else {
            None
        }
    }
}

impl From<Reg> for ffi::hv_reg_t {
    fn from(reg: Reg) -> Self {
        reg as Self
    }
}

/// A 128-bit SIMD&FP register (`hv_simd_fp_reg_t`).
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SimdFpReg {
    /// SIMD&FP register `Q0`.
    Q0 = 0,
    /// SIMD&FP register `Q1`.
    Q1 = 1,
    /// SIMD&FP register `Q2`.
    Q2 = 2,
    /// SIMD&FP register `Q3`.
    Q3 = 3,
    /// SIMD&FP register `Q4`.
    Q4 = 4,
    /// SIMD&FP register `Q5`.
    Q5 = 5,
    /// SIMD&FP register `Q6`.
    Q6 = 6,
    /// SIMD&FP register `Q7`.
    Q7 = 7,
    /// SIMD&FP register `Q8`.
    Q8 = 8,
    /// SIMD&FP register `Q9`.
    Q9 = 9,
    /// SIMD&FP register `Q10`.
    Q10 = 10,
    /// SIMD&FP register `Q11`.
    Q11 = 11,
    /// SIMD&FP register `Q12`.
    Q12 = 12,
    /// SIMD&FP register `Q13`.
    Q13 = 13,
    /// SIMD&FP register `Q14`.
    Q14 = 14,
    /// SIMD&FP register `Q15`.
    Q15 = 15,
    /// SIMD&FP register `Q16`.
    Q16 = 16,
    /// SIMD&FP register `Q17`.
    Q17 = 17,
    /// SIMD&FP register `Q18`.
    Q18 = 18,
    /// SIMD&FP register `Q19`.
    Q19 = 19,
    /// SIMD&FP register `Q20`.
    Q20 = 20,
    /// SIMD&FP register `Q21`.
    Q21 = 21,
    /// SIMD&FP register `Q22`.
    Q22 = 22,
    /// SIMD&FP register `Q23`.
    Q23 = 23,
    /// SIMD&FP register `Q24`.
    Q24 = 24,
    /// SIMD&FP register `Q25`.
    Q25 = 25,
    /// SIMD&FP register `Q26`.
    Q26 = 26,
    /// SIMD&FP register `Q27`.
    Q27 = 27,
    /// SIMD&FP register `Q28`.
    Q28 = 28,
    /// SIMD&FP register `Q29`.
    Q29 = 29,
    /// SIMD&FP register `Q30`.
    Q30 = 30,
    /// SIMD&FP register `Q31`.
    Q31 = 31,
}

impl SimdFpReg {
    const QS: [SimdFpReg; 32] = [
        SimdFpReg::Q0,
        SimdFpReg::Q1,
        SimdFpReg::Q2,
        SimdFpReg::Q3,
        SimdFpReg::Q4,
        SimdFpReg::Q5,
        SimdFpReg::Q6,
        SimdFpReg::Q7,
        SimdFpReg::Q8,
        SimdFpReg::Q9,
        SimdFpReg::Q10,
        SimdFpReg::Q11,
        SimdFpReg::Q12,
        SimdFpReg::Q13,
        SimdFpReg::Q14,
        SimdFpReg::Q15,
        SimdFpReg::Q16,
        SimdFpReg::Q17,
        SimdFpReg::Q18,
        SimdFpReg::Q19,
        SimdFpReg::Q20,
        SimdFpReg::Q21,
        SimdFpReg::Q22,
        SimdFpReg::Q23,
        SimdFpReg::Q24,
        SimdFpReg::Q25,
        SimdFpReg::Q26,
        SimdFpReg::Q27,
        SimdFpReg::Q28,
        SimdFpReg::Q29,
        SimdFpReg::Q30,
        SimdFpReg::Q31,
    ];

    /// Returns SIMD&FP register `Qn` for `n` in `0..=31`.
    pub const fn q(n: u8) -> Option<Self> {
        if (n as usize) < Self::QS.len() {
            Some(Self::QS[n as usize])
        } else {
            None
        }
    }
}

impl From<SimdFpReg> for ffi::hv_simd_fp_reg_t {
    fn from(reg: SimdFpReg) -> Self {
        reg as Self
    }
}

/// A system register (`hv_sys_reg_t`).
///
/// The value is the register's `op0:op1:CRn:CRm:op2` encoding, as used by the
/// framework. All registers listed in the macOS SDK are provided as associated
/// constants. Registers marked with a macOS version are only accepted by that
/// release and later; using them earlier fails with an error, not undefined
/// behaviour.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SysReg(pub ffi::hv_sys_reg_t);

impl SysReg {
    /// `DBGBVR0_EL1`.
    pub const DBGBVR0_EL1: Self = Self(0x8004);
    /// `DBGBCR0_EL1`.
    pub const DBGBCR0_EL1: Self = Self(0x8005);
    /// `DBGWVR0_EL1`.
    pub const DBGWVR0_EL1: Self = Self(0x8006);
    /// `DBGWCR0_EL1`.
    pub const DBGWCR0_EL1: Self = Self(0x8007);
    /// `DBGBVR1_EL1`.
    pub const DBGBVR1_EL1: Self = Self(0x800c);
    /// `DBGBCR1_EL1`.
    pub const DBGBCR1_EL1: Self = Self(0x800d);
    /// `DBGWVR1_EL1`.
    pub const DBGWVR1_EL1: Self = Self(0x800e);
    /// `DBGWCR1_EL1`.
    pub const DBGWCR1_EL1: Self = Self(0x800f);
    /// `MDCCINT_EL1`.
    pub const MDCCINT_EL1: Self = Self(0x8010);
    /// `MDSCR_EL1`.
    pub const MDSCR_EL1: Self = Self(0x8012);
    /// `DBGBVR2_EL1`.
    pub const DBGBVR2_EL1: Self = Self(0x8014);
    /// `DBGBCR2_EL1`.
    pub const DBGBCR2_EL1: Self = Self(0x8015);
    /// `DBGWVR2_EL1`.
    pub const DBGWVR2_EL1: Self = Self(0x8016);
    /// `DBGWCR2_EL1`.
    pub const DBGWCR2_EL1: Self = Self(0x8017);
    /// `DBGBVR3_EL1`.
    pub const DBGBVR3_EL1: Self = Self(0x801c);
    /// `DBGBCR3_EL1`.
    pub const DBGBCR3_EL1: Self = Self(0x801d);
    /// `DBGWVR3_EL1`.
    pub const DBGWVR3_EL1: Self = Self(0x801e);
    /// `DBGWCR3_EL1`.
    pub const DBGWCR3_EL1: Self = Self(0x801f);
    /// `DBGBVR4_EL1`.
    pub const DBGBVR4_EL1: Self = Self(0x8024);
    /// `DBGBCR4_EL1`.
    pub const DBGBCR4_EL1: Self = Self(0x8025);
    /// `DBGWVR4_EL1`.
    pub const DBGWVR4_EL1: Self = Self(0x8026);
    /// `DBGWCR4_EL1`.
    pub const DBGWCR4_EL1: Self = Self(0x8027);
    /// `DBGBVR5_EL1`.
    pub const DBGBVR5_EL1: Self = Self(0x802c);
    /// `DBGBCR5_EL1`.
    pub const DBGBCR5_EL1: Self = Self(0x802d);
    /// `DBGWVR5_EL1`.
    pub const DBGWVR5_EL1: Self = Self(0x802e);
    /// `DBGWCR5_EL1`.
    pub const DBGWCR5_EL1: Self = Self(0x802f);
    /// `DBGBVR6_EL1`.
    pub const DBGBVR6_EL1: Self = Self(0x8034);
    /// `DBGBCR6_EL1`.
    pub const DBGBCR6_EL1: Self = Self(0x8035);
    /// `DBGWVR6_EL1`.
    pub const DBGWVR6_EL1: Self = Self(0x8036);
    /// `DBGWCR6_EL1`.
    pub const DBGWCR6_EL1: Self = Self(0x8037);
    /// `DBGBVR7_EL1`.
    pub const DBGBVR7_EL1: Self = Self(0x803c);
    /// `DBGBCR7_EL1`.
    pub const DBGBCR7_EL1: Self = Self(0x803d);
    /// `DBGWVR7_EL1`.
    pub const DBGWVR7_EL1: Self = Self(0x803e);
    /// `DBGWCR7_EL1`.
    pub const DBGWCR7_EL1: Self = Self(0x803f);
    /// `DBGBVR8_EL1`.
    pub const DBGBVR8_EL1: Self = Self(0x8044);
    /// `DBGBCR8_EL1`.
    pub const DBGBCR8_EL1: Self = Self(0x8045);
    /// `DBGWVR8_EL1`.
    pub const DBGWVR8_EL1: Self = Self(0x8046);
    /// `DBGWCR8_EL1`.
    pub const DBGWCR8_EL1: Self = Self(0x8047);
    /// `DBGBVR9_EL1`.
    pub const DBGBVR9_EL1: Self = Self(0x804c);
    /// `DBGBCR9_EL1`.
    pub const DBGBCR9_EL1: Self = Self(0x804d);
    /// `DBGWVR9_EL1`.
    pub const DBGWVR9_EL1: Self = Self(0x804e);
    /// `DBGWCR9_EL1`.
    pub const DBGWCR9_EL1: Self = Self(0x804f);
    /// `DBGBVR10_EL1`.
    pub const DBGBVR10_EL1: Self = Self(0x8054);
    /// `DBGBCR10_EL1`.
    pub const DBGBCR10_EL1: Self = Self(0x8055);
    /// `DBGWVR10_EL1`.
    pub const DBGWVR10_EL1: Self = Self(0x8056);
    /// `DBGWCR10_EL1`.
    pub const DBGWCR10_EL1: Self = Self(0x8057);
    /// `DBGBVR11_EL1`.
    pub const DBGBVR11_EL1: Self = Self(0x805c);
    /// `DBGBCR11_EL1`.
    pub const DBGBCR11_EL1: Self = Self(0x805d);
    /// `DBGWVR11_EL1`.
    pub const DBGWVR11_EL1: Self = Self(0x805e);
    /// `DBGWCR11_EL1`.
    pub const DBGWCR11_EL1: Self = Self(0x805f);
    /// `DBGBVR12_EL1`.
    pub const DBGBVR12_EL1: Self = Self(0x8064);
    /// `DBGBCR12_EL1`.
    pub const DBGBCR12_EL1: Self = Self(0x8065);
    /// `DBGWVR12_EL1`.
    pub const DBGWVR12_EL1: Self = Self(0x8066);
    /// `DBGWCR12_EL1`.
    pub const DBGWCR12_EL1: Self = Self(0x8067);
    /// `DBGBVR13_EL1`.
    pub const DBGBVR13_EL1: Self = Self(0x806c);
    /// `DBGBCR13_EL1`.
    pub const DBGBCR13_EL1: Self = Self(0x806d);
    /// `DBGWVR13_EL1`.
    pub const DBGWVR13_EL1: Self = Self(0x806e);
    /// `DBGWCR13_EL1`.
    pub const DBGWCR13_EL1: Self = Self(0x806f);
    /// `DBGBVR14_EL1`.
    pub const DBGBVR14_EL1: Self = Self(0x8074);
    /// `DBGBCR14_EL1`.
    pub const DBGBCR14_EL1: Self = Self(0x8075);
    /// `DBGWVR14_EL1`.
    pub const DBGWVR14_EL1: Self = Self(0x8076);
    /// `DBGWCR14_EL1`.
    pub const DBGWCR14_EL1: Self = Self(0x8077);
    /// `DBGBVR15_EL1`.
    pub const DBGBVR15_EL1: Self = Self(0x807c);
    /// `DBGBCR15_EL1`.
    pub const DBGBCR15_EL1: Self = Self(0x807d);
    /// `DBGWVR15_EL1`.
    pub const DBGWVR15_EL1: Self = Self(0x807e);
    /// `DBGWCR15_EL1`.
    pub const DBGWCR15_EL1: Self = Self(0x807f);
    /// `MIDR_EL1`.
    pub const MIDR_EL1: Self = Self(0xc000);
    /// `MPIDR_EL1`.
    pub const MPIDR_EL1: Self = Self(0xc005);
    /// `ID_AA64PFR0_EL1`.
    pub const ID_AA64PFR0_EL1: Self = Self(0xc020);
    /// `ID_AA64PFR1_EL1`.
    pub const ID_AA64PFR1_EL1: Self = Self(0xc021);
    /// `ID_AA64PFR2_EL1` (macOS 27.0+).
    pub const ID_AA64PFR2_EL1: Self = Self(0xc022);
    /// `ID_AA64ZFR0_EL1` (macOS 15.2+).
    pub const ID_AA64ZFR0_EL1: Self = Self(0xc024);
    /// `ID_AA64SMFR0_EL1` (macOS 15.2+).
    pub const ID_AA64SMFR0_EL1: Self = Self(0xc025);
    /// `ID_AA64DFR0_EL1`.
    pub const ID_AA64DFR0_EL1: Self = Self(0xc028);
    /// `ID_AA64DFR1_EL1`.
    pub const ID_AA64DFR1_EL1: Self = Self(0xc029);
    /// `ID_AA64ISAR0_EL1`.
    pub const ID_AA64ISAR0_EL1: Self = Self(0xc030);
    /// `ID_AA64ISAR1_EL1`.
    pub const ID_AA64ISAR1_EL1: Self = Self(0xc031);
    /// `ID_AA64ISAR2_EL1` (macOS 27.0+).
    pub const ID_AA64ISAR2_EL1: Self = Self(0xc032);
    /// `ID_AA64MMFR0_EL1`.
    pub const ID_AA64MMFR0_EL1: Self = Self(0xc038);
    /// `ID_AA64MMFR1_EL1`.
    pub const ID_AA64MMFR1_EL1: Self = Self(0xc039);
    /// `ID_AA64MMFR2_EL1`.
    pub const ID_AA64MMFR2_EL1: Self = Self(0xc03a);
    /// `ID_AA64MMFR3_EL1` (macOS 27.0+).
    pub const ID_AA64MMFR3_EL1: Self = Self(0xc03b);
    /// `ID_AA64MMFR4_EL1` (macOS 27.0+).
    pub const ID_AA64MMFR4_EL1: Self = Self(0xc03c);
    /// `SCTLR_EL1`.
    pub const SCTLR_EL1: Self = Self(0xc080);
    /// `ACTLR_EL1` (macOS 15.0+). Only the `EnTSO` bit (bit 1) can be accessed: setting it selects the TSO memory model.
    pub const ACTLR_EL1: Self = Self(0xc081);
    /// `CPACR_EL1`.
    pub const CPACR_EL1: Self = Self(0xc082);
    /// `SMPRI_EL1` (macOS 15.2+).
    pub const SMPRI_EL1: Self = Self(0xc094);
    /// `SMCR_EL1` (macOS 15.2+).
    pub const SMCR_EL1: Self = Self(0xc096);
    /// `TTBR0_EL1`.
    pub const TTBR0_EL1: Self = Self(0xc100);
    /// `TTBR1_EL1`.
    pub const TTBR1_EL1: Self = Self(0xc101);
    /// `TCR_EL1`.
    pub const TCR_EL1: Self = Self(0xc102);
    /// `APIAKEYLO_EL1`.
    pub const APIAKEYLO_EL1: Self = Self(0xc108);
    /// `APIAKEYHI_EL1`.
    pub const APIAKEYHI_EL1: Self = Self(0xc109);
    /// `APIBKEYLO_EL1`.
    pub const APIBKEYLO_EL1: Self = Self(0xc10a);
    /// `APIBKEYHI_EL1`.
    pub const APIBKEYHI_EL1: Self = Self(0xc10b);
    /// `APDAKEYLO_EL1`.
    pub const APDAKEYLO_EL1: Self = Self(0xc110);
    /// `APDAKEYHI_EL1`.
    pub const APDAKEYHI_EL1: Self = Self(0xc111);
    /// `APDBKEYLO_EL1`.
    pub const APDBKEYLO_EL1: Self = Self(0xc112);
    /// `APDBKEYHI_EL1`.
    pub const APDBKEYHI_EL1: Self = Self(0xc113);
    /// `APGAKEYLO_EL1`.
    pub const APGAKEYLO_EL1: Self = Self(0xc118);
    /// `APGAKEYHI_EL1`.
    pub const APGAKEYHI_EL1: Self = Self(0xc119);
    /// `SPSR_EL1`.
    pub const SPSR_EL1: Self = Self(0xc200);
    /// `ELR_EL1`.
    pub const ELR_EL1: Self = Self(0xc201);
    /// `SP_EL0`.
    pub const SP_EL0: Self = Self(0xc208);
    /// `AFSR0_EL1`.
    pub const AFSR0_EL1: Self = Self(0xc288);
    /// `AFSR1_EL1`.
    pub const AFSR1_EL1: Self = Self(0xc289);
    /// `ESR_EL1`.
    pub const ESR_EL1: Self = Self(0xc290);
    /// `FAR_EL1`.
    pub const FAR_EL1: Self = Self(0xc300);
    /// `PAR_EL1`.
    pub const PAR_EL1: Self = Self(0xc3a0);
    /// `MAIR_EL1`.
    pub const MAIR_EL1: Self = Self(0xc510);
    /// `AMAIR_EL1`.
    pub const AMAIR_EL1: Self = Self(0xc518);
    /// `VBAR_EL1`.
    pub const VBAR_EL1: Self = Self(0xc600);
    /// `CONTEXTIDR_EL1`.
    pub const CONTEXTIDR_EL1: Self = Self(0xc681);
    /// `TPIDR_EL1`.
    pub const TPIDR_EL1: Self = Self(0xc684);
    /// `SCXTNUM_EL1` (macOS 15.2+).
    pub const SCXTNUM_EL1: Self = Self(0xc687);
    /// `CNTKCTL_EL1`.
    pub const CNTKCTL_EL1: Self = Self(0xc708);
    /// `CSSELR_EL1`.
    pub const CSSELR_EL1: Self = Self(0xd000);
    /// `TPIDR_EL0`.
    pub const TPIDR_EL0: Self = Self(0xde82);
    /// `TPIDRRO_EL0`.
    pub const TPIDRRO_EL0: Self = Self(0xde83);
    /// `TPIDR2_EL0` (macOS 15.2+).
    pub const TPIDR2_EL0: Self = Self(0xde85);
    /// `SCXTNUM_EL0` (macOS 15.2+).
    pub const SCXTNUM_EL0: Self = Self(0xde87);
    /// `CNTV_CTL_EL0`.
    pub const CNTV_CTL_EL0: Self = Self(0xdf19);
    /// `CNTV_CVAL_EL0`.
    pub const CNTV_CVAL_EL0: Self = Self(0xdf1a);
    /// `SP_EL1`.
    pub const SP_EL1: Self = Self(0xe208);
    // Physical timer registers: only available if the VM has a GIC device (macOS 15+).
    /// `CNTP_CTL_EL0` (macOS 15.0+).
    pub const CNTP_CTL_EL0: Self = Self(0xdf11);
    /// `CNTP_CVAL_EL0` (macOS 15.0+).
    pub const CNTP_CVAL_EL0: Self = Self(0xdf12);
    /// `CNTP_TVAL_EL0` (macOS 15.0+).
    pub const CNTP_TVAL_EL0: Self = Self(0xdf10);
    // EL2 registers: only available if EL2 is enabled in the VM configuration (macOS 15+).
    /// `CNTHCTL_EL2` (macOS 15.0+).
    pub const CNTHCTL_EL2: Self = Self(0xe708);
    /// `CNTHP_CTL_EL2` (macOS 15.0+).
    pub const CNTHP_CTL_EL2: Self = Self(0xe711);
    /// `CNTHP_CVAL_EL2` (macOS 15.0+).
    pub const CNTHP_CVAL_EL2: Self = Self(0xe712);
    /// `CNTHP_TVAL_EL2` (macOS 15.0+).
    pub const CNTHP_TVAL_EL2: Self = Self(0xe710);
    /// `CNTVOFF_EL2` (macOS 15.0+).
    pub const CNTVOFF_EL2: Self = Self(0xe703);
    /// `CPTR_EL2` (macOS 15.0+).
    pub const CPTR_EL2: Self = Self(0xe08a);
    /// `ELR_EL2` (macOS 15.0+).
    pub const ELR_EL2: Self = Self(0xe201);
    /// `ESR_EL2` (macOS 15.0+).
    pub const ESR_EL2: Self = Self(0xe290);
    /// `FAR_EL2` (macOS 15.0+).
    pub const FAR_EL2: Self = Self(0xe300);
    /// `HCR_EL2` (macOS 15.0+).
    pub const HCR_EL2: Self = Self(0xe088);
    /// `HPFAR_EL2` (macOS 15.0+).
    pub const HPFAR_EL2: Self = Self(0xe304);
    /// `MAIR_EL2` (macOS 15.0+).
    pub const MAIR_EL2: Self = Self(0xe510);
    /// `MDCR_EL2` (macOS 15.0+).
    pub const MDCR_EL2: Self = Self(0xe089);
    /// `SCTLR_EL2` (macOS 15.0+).
    pub const SCTLR_EL2: Self = Self(0xe080);
    /// `SPSR_EL2` (macOS 15.0+).
    pub const SPSR_EL2: Self = Self(0xe200);
    /// `SP_EL2` (macOS 15.0+).
    pub const SP_EL2: Self = Self(0xf208);
    /// `TCR_EL2` (macOS 15.0+).
    pub const TCR_EL2: Self = Self(0xe102);
    /// `TPIDR_EL2` (macOS 15.0+).
    pub const TPIDR_EL2: Self = Self(0xe682);
    /// `TTBR0_EL2` (macOS 15.0+).
    pub const TTBR0_EL2: Self = Self(0xe100);
    /// `TTBR1_EL2` (macOS 15.0+).
    pub const TTBR1_EL2: Self = Self(0xe101);
    /// `VBAR_EL2` (macOS 15.0+).
    pub const VBAR_EL2: Self = Self(0xe600);
    /// `VMPIDR_EL2` (macOS 15.0+).
    pub const VMPIDR_EL2: Self = Self(0xe005);
    /// `VPIDR_EL2` (macOS 15.0+).
    pub const VPIDR_EL2: Self = Self(0xe000);
    /// `VTCR_EL2` (macOS 15.0+).
    pub const VTCR_EL2: Self = Self(0xe10a);
    /// `VTTBR_EL2` (macOS 15.0+).
    pub const VTTBR_EL2: Self = Self(0xe108);
}

impl From<SysReg> for ffi::hv_sys_reg_t {
    fn from(reg: SysReg) -> Self {
        reg.0
    }
}

/// A virtual interrupt line (`hv_interrupt_type_t`).
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum InterruptType {
    /// IRQ.
    Irq = ffi::HV_INTERRUPT_TYPE_IRQ,
    /// FIQ.
    Fiq = ffi::HV_INTERRUPT_TYPE_FIQ,
}

/// A feature (ID) register that can be queried from a
/// [`VcpuConfig`](crate::VcpuConfig) (`hv_feature_reg_t`).
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum FeatureReg {
    /// `ID_AA64DFR0_EL1`.
    IdAa64Dfr0El1 = 0,
    /// `ID_AA64DFR1_EL1`.
    IdAa64Dfr1El1 = 1,
    /// `ID_AA64ISAR0_EL1`.
    IdAa64Isar0El1 = 2,
    /// `ID_AA64ISAR1_EL1`.
    IdAa64Isar1El1 = 3,
    /// `ID_AA64MMFR0_EL1`.
    IdAa64Mmfr0El1 = 4,
    /// `ID_AA64MMFR1_EL1`.
    IdAa64Mmfr1El1 = 5,
    /// `ID_AA64MMFR2_EL1`.
    IdAa64Mmfr2El1 = 6,
    /// `ID_AA64PFR0_EL1`.
    IdAa64Pfr0El1 = 7,
    /// `ID_AA64PFR1_EL1`.
    IdAa64Pfr1El1 = 8,
    /// `CTR_EL0`.
    CtrEl0 = 9,
    /// `CLIDR_EL1`.
    ClidrEl1 = 10,
    /// `DCZID_EL0`.
    DczidEl0 = 11,
    /// `ID_AA64SMFR0_EL1` (macOS 15.2+).
    IdAa64Smfr0El1 = 12,
    /// `ID_AA64ZFR0_EL1` (macOS 15.2+).
    IdAa64Zfr0El1 = 13,
}

/// Cache type for [`VcpuConfig::ccsidr_el1_values`](crate::VcpuConfig::ccsidr_el1_values)
/// (`hv_cache_type_t`).
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CacheType {
    /// Data or unified cache.
    Data = ffi::HV_CACHE_TYPE_DATA,
    /// Instruction cache.
    Instruction = ffi::HV_CACHE_TYPE_INSTRUCTION,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reg_values_match_sdk() {
        assert_eq!(Reg::X0 as u32, 0);
        assert_eq!(Reg::X30 as u32, 30);
        assert_eq!(Reg::FP, Reg::X29);
        assert_eq!(Reg::LR, Reg::X30);
        assert_eq!(Reg::Pc as u32, 31);
        assert_eq!(Reg::Cpsr as u32, 34);
        assert_eq!(SimdFpReg::Q31 as u32, 31);
        assert_eq!(SysReg::SCTLR_EL1.0, 0xc080);
        assert_eq!(SysReg::SP_EL1.0, 0xe208);
    }

    #[test]
    fn index_helpers() {
        for n in 0..31u8 {
            assert_eq!(Reg::x(n).map(|r| r as u32), Some(u32::from(n)));
        }
        assert_eq!(Reg::x(31), None);
        assert_eq!(SimdFpReg::q(31), Some(SimdFpReg::Q31));
        assert_eq!(SimdFpReg::q(32), None);
    }
}
