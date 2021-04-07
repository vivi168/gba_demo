#![allow(dead_code)]

// Background

pub const BG_COLOR_16             : u16 = 0x0000;
pub const BG_SCREEN_SIZE_0        : u16 = 0x0000;
pub const BG_PRIORITY_0           : u16 = 0x0000;

pub const BG_PRIORITY_SHIFT       : u16 = 0;
pub const BG_CHAR_BASE_SHIFT      : u16 = 2;
pub const BG_SCREEN_BASE_SHIFT    : u16 = 8;
pub const BG_SCREEN_SIZE_SHIFT    : u16 = 14;

pub const DISP_MODE_0             : u16 = 0x0000;
pub const DISP_OBJ_ON             : u16 = 0x1000;
pub const DISP_BG0_ON             : u16 = 0x0100;

// Interrupt

pub const V_BLANK_INTR_FLAG       : u16 = 0x0001;
pub const STAT_V_BLANK_IF_ENABLE  : u16 = 0x0008;
