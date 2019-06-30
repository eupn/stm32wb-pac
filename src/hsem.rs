#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Semaphore 0 register"]
    pub r0: R0,
    #[doc = "0x04 - Semaphore 1 register"]
    pub r1: R1,
    #[doc = "0x08 - Semaphore 2 register"]
    pub r2: R2,
    #[doc = "0x0c - Semaphore 3 register"]
    pub r3: R3,
    #[doc = "0x10 - Semaphore 4 register"]
    pub r4: R4,
    #[doc = "0x14 - Semaphore 5 register"]
    pub r5: R5,
    #[doc = "0x18 - Semaphore 6 register"]
    pub r6: R6,
    #[doc = "0x1c - Semaphore 7 register"]
    pub r7: R7,
    #[doc = "0x20 - Semaphore 8 register"]
    pub r8: R8,
    #[doc = "0x24 - Semaphore 9 register"]
    pub r9: R9,
    #[doc = "0x28 - Semaphore 10 register"]
    pub r10: R10,
    #[doc = "0x2c - Semaphore 11 register"]
    pub r11: R11,
    #[doc = "0x30 - Semaphore 12 register"]
    pub r12: R12,
    #[doc = "0x34 - Semaphore 13 register"]
    pub r13: R13,
    #[doc = "0x38 - Semaphore 14 register"]
    pub r14: R14,
    #[doc = "0x3c - Semaphore 15 register"]
    pub r15: R15,
    #[doc = "0x40 - Semaphore 16 register"]
    pub r16: R16,
    #[doc = "0x44 - Semaphore 17 register"]
    pub r17: R17,
    #[doc = "0x48 - Semaphore 18 register"]
    pub r18: R18,
    #[doc = "0x4c - Semaphore 19 register"]
    pub r19: R19,
    #[doc = "0x50 - Semaphore 20 register"]
    pub r20: R20,
    #[doc = "0x54 - Semaphore 21 register"]
    pub r21: R21,
    #[doc = "0x58 - Semaphore 22 register"]
    pub r22: R22,
    #[doc = "0x5c - Semaphore 23 register"]
    pub r23: R23,
    #[doc = "0x60 - Semaphore 24 register"]
    pub r24: R24,
    #[doc = "0x64 - Semaphore 25 register"]
    pub r25: R25,
    #[doc = "0x68 - Semaphore 26 register"]
    pub r26: R26,
    #[doc = "0x6c - Semaphore 27 register"]
    pub r27: R27,
    #[doc = "0x70 - Semaphore 28 register"]
    pub r28: R28,
    #[doc = "0x74 - Semaphore 29 register"]
    pub r29: R29,
    #[doc = "0x78 - Semaphore 30 register"]
    pub r30: R30,
    #[doc = "0x7c - Semaphore 31 register"]
    pub r31: R31,
    #[doc = "0x80 - Semaphore 0 read lock register"]
    pub rlr0: RLR0,
    #[doc = "0x84 - Semaphore 1 read lock register"]
    pub rlr1: RLR1,
    #[doc = "0x88 - Semaphore 2 read lock register"]
    pub rlr2: RLR2,
    #[doc = "0x8c - Semaphore 3 read lock register"]
    pub rlr3: RLR3,
    #[doc = "0x90 - Semaphore 4 read lock read lock register"]
    pub rlr4: RLR4,
    #[doc = "0x94 - Semaphore 5 read lock register"]
    pub rlr5: RLR5,
    #[doc = "0x98 - Semaphore 6 read lock register"]
    pub rlr6: RLR6,
    #[doc = "0x9c - Semaphore 7 read lock register"]
    pub rlr7: RLR7,
    #[doc = "0xa0 - Semaphore 8 read lock register"]
    pub rlr8: RLR8,
    #[doc = "0xa4 - Semaphore 9 read lock register"]
    pub rlr9: RLR9,
    #[doc = "0xa8 - Semaphore 10 read lock register"]
    pub rlr10: RLR10,
    #[doc = "0xac - Semaphore 11 read lock register"]
    pub rlr11: RLR11,
    #[doc = "0xb0 - Semaphore 12 read lock register"]
    pub rlr12: RLR12,
    #[doc = "0xb4 - Semaphore 13 read lock register"]
    pub rlr13: RLR13,
    #[doc = "0xb8 - Semaphore 14 read lock register"]
    pub rlr14: RLR14,
    #[doc = "0xbc - Semaphore 15 read lock register"]
    pub rlr15: RLR15,
    #[doc = "0xc0 - Semaphore 16 read lock register"]
    pub rlr16: RLR16,
    #[doc = "0xc4 - Semaphore 17 read lock register"]
    pub rlr17: RLR17,
    #[doc = "0xc8 - Semaphore 18 read lock register"]
    pub rlr18: RLR18,
    #[doc = "0xcc - Semaphore 19 read lock register"]
    pub rlr19: RLR19,
    #[doc = "0xd0 - Semaphore 20 read lock register"]
    pub rlr20: RLR20,
    #[doc = "0xd4 - Semaphore 21 read lock register"]
    pub rlr21: RLR21,
    #[doc = "0xd8 - Semaphore 22 read lock register"]
    pub rlr22: RLR22,
    #[doc = "0xdc - Semaphore 23 read lock register"]
    pub rlr23: RLR23,
    #[doc = "0xe0 - Semaphore 24 read lock register"]
    pub rlr24: RLR24,
    #[doc = "0xe4 - Semaphore 25 read lock register"]
    pub rlr25: RLR25,
    #[doc = "0xe8 - Semaphore 26 read lock register"]
    pub rlr26: RLR26,
    #[doc = "0xec - Semaphore 27 read lock register"]
    pub rlr27: RLR27,
    #[doc = "0xf0 - Semaphore 28 read lock register"]
    pub rlr28: RLR28,
    #[doc = "0xf4 - Semaphore 29 read lock register"]
    pub rlr29: RLR29,
    #[doc = "0xf8 - Semaphore 30 read lock register"]
    pub rlr30: RLR30,
    #[doc = "0xfc - Semaphore 31 read lock register"]
    pub rlr31: RLR31,
    #[doc = "0x100 - HSEM Interrupt enable register"]
    pub c1ier0: C1IER0,
    #[doc = "0x104 - HSEM Interrupt clear register"]
    pub c1icr: C1ICR,
    #[doc = "0x108 - HSEM Interrupt status register"]
    pub c1isr: C1ISR,
    #[doc = "0x10c - HSEM Masked interrupt status register"]
    pub c1misr: C1MISR,
    #[doc = "0x110 - HSEM Interrupt enable register"]
    pub c2ier0: C2IER0,
    #[doc = "0x114 - HSEM Interrupt clear register"]
    pub c2icr: C2ICR,
    #[doc = "0x118 - HSEM Interrupt status register"]
    pub c2isr: C2ISR,
    #[doc = "0x11c - HSEM Masked interrupt status register"]
    pub c2misr: C2MISR,
    _reserved0: [u8; 32usize],
    #[doc = "0x140 - Semaphore Clear register"]
    pub cr: CR,
    #[doc = "0x144 - Interrupt clear register"]
    pub keyr: KEYR,
    _reserved1: [u8; 676usize],
    #[doc = "0x3ec - Semaphore hardware configuration register 2"]
    pub hwcfgr2: HWCFGR2,
    #[doc = "0x3f0 - Semaphore hardware configuration register 1"]
    pub hwcfgr1: HWCFGR1,
    #[doc = "0x3f4 - HSEM version register"]
    pub verr: VERR,
    #[doc = "0x3f8 - HSEM indentification register"]
    pub ipidr: IPIDR,
    #[doc = "0x3fc - HSEM size indentification register"]
    pub sidr: SIDR,
}
#[doc = "Semaphore 0 register"]
pub struct R0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 0 register"]
pub mod r0;
#[doc = "Semaphore 1 register"]
pub struct R1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 1 register"]
pub mod r1;
#[doc = "Semaphore 2 register"]
pub struct R2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 2 register"]
pub mod r2;
#[doc = "Semaphore 3 register"]
pub struct R3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 3 register"]
pub mod r3;
#[doc = "Semaphore 4 register"]
pub struct R4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 4 register"]
pub mod r4;
#[doc = "Semaphore 5 register"]
pub struct R5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 5 register"]
pub mod r5;
#[doc = "Semaphore 6 register"]
pub struct R6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 6 register"]
pub mod r6;
#[doc = "Semaphore 7 register"]
pub struct R7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 7 register"]
pub mod r7;
#[doc = "Semaphore 8 register"]
pub struct R8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 8 register"]
pub mod r8;
#[doc = "Semaphore 9 register"]
pub struct R9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 9 register"]
pub mod r9;
#[doc = "Semaphore 10 register"]
pub struct R10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 10 register"]
pub mod r10;
#[doc = "Semaphore 11 register"]
pub struct R11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 11 register"]
pub mod r11;
#[doc = "Semaphore 12 register"]
pub struct R12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 12 register"]
pub mod r12;
#[doc = "Semaphore 13 register"]
pub struct R13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 13 register"]
pub mod r13;
#[doc = "Semaphore 14 register"]
pub struct R14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 14 register"]
pub mod r14;
#[doc = "Semaphore 15 register"]
pub struct R15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 15 register"]
pub mod r15;
#[doc = "Semaphore 16 register"]
pub struct R16 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 16 register"]
pub mod r16;
#[doc = "Semaphore 17 register"]
pub struct R17 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 17 register"]
pub mod r17;
#[doc = "Semaphore 18 register"]
pub struct R18 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 18 register"]
pub mod r18;
#[doc = "Semaphore 19 register"]
pub struct R19 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 19 register"]
pub mod r19;
#[doc = "Semaphore 20 register"]
pub struct R20 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 20 register"]
pub mod r20;
#[doc = "Semaphore 21 register"]
pub struct R21 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 21 register"]
pub mod r21;
#[doc = "Semaphore 22 register"]
pub struct R22 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 22 register"]
pub mod r22;
#[doc = "Semaphore 23 register"]
pub struct R23 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 23 register"]
pub mod r23;
#[doc = "Semaphore 24 register"]
pub struct R24 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 24 register"]
pub mod r24;
#[doc = "Semaphore 25 register"]
pub struct R25 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 25 register"]
pub mod r25;
#[doc = "Semaphore 26 register"]
pub struct R26 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 26 register"]
pub mod r26;
#[doc = "Semaphore 27 register"]
pub struct R27 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 27 register"]
pub mod r27;
#[doc = "Semaphore 28 register"]
pub struct R28 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 28 register"]
pub mod r28;
#[doc = "Semaphore 29 register"]
pub struct R29 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 29 register"]
pub mod r29;
#[doc = "Semaphore 30 register"]
pub struct R30 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 30 register"]
pub mod r30;
#[doc = "Semaphore 31 register"]
pub struct R31 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 31 register"]
pub mod r31;
#[doc = "Semaphore 0 read lock register"]
pub struct RLR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 0 read lock register"]
pub mod rlr0;
#[doc = "Semaphore 1 read lock register"]
pub struct RLR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 1 read lock register"]
pub mod rlr1;
#[doc = "Semaphore 2 read lock register"]
pub struct RLR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 2 read lock register"]
pub mod rlr2;
#[doc = "Semaphore 3 read lock register"]
pub struct RLR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 3 read lock register"]
pub mod rlr3;
#[doc = "Semaphore 4 read lock read lock register"]
pub struct RLR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 4 read lock read lock register"]
pub mod rlr4;
#[doc = "Semaphore 5 read lock register"]
pub struct RLR5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 5 read lock register"]
pub mod rlr5;
#[doc = "Semaphore 6 read lock register"]
pub struct RLR6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 6 read lock register"]
pub mod rlr6;
#[doc = "Semaphore 7 read lock register"]
pub struct RLR7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 7 read lock register"]
pub mod rlr7;
#[doc = "Semaphore 8 read lock register"]
pub struct RLR8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 8 read lock register"]
pub mod rlr8;
#[doc = "Semaphore 9 read lock register"]
pub struct RLR9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 9 read lock register"]
pub mod rlr9;
#[doc = "Semaphore 10 read lock register"]
pub struct RLR10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 10 read lock register"]
pub mod rlr10;
#[doc = "Semaphore 11 read lock register"]
pub struct RLR11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 11 read lock register"]
pub mod rlr11;
#[doc = "Semaphore 12 read lock register"]
pub struct RLR12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 12 read lock register"]
pub mod rlr12;
#[doc = "Semaphore 13 read lock register"]
pub struct RLR13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 13 read lock register"]
pub mod rlr13;
#[doc = "Semaphore 14 read lock register"]
pub struct RLR14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 14 read lock register"]
pub mod rlr14;
#[doc = "Semaphore 15 read lock register"]
pub struct RLR15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 15 read lock register"]
pub mod rlr15;
#[doc = "Semaphore 16 read lock register"]
pub struct RLR16 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 16 read lock register"]
pub mod rlr16;
#[doc = "Semaphore 17 read lock register"]
pub struct RLR17 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 17 read lock register"]
pub mod rlr17;
#[doc = "Semaphore 18 read lock register"]
pub struct RLR18 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 18 read lock register"]
pub mod rlr18;
#[doc = "Semaphore 19 read lock register"]
pub struct RLR19 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 19 read lock register"]
pub mod rlr19;
#[doc = "Semaphore 20 read lock register"]
pub struct RLR20 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 20 read lock register"]
pub mod rlr20;
#[doc = "Semaphore 21 read lock register"]
pub struct RLR21 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 21 read lock register"]
pub mod rlr21;
#[doc = "Semaphore 22 read lock register"]
pub struct RLR22 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 22 read lock register"]
pub mod rlr22;
#[doc = "Semaphore 23 read lock register"]
pub struct RLR23 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 23 read lock register"]
pub mod rlr23;
#[doc = "Semaphore 24 read lock register"]
pub struct RLR24 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 24 read lock register"]
pub mod rlr24;
#[doc = "Semaphore 25 read lock register"]
pub struct RLR25 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 25 read lock register"]
pub mod rlr25;
#[doc = "Semaphore 26 read lock register"]
pub struct RLR26 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 26 read lock register"]
pub mod rlr26;
#[doc = "Semaphore 27 read lock register"]
pub struct RLR27 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 27 read lock register"]
pub mod rlr27;
#[doc = "Semaphore 28 read lock register"]
pub struct RLR28 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 28 read lock register"]
pub mod rlr28;
#[doc = "Semaphore 29 read lock register"]
pub struct RLR29 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 29 read lock register"]
pub mod rlr29;
#[doc = "Semaphore 30 read lock register"]
pub struct RLR30 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 30 read lock register"]
pub mod rlr30;
#[doc = "Semaphore 31 read lock register"]
pub struct RLR31 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 31 read lock register"]
pub mod rlr31;
#[doc = "Semaphore Clear register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore Clear register"]
pub mod cr;
#[doc = "Interrupt clear register"]
pub struct KEYR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt clear register"]
pub mod keyr;
#[doc = "Semaphore hardware configuration register 2"]
pub struct HWCFGR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore hardware configuration register 2"]
pub mod hwcfgr2;
#[doc = "Semaphore hardware configuration register 1"]
pub struct HWCFGR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore hardware configuration register 1"]
pub mod hwcfgr1;
#[doc = "HSEM version register"]
pub struct VERR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HSEM version register"]
pub mod verr;
#[doc = "HSEM indentification register"]
pub struct IPIDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HSEM indentification register"]
pub mod ipidr;
#[doc = "HSEM size indentification register"]
pub struct SIDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HSEM size indentification register"]
pub mod sidr;
#[doc = "HSEM Interrupt enable register"]
pub struct C1IER0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HSEM Interrupt enable register"]
pub mod c1ier0;
#[doc = "HSEM Interrupt clear register"]
pub struct C1ICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HSEM Interrupt clear register"]
pub mod c1icr;
#[doc = "HSEM Interrupt status register"]
pub struct C1ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HSEM Interrupt status register"]
pub mod c1isr;
#[doc = "HSEM Masked interrupt status register"]
pub struct C1MISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HSEM Masked interrupt status register"]
pub mod c1misr;
#[doc = "HSEM Interrupt enable register"]
pub struct C2IER0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HSEM Interrupt enable register"]
pub mod c2ier0;
#[doc = "HSEM Interrupt clear register"]
pub struct C2ICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HSEM Interrupt clear register"]
pub mod c2icr;
#[doc = "HSEM Interrupt status register"]
pub struct C2ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HSEM Interrupt status register"]
pub mod c2isr;
#[doc = "HSEM Masked interrupt status register"]
pub struct C2MISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HSEM Masked interrupt status register"]
pub mod c2misr;
