#![allow(dead_code)]

use memory;

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

pub struct JoyPad {
  pub key_held: u16,
  pub key_press: u16
}

impl JoyPad {
  pub unsafe fn read(&mut self) {
    let read_data: u16 = (memory::REG_KEYINPUT as *mut u16).read_volatile() ^ ALL_KEY_MASK;

    self.key_press = read_data & (read_data ^ self.key_held);
    self.key_held = read_data;
  }

  pub fn is_pressed(&self, key: u16) -> bool {
    return self.key_press & key != 0
  }

  pub fn is_held(&self, key: u16) -> bool {
    return self.key_held & key != 0
  }
}
