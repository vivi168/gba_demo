#![allow(dead_code)]

use memory;

pub const OAM_SIZE: u32 = 128 * 2; // a single OamData is 2 * u32 long

#[derive(Copy, Clone)]
pub struct OamData {
  attribute_0: u16,
  attribute_1: u16,
  attribute_2: u16,
  affine_param: u16
}

impl OamData {
  pub const fn default() -> OamData {
    return OamData {
      attribute_0: 0,
      attribute_1: 0,
      attribute_2: 0,
      affine_param: 0
    };
  }

  // attribute 0
  pub fn set_y_coord(&mut self, data: u16) {
    self.attribute_0 &= !0xff;
    self.attribute_0 |= (data & 0xff);
  }
  pub fn set_affine_mode(&mut self, data: u16) {
    self.attribute_0 &= !(3 << 8);
    self.attribute_0 |= ((data & 3) << 8);
  }
  pub fn set_obj_mode(&mut self, data: u16) {
    self.attribute_0 &= !(3 << 10);
    self.attribute_0 |= ((data & 3) << 10);
  }
  pub fn set_mosaic(&mut self, data: u16) {
    self.attribute_0 &= !(1 << 12);
    self.attribute_0 |= ((data & 1) << 12);
  }
  pub fn set_color_mode(&mut self, data: u16) {
    self.attribute_0 &= !(1 << 13);
    self.attribute_0 |= ((data & 1) << 13);
  }
  pub fn set_obj_shape(&mut self, data: u16) {
    self.attribute_0 &= !(3 << 14);
    self.attribute_0 |= ((data & 3) << 14);
  }

  pub fn get_y_coord(&self) -> u16 {
    return &self.attribute_0 & 0xff;
  }
  pub fn get_affine_mode(&self) -> u16 {
    return (&self.attribute_0 & (3 << 8)) >> 8;
  }
  pub fn get_obj_mode(&self) -> u16 {
    return (&self.attribute_0 & (3 << 10)) >> 10;
  }
  pub fn get_mosaic(&self) -> u16 {
    return (&self.attribute_0 & (1 << 12)) >> 12;
  }
  pub fn get_color_mode(&self) -> u16 {
    return (&self.attribute_0 & (1 << 13)) >> 13;
  }
  pub fn get_obj_shape(&self) -> u16 {
    return (&self.attribute_0 & (3 << 14)) >> 14;
  }

  // attribute 1
  pub fn set_x_coord(&mut self, data: u16) {
    self.attribute_1 &= !0x1ff;
    self.attribute_1 |= (data & 0x1ff);
  }
  pub fn set_affine_param_no(&mut self, data: u16) {
    self.attribute_1 &= !(7 << 9);
    self.attribute_1 |= ((data & 7) << 9);
  }
  pub fn set_h_flip(&mut self, data: u16) {
    self.attribute_1 &= !(1 << 12);
    self.attribute_1 |= ((data & 1) << 12);
  }
  pub fn set_v_flip(&mut self, data: u16) {
    self.attribute_1 &= !(1 << 13);
    self.attribute_1 |= ((data & 1) << 13);
  }
  pub fn set_obj_size(&mut self, data: u16) {
    self.attribute_1 &= !(3 << 14);
    self.attribute_1 |= ((data & 3) << 14);
  }

  pub fn get_x_coord(&self) -> u16 {
    return &self.attribute_1 & 0x1ff;
  }
  pub fn get_affine_param_no(&self) -> u16 {
    return (&self.attribute_1 & (7 << 9)) >> 9;
  }
  pub fn get_h_flip(&self) -> u16 {
    return (&self.attribute_1 & (1 << 12)) >> 12;
  }
  pub fn get_v_flip(&self) -> u16 {
    return (&self.attribute_1 & (1 << 13)) >> 13;
  }
  pub fn get_obj_size(&self) -> u16 {
    return (&self.attribute_1 & (3 << 14)) >> 14;
  }

  // attribute 2
  pub fn set_char_no(&mut self, data: u16) {
    self.attribute_2 &= !0x3ff;
    self.attribute_2 |= (data & 0x3ff);
  }
  pub fn set_priority(&mut self, data: u16) {
    self.attribute_2 &= !(3 << 10);
    self.attribute_2 |= ((data & 3) << 10);
  }
  pub fn set_palette_no(&mut self, data: u16) {
    self.attribute_2 &= !(0xf << 12);
    self.attribute_2 |= ((data & 0xf) << 12);
  }

  pub fn get_char_no(&self) -> u16 {
    return &self.attribute_2 & 0x3ff;
  }
  pub fn get_priority(&self) -> u16 {
    return (&self.attribute_2 & (3 << 10)) >> 10;
  }
  pub fn get_palette_no(&self) -> u16 {
    return (&self.attribute_2 & (0xf << 12)) >> 12;
  }
}
