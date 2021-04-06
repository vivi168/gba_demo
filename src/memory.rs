#![allow(dead_code)]

// Memory map

pub const EX_WRAM         : u32 = 0x02000000;
pub const CPU_WRAM        : u32 = 0x03000000;
pub const CPU_WRAM_END    : u32 = CPU_WRAM + 0x8000;
pub const INTR_VECTOR_BUF : u32 = CPU_WRAM_END - 0x4;
pub const INTR_CHECK_BUF  : u32 = CPU_WRAM_END - 0x8;

pub const PALETTE         : u32 = 0x05000000;
pub const PALETTE_OAM     : u32 = PALETTE + 0x200;
pub const VRAM            : u32 = 0x06000000;
pub const BG_VRAM         : u32 = VRAM;
pub const BG_CH_BLOCK_0   : u32 = BG_VRAM;
pub const BG_CH_BLOCK_1   : u32 = BG_VRAM + 0x4000;
pub const BG_CH_BLOCK_2   : u32 = BG_VRAM + 0x8000;
pub const BG_CH_BLOCK_3   : u32 = BG_VRAM + 0xc000;
pub const OBJ_MODE0_VRAM  : u32 = VRAM + 0x10000;

pub const OAM             : u32 = 0x07000000;

// Memory size

pub const EX_WRAM_SIZE    : u32 = 0x40000;
pub const CPU_WRAM_SIZE   : u32 =  0x8000;

// Registers

pub const REG_BASE        : u32 = 0x4000_000;
pub const REG_DISPCNT     : u32 = REG_BASE;
pub const REG_DISPSTAT    : u32 = REG_BASE + 0x4;

pub const REG_BG0CNT      : u32 = REG_BASE + 0x8;
pub const REG_BG0HOFS     : u32 = REG_BASE + 0x10;
pub const REG_BG0VOFS     : u32 = REG_BASE + 0x12;

pub const REG_IE          : u32 = REG_BASE + 0x200;
pub const REG_IME         : u32 = REG_BASE + 0x208;

pub const REG_DMA3        : u32 = REG_BASE + 0xd4;
pub const REG_DMA3SAD     : u32 = REG_BASE + 0xd4;
pub const REG_DMA3DAD     : u32 = REG_BASE + 0xd8;
pub const REG_DMA3CNT     : u32 = REG_BASE + 0xdc;

pub const REG_KEYINPUT    : u32 = REG_BASE + 0x130;
pub const REG_KEYCNT      : u32 = REG_BASE + 0x132;
