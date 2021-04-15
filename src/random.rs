pub struct Random {
  next : u32
}

impl Random {
  pub const fn default() -> Random {
    return Random { next: 1 };
  }

  pub fn seed(&mut self, seed : u32) -> u32 {
    let prev = self.next;
    self.next = seed;

    return prev;
  }

  pub fn rand16(&mut self) -> u16 {
    self.next = self.next.wrapping_mul(1103515245).wrapping_add(12345);

    return (self.next >> 16) as u16;
  }

  pub fn rand(&mut self, min: u16, max: u16) -> u16 {
    return self.rand16() % (max - min + 1) + min;
  }
}
