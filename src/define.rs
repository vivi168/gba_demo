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

// Input

pub const BUTTON_MASK             : u16 = 0x030f;
pub const PLUS_KEY_MASK           : u16 = 0x00f0;
pub const ALL_KEY_MASK            : u16 = 0x03ff;

pub const A_BUTTON                : u16 = 0x0001;
pub const B_BUTTON                : u16 = 0x0002;
pub const SELECT_BUTTON           : u16 = 0x0004;
pub const START_BUTTON            : u16 = 0x0008;
pub const R_KEY                   : u16 = 0x0010;
pub const L_KEY                   : u16 = 0x0020;
pub const U_KEY                   : u16 = 0x0040;
pub const D_KEY                   : u16 = 0x0080;
pub const R_BUTTON                : u16 = 0x0100;
pub const L_BUTTON                : u16 = 0x0200;
