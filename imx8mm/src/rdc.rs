#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Version Information"]
    pub vir: VIR,
    _reserved0: [u8; 32usize],
    #[doc = "0x24 - Status"]
    pub stat: STAT,
    #[doc = "0x28 - Interrupt and Control"]
    pub intctrl: INTCTRL,
    #[doc = "0x2c - Interrupt Status"]
    pub intstat: INTSTAT,
    _reserved1: [u8; 464usize],
    #[doc = "0x200 - Master Domain Assignment"]
    pub mda: [MDA; 27],
    _reserved2: [u8; 404usize],
    #[doc = "0x400 - Peripheral Domain Access Permissions"]
    pub pdap: [PDAP; 118],
    _reserved3: [u8; 552usize],
    #[doc = "0x800 - Memory Region Start Address"]
    pub mrsa0: MRSA,
    #[doc = "0x804 - Memory Region End Address"]
    pub mrea0: MREA,
    #[doc = "0x808 - Memory Region Control"]
    pub mrc0: MRC,
    #[doc = "0x80c - Memory Region Violation Status"]
    pub mrvs0: MRVS,
    #[doc = "0x810 - Memory Region Start Address"]
    pub mrsa1: MRSA,
    #[doc = "0x814 - Memory Region End Address"]
    pub mrea1: MREA,
    #[doc = "0x818 - Memory Region Control"]
    pub mrc1: MRC,
    #[doc = "0x81c - Memory Region Violation Status"]
    pub mrvs1: MRVS,
    #[doc = "0x820 - Memory Region Start Address"]
    pub mrsa2: MRSA,
    #[doc = "0x824 - Memory Region End Address"]
    pub mrea2: MREA,
    #[doc = "0x828 - Memory Region Control"]
    pub mrc2: MRC,
    #[doc = "0x82c - Memory Region Violation Status"]
    pub mrvs2: MRVS,
    #[doc = "0x830 - Memory Region Start Address"]
    pub mrsa3: MRSA,
    #[doc = "0x834 - Memory Region End Address"]
    pub mrea3: MREA,
    #[doc = "0x838 - Memory Region Control"]
    pub mrc3: MRC,
    #[doc = "0x83c - Memory Region Violation Status"]
    pub mrvs3: MRVS,
    #[doc = "0x840 - Memory Region Start Address"]
    pub mrsa4: MRSA,
    #[doc = "0x844 - Memory Region End Address"]
    pub mrea4: MREA,
    #[doc = "0x848 - Memory Region Control"]
    pub mrc4: MRC,
    #[doc = "0x84c - Memory Region Violation Status"]
    pub mrvs4: MRVS,
    #[doc = "0x850 - Memory Region Start Address"]
    pub mrsa5: MRSA,
    #[doc = "0x854 - Memory Region End Address"]
    pub mrea5: MREA,
    #[doc = "0x858 - Memory Region Control"]
    pub mrc5: MRC,
    #[doc = "0x85c - Memory Region Violation Status"]
    pub mrvs5: MRVS,
    #[doc = "0x860 - Memory Region Start Address"]
    pub mrsa6: MRSA,
    #[doc = "0x864 - Memory Region End Address"]
    pub mrea6: MREA,
    #[doc = "0x868 - Memory Region Control"]
    pub mrc6: MRC,
    #[doc = "0x86c - Memory Region Violation Status"]
    pub mrvs6: MRVS,
    #[doc = "0x870 - Memory Region Start Address"]
    pub mrsa7: MRSA,
    #[doc = "0x874 - Memory Region End Address"]
    pub mrea7: MREA,
    #[doc = "0x878 - Memory Region Control"]
    pub mrc7: MRC,
    #[doc = "0x87c - Memory Region Violation Status"]
    pub mrvs7: MRVS,
    #[doc = "0x880 - Memory Region Start Address"]
    pub mrsa8: MRSA,
    #[doc = "0x884 - Memory Region End Address"]
    pub mrea8: MREA,
    #[doc = "0x888 - Memory Region Control"]
    pub mrc8: MRC,
    #[doc = "0x88c - Memory Region Violation Status"]
    pub mrvs8: MRVS,
    #[doc = "0x890 - Memory Region Start Address"]
    pub mrsa9: MRSA,
    #[doc = "0x894 - Memory Region End Address"]
    pub mrea9: MREA,
    #[doc = "0x898 - Memory Region Control"]
    pub mrc9: MRC,
    #[doc = "0x89c - Memory Region Violation Status"]
    pub mrvs9: MRVS,
    #[doc = "0x8a0 - Memory Region Start Address"]
    pub mrsa10: MRSA,
    #[doc = "0x8a4 - Memory Region End Address"]
    pub mrea10: MREA,
    #[doc = "0x8a8 - Memory Region Control"]
    pub mrc10: MRC,
    #[doc = "0x8ac - Memory Region Violation Status"]
    pub mrvs10: MRVS,
    #[doc = "0x8b0 - Memory Region Start Address"]
    pub mrsa11: MRSA,
    #[doc = "0x8b4 - Memory Region End Address"]
    pub mrea11: MREA,
    #[doc = "0x8b8 - Memory Region Control"]
    pub mrc11: MRC,
    #[doc = "0x8bc - Memory Region Violation Status"]
    pub mrvs11: MRVS,
    #[doc = "0x8c0 - Memory Region Start Address"]
    pub mrsa12: MRSA,
    #[doc = "0x8c4 - Memory Region End Address"]
    pub mrea12: MREA,
    #[doc = "0x8c8 - Memory Region Control"]
    pub mrc12: MRC,
    #[doc = "0x8cc - Memory Region Violation Status"]
    pub mrvs12: MRVS,
    #[doc = "0x8d0 - Memory Region Start Address"]
    pub mrsa13: MRSA,
    #[doc = "0x8d4 - Memory Region End Address"]
    pub mrea13: MREA,
    #[doc = "0x8d8 - Memory Region Control"]
    pub mrc13: MRC,
    #[doc = "0x8dc - Memory Region Violation Status"]
    pub mrvs13: MRVS,
    #[doc = "0x8e0 - Memory Region Start Address"]
    pub mrsa14: MRSA,
    #[doc = "0x8e4 - Memory Region End Address"]
    pub mrea14: MREA,
    #[doc = "0x8e8 - Memory Region Control"]
    pub mrc14: MRC,
    #[doc = "0x8ec - Memory Region Violation Status"]
    pub mrvs14: MRVS,
    #[doc = "0x8f0 - Memory Region Start Address"]
    pub mrsa15: MRSA,
    #[doc = "0x8f4 - Memory Region End Address"]
    pub mrea15: MREA,
    #[doc = "0x8f8 - Memory Region Control"]
    pub mrc15: MRC,
    #[doc = "0x8fc - Memory Region Violation Status"]
    pub mrvs15: MRVS,
    #[doc = "0x900 - Memory Region Start Address"]
    pub mrsa16: MRSA,
    #[doc = "0x904 - Memory Region End Address"]
    pub mrea16: MREA,
    #[doc = "0x908 - Memory Region Control"]
    pub mrc16: MRC,
    #[doc = "0x90c - Memory Region Violation Status"]
    pub mrvs16: MRVS,
    #[doc = "0x910 - Memory Region Start Address"]
    pub mrsa17: MRSA,
    #[doc = "0x914 - Memory Region End Address"]
    pub mrea17: MREA,
    #[doc = "0x918 - Memory Region Control"]
    pub mrc17: MRC,
    #[doc = "0x91c - Memory Region Violation Status"]
    pub mrvs17: MRVS,
    #[doc = "0x920 - Memory Region Start Address"]
    pub mrsa18: MRSA,
    #[doc = "0x924 - Memory Region End Address"]
    pub mrea18: MREA,
    #[doc = "0x928 - Memory Region Control"]
    pub mrc18: MRC,
    #[doc = "0x92c - Memory Region Violation Status"]
    pub mrvs18: MRVS,
    #[doc = "0x930 - Memory Region Start Address"]
    pub mrsa19: MRSA,
    #[doc = "0x934 - Memory Region End Address"]
    pub mrea19: MREA,
    #[doc = "0x938 - Memory Region Control"]
    pub mrc19: MRC,
    #[doc = "0x93c - Memory Region Violation Status"]
    pub mrvs19: MRVS,
    #[doc = "0x940 - Memory Region Start Address"]
    pub mrsa20: MRSA,
    #[doc = "0x944 - Memory Region End Address"]
    pub mrea20: MREA,
    #[doc = "0x948 - Memory Region Control"]
    pub mrc20: MRC,
    #[doc = "0x94c - Memory Region Violation Status"]
    pub mrvs20: MRVS,
    #[doc = "0x950 - Memory Region Start Address"]
    pub mrsa21: MRSA,
    #[doc = "0x954 - Memory Region End Address"]
    pub mrea21: MREA,
    #[doc = "0x958 - Memory Region Control"]
    pub mrc21: MRC,
    #[doc = "0x95c - Memory Region Violation Status"]
    pub mrvs21: MRVS,
    #[doc = "0x960 - Memory Region Start Address"]
    pub mrsa22: MRSA,
    #[doc = "0x964 - Memory Region End Address"]
    pub mrea22: MREA,
    #[doc = "0x968 - Memory Region Control"]
    pub mrc22: MRC,
    #[doc = "0x96c - Memory Region Violation Status"]
    pub mrvs22: MRVS,
    #[doc = "0x970 - Memory Region Start Address"]
    pub mrsa23: MRSA,
    #[doc = "0x974 - Memory Region End Address"]
    pub mrea23: MREA,
    #[doc = "0x978 - Memory Region Control"]
    pub mrc23: MRC,
    #[doc = "0x97c - Memory Region Violation Status"]
    pub mrvs23: MRVS,
    #[doc = "0x980 - Memory Region Start Address"]
    pub mrsa24: MRSA,
    #[doc = "0x984 - Memory Region End Address"]
    pub mrea24: MREA,
    #[doc = "0x988 - Memory Region Control"]
    pub mrc24: MRC,
    #[doc = "0x98c - Memory Region Violation Status"]
    pub mrvs24: MRVS,
    #[doc = "0x990 - Memory Region Start Address"]
    pub mrsa25: MRSA,
    #[doc = "0x994 - Memory Region End Address"]
    pub mrea25: MREA,
    #[doc = "0x998 - Memory Region Control"]
    pub mrc25: MRC,
    #[doc = "0x99c - Memory Region Violation Status"]
    pub mrvs25: MRVS,
    #[doc = "0x9a0 - Memory Region Start Address"]
    pub mrsa26: MRSA,
    #[doc = "0x9a4 - Memory Region End Address"]
    pub mrea26: MREA,
    #[doc = "0x9a8 - Memory Region Control"]
    pub mrc26: MRC,
    #[doc = "0x9ac - Memory Region Violation Status"]
    pub mrvs26: MRVS,
    #[doc = "0x9b0 - Memory Region Start Address"]
    pub mrsa27: MRSA,
    #[doc = "0x9b4 - Memory Region End Address"]
    pub mrea27: MREA,
    #[doc = "0x9b8 - Memory Region Control"]
    pub mrc27: MRC,
    #[doc = "0x9bc - Memory Region Violation Status"]
    pub mrvs27: MRVS,
    #[doc = "0x9c0 - Memory Region Start Address"]
    pub mrsa28: MRSA,
    #[doc = "0x9c4 - Memory Region End Address"]
    pub mrea28: MREA,
    #[doc = "0x9c8 - Memory Region Control"]
    pub mrc28: MRC,
    #[doc = "0x9cc - Memory Region Violation Status"]
    pub mrvs28: MRVS,
    #[doc = "0x9d0 - Memory Region Start Address"]
    pub mrsa29: MRSA,
    #[doc = "0x9d4 - Memory Region End Address"]
    pub mrea29: MREA,
    #[doc = "0x9d8 - Memory Region Control"]
    pub mrc29: MRC,
    #[doc = "0x9dc - Memory Region Violation Status"]
    pub mrvs29: MRVS,
    #[doc = "0x9e0 - Memory Region Start Address"]
    pub mrsa30: MRSA,
    #[doc = "0x9e4 - Memory Region End Address"]
    pub mrea30: MREA,
    #[doc = "0x9e8 - Memory Region Control"]
    pub mrc30: MRC,
    #[doc = "0x9ec - Memory Region Violation Status"]
    pub mrvs30: MRVS,
    #[doc = "0x9f0 - Memory Region Start Address"]
    pub mrsa31: MRSA,
    #[doc = "0x9f4 - Memory Region End Address"]
    pub mrea31: MREA,
    #[doc = "0x9f8 - Memory Region Control"]
    pub mrc31: MRC,
    #[doc = "0x9fc - Memory Region Violation Status"]
    pub mrvs31: MRVS,
    #[doc = "0xa00 - Memory Region Start Address"]
    pub mrsa32: MRSA,
    #[doc = "0xa04 - Memory Region End Address"]
    pub mrea32: MREA,
    #[doc = "0xa08 - Memory Region Control"]
    pub mrc32: MRC,
    #[doc = "0xa0c - Memory Region Violation Status"]
    pub mrvs32: MRVS,
    #[doc = "0xa10 - Memory Region Start Address"]
    pub mrsa33: MRSA,
    #[doc = "0xa14 - Memory Region End Address"]
    pub mrea33: MREA,
    #[doc = "0xa18 - Memory Region Control"]
    pub mrc33: MRC,
    #[doc = "0xa1c - Memory Region Violation Status"]
    pub mrvs33: MRVS,
    #[doc = "0xa20 - Memory Region Start Address"]
    pub mrsa34: MRSA,
    #[doc = "0xa24 - Memory Region End Address"]
    pub mrea34: MREA,
    #[doc = "0xa28 - Memory Region Control"]
    pub mrc34: MRC,
    #[doc = "0xa2c - Memory Region Violation Status"]
    pub mrvs34: MRVS,
    #[doc = "0xa30 - Memory Region Start Address"]
    pub mrsa35: MRSA,
    #[doc = "0xa34 - Memory Region End Address"]
    pub mrea35: MREA,
    #[doc = "0xa38 - Memory Region Control"]
    pub mrc35: MRC,
    #[doc = "0xa3c - Memory Region Violation Status"]
    pub mrvs35: MRVS,
    #[doc = "0xa40 - Memory Region Start Address"]
    pub mrsa36: MRSA,
    #[doc = "0xa44 - Memory Region End Address"]
    pub mrea36: MREA,
    #[doc = "0xa48 - Memory Region Control"]
    pub mrc36: MRC,
    #[doc = "0xa4c - Memory Region Violation Status"]
    pub mrvs36: MRVS,
    #[doc = "0xa50 - Memory Region Start Address"]
    pub mrsa37: MRSA,
    #[doc = "0xa54 - Memory Region End Address"]
    pub mrea37: MREA,
    #[doc = "0xa58 - Memory Region Control"]
    pub mrc37: MRC,
    #[doc = "0xa5c - Memory Region Violation Status"]
    pub mrvs37: MRVS,
    #[doc = "0xa60 - Memory Region Start Address"]
    pub mrsa38: MRSA,
    #[doc = "0xa64 - Memory Region End Address"]
    pub mrea38: MREA,
    #[doc = "0xa68 - Memory Region Control"]
    pub mrc38: MRC,
    #[doc = "0xa6c - Memory Region Violation Status"]
    pub mrvs38: MRVS,
    #[doc = "0xa70 - Memory Region Start Address"]
    pub mrsa39: MRSA,
    #[doc = "0xa74 - Memory Region End Address"]
    pub mrea39: MREA,
    #[doc = "0xa78 - Memory Region Control"]
    pub mrc39: MRC,
    #[doc = "0xa7c - Memory Region Violation Status"]
    pub mrvs39: MRVS,
    #[doc = "0xa80 - Memory Region Start Address"]
    pub mrsa40: MRSA,
    #[doc = "0xa84 - Memory Region End Address"]
    pub mrea40: MREA,
    #[doc = "0xa88 - Memory Region Control"]
    pub mrc40: MRC,
    #[doc = "0xa8c - Memory Region Violation Status"]
    pub mrvs40: MRVS,
    #[doc = "0xa90 - Memory Region Start Address"]
    pub mrsa41: MRSA,
    #[doc = "0xa94 - Memory Region End Address"]
    pub mrea41: MREA,
    #[doc = "0xa98 - Memory Region Control"]
    pub mrc41: MRC,
    #[doc = "0xa9c - Memory Region Violation Status"]
    pub mrvs41: MRVS,
    #[doc = "0xaa0 - Memory Region Start Address"]
    pub mrsa42: MRSA,
    #[doc = "0xaa4 - Memory Region End Address"]
    pub mrea42: MREA,
    #[doc = "0xaa8 - Memory Region Control"]
    pub mrc42: MRC,
    #[doc = "0xaac - Memory Region Violation Status"]
    pub mrvs42: MRVS,
    #[doc = "0xab0 - Memory Region Start Address"]
    pub mrsa43: MRSA,
    #[doc = "0xab4 - Memory Region End Address"]
    pub mrea43: MREA,
    #[doc = "0xab8 - Memory Region Control"]
    pub mrc43: MRC,
    #[doc = "0xabc - Memory Region Violation Status"]
    pub mrvs43: MRVS,
    #[doc = "0xac0 - Memory Region Start Address"]
    pub mrsa44: MRSA,
    #[doc = "0xac4 - Memory Region End Address"]
    pub mrea44: MREA,
    #[doc = "0xac8 - Memory Region Control"]
    pub mrc44: MRC,
    #[doc = "0xacc - Memory Region Violation Status"]
    pub mrvs44: MRVS,
    #[doc = "0xad0 - Memory Region Start Address"]
    pub mrsa45: MRSA,
    #[doc = "0xad4 - Memory Region End Address"]
    pub mrea45: MREA,
    #[doc = "0xad8 - Memory Region Control"]
    pub mrc45: MRC,
    #[doc = "0xadc - Memory Region Violation Status"]
    pub mrvs45: MRVS,
    #[doc = "0xae0 - Memory Region Start Address"]
    pub mrsa46: MRSA,
    #[doc = "0xae4 - Memory Region End Address"]
    pub mrea46: MREA,
    #[doc = "0xae8 - Memory Region Control"]
    pub mrc46: MRC,
    #[doc = "0xaec - Memory Region Violation Status"]
    pub mrvs46: MRVS,
    #[doc = "0xaf0 - Memory Region Start Address"]
    pub mrsa47: MRSA,
    #[doc = "0xaf4 - Memory Region End Address"]
    pub mrea47: MREA,
    #[doc = "0xaf8 - Memory Region Control"]
    pub mrc47: MRC,
    #[doc = "0xafc - Memory Region Violation Status"]
    pub mrvs47: MRVS,
    #[doc = "0xb00 - Memory Region Start Address"]
    pub mrsa48: MRSA,
    #[doc = "0xb04 - Memory Region End Address"]
    pub mrea48: MREA,
    #[doc = "0xb08 - Memory Region Control"]
    pub mrc48: MRC,
    #[doc = "0xb0c - Memory Region Violation Status"]
    pub mrvs48: MRVS,
    #[doc = "0xb10 - Memory Region Start Address"]
    pub mrsa49: MRSA,
    #[doc = "0xb14 - Memory Region End Address"]
    pub mrea49: MREA,
    #[doc = "0xb18 - Memory Region Control"]
    pub mrc49: MRC,
    #[doc = "0xb1c - Memory Region Violation Status"]
    pub mrvs49: MRVS,
    #[doc = "0xb20 - Memory Region Start Address"]
    pub mrsa50: MRSA,
    #[doc = "0xb24 - Memory Region End Address"]
    pub mrea50: MREA,
    #[doc = "0xb28 - Memory Region Control"]
    pub mrc50: MRC,
    #[doc = "0xb2c - Memory Region Violation Status"]
    pub mrvs50: MRVS,
    #[doc = "0xb30 - Memory Region Start Address"]
    pub mrsa51: MRSA,
    #[doc = "0xb34 - Memory Region End Address"]
    pub mrea51: MREA,
    #[doc = "0xb38 - Memory Region Control"]
    pub mrc51: MRC,
    #[doc = "0xb3c - Memory Region Violation Status"]
    pub mrvs51: MRVS,
}
#[doc = "Version Information"]
pub struct VIR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Version Information"]
pub mod vir;
#[doc = "Status"]
pub struct STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status"]
pub mod stat;
#[doc = "Interrupt and Control"]
pub struct INTCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt and Control"]
pub mod intctrl;
#[doc = "Interrupt Status"]
pub struct INTSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Status"]
pub mod intstat;
#[doc = "Master Domain Assignment"]
pub struct MDA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Master Domain Assignment"]
pub mod mda;
#[doc = "Peripheral Domain Access Permissions"]
pub struct PDAP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Domain Access Permissions"]
pub mod pdap;
#[doc = "Memory Region Start Address"]
pub struct MRSA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Region Start Address"]
pub mod mrsa;
#[doc = "Memory Region End Address"]
pub struct MREA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Region End Address"]
pub mod mrea;
#[doc = "Memory Region Control"]
pub struct MRC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Region Control"]
pub mod mrc;
#[doc = "Memory Region Violation Status"]
pub struct MRVS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Region Violation Status"]
pub mod mrvs;
