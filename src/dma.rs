#![allow(dead_code)]
const REG_BASE:u32 = 0x4000_000;
const REG_DMA3:u32 = REG_BASE + 0xd4;
const REG_DMA3SAD:u32 = REG_BASE + 0xd4;
const REG_DMA3DAD:u32 = REG_BASE + 0xd8;
const REG_DMA3CNT:u32 = REG_BASE + 0xdc;

const DMA_ENABLE:u32      = 0x80000000;
const DMA_TIMMING_IMM:u32 = 0x00000000;
const DMA_SRC_INC:u32     = 0x00000000;
const DMA_SRC_FIX:u32     = 0x01000000;
const DMA_DEST_INC:u32    = 0x00000000;
const DMA_DEST_FIX:u32    = 0x00400000;
const DMA_32BIT_BUS:u32   = 0x04000000;

pub fn dma_set(srcp: u32, destp: u32, len: u32) {
  let count:u32 = DMA_ENABLE | DMA_TIMMING_IMM | DMA_SRC_INC | DMA_DEST_INC | DMA_32BIT_BUS | len;

  unsafe {
    (REG_DMA3SAD as *mut u32).write_volatile(srcp);
    (REG_DMA3DAD as *mut u32).write_volatile(destp);
    (REG_DMA3CNT as *mut u32).write_volatile(count);
  }
}

pub fn dma_clear(data: u32, destp: u32, len: u32) {
  let data_ref = &data;
  let data_ptr = data_ref as *const u32;
  let count:u32 = DMA_ENABLE | DMA_TIMMING_IMM | DMA_SRC_FIX | DMA_DEST_INC | DMA_32BIT_BUS | len;

  unsafe {
    (REG_DMA3SAD as *mut u32).write_volatile(data_ptr as u32);
    (REG_DMA3DAD as *mut u32).write_volatile(destp);
    (REG_DMA3CNT as *mut u32).write_volatile(count);
  }
}
