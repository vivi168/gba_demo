#![no_std]
#![no_main]

mod dma;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
  loop {}
}

const pal: &[u8; 256] = include_bytes!("../palette.bin");
const pal_ptr: *const u8 = pal.as_ptr();

const REG_BASE:u32 = 0x4000_000;
const REG_DISPSTAT:u32 = REG_BASE + 0x4;
const REG_IE:u32 = REG_BASE + 0x200;
const REG_IME:u32 = REG_BASE + 0x208;

const VRAM:u32 = 0x6000_000;
const PALETTE:u32 = 0x5000_000;
const PALETTE_OAM:u32 = PALETTE + 0x200;

fn copy_16(data: &[u8], dest: u32) {
  let mut buffer: u16 = 0;

  for (i, &item) in data.iter().enumerate() {
    if i % 2 == 1 {
      buffer |= (item as u16) << 8;
      unsafe {
        (dest as *mut u16).offset(((i-1) / 2) as isize).write_volatile(buffer);
      }
    }
    buffer = item as u16;
  }
}

extern "C" {
  fn vblankWait();
}

#[no_mangle]
extern "C" fn VBlankInterrupt() {
  dma::dma_clear(32, VRAM, 160*240);

  unsafe { (0x3007ff8 as *mut u16).write_volatile(1); }
}

#[no_mangle]
extern "C" fn AgbMain() {
  unsafe {
    (REG_IME as *mut u16).write_volatile(1);
    (REG_IE as *mut u16).write_volatile(0x0001);
    (REG_DISPSTAT as *mut u16).write_volatile(0x0008);
  }

  dma::dma_set(pal_ptr as u32, PALETTE_OAM, (pal.len() / 4) as u32);

  unsafe {
    (REG_BASE as *mut u16).write_volatile(3 | 0x400);
  }

  let mut i:isize = 0;

  loop {
    unsafe {
      vblankWait();

      (VRAM as *mut u16).offset(240 * 160 / 2 + i).write_volatile(31);
    }

    i += 1;
    if i > 240 {
      i = 0
    }
  }
}
