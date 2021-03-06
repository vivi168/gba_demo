#![allow(dead_code)]

use memory;

const DMA_ENABLE:u32      = 0x80000000;
const DMA_TIMMING_IMM:u32 = 0x00000000;
const DMA_SRC_INC:u32     = 0x00000000;
const DMA_SRC_FIX:u32     = 0x01000000;
const DMA_DEST_INC:u32    = 0x00000000;
const DMA_DEST_FIX:u32    = 0x00400000;
const DMA_16BIT_BUS:u32   = 0x00000000;
const DMA_32BIT_BUS:u32   = 0x04000000;

pub fn dma_copy(srcp: u32, destp: u32, len: u32) {
  let control:u32 = DMA_ENABLE | DMA_TIMMING_IMM | DMA_SRC_INC | DMA_DEST_INC | DMA_32BIT_BUS | len;

  unsafe {
    (memory::REG_DMA3SAD as *mut u32).write_volatile(srcp);
    (memory::REG_DMA3DAD as *mut u32).write_volatile(destp);
    (memory::REG_DMA3CNT as *mut u32).write_volatile(control);
  }
}

#[inline(never)]
pub fn dma_clear(data_ref: &u32, destp: u32, len: u32) {
  let data_ptr = data_ref as *const u32;
  let control:u32 = DMA_ENABLE | DMA_TIMMING_IMM | DMA_SRC_FIX | DMA_DEST_INC | DMA_32BIT_BUS | len;

  unsafe {
    (memory::REG_DMA3SAD as *mut u32).write_volatile(data_ptr as u32);
    (memory::REG_DMA3DAD as *mut u32).write_volatile(destp);
    (memory::REG_DMA3CNT as *mut u32).write_volatile(control);
  }
}

pub fn cpu_copy(srcp: u32, destp: u32, len: u32, bit: u8) {
  let bit_bus: u32 = if bit == 16 { DMA_16BIT_BUS } else { DMA_32BIT_BUS };
  let control:u32 = DMA_SRC_INC | bit_bus | (len / (bit as u32 / 8) & 0x1fffff);

  unsafe { CpuSet(srcp, destp, control); }
}

pub fn cpu_fast_copy(srcp: u32, destp: u32, len: u32) {
  let control:u32 = DMA_SRC_INC | (len / (32 / 8) & 0x1fffff);

  unsafe { CpuFastSet(srcp, destp, control); }
}

extern "C" {
  pub fn CpuSet(srcp: u32, destp: u32, len: u32);
  pub fn CpuFastSet(srcp: u32, destp: u32, len: u32);
}
