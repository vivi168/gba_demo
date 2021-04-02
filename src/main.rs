#![no_std]
#![no_main]

mod dma;

#[link_section = ".exram"]
static mut ARRAYTEST: [u16; 10] = [0; 10];

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
  loop {}
}

const PAL: &[u8; 256] = include_bytes!("../palette.bin");
const PAL_PTR: *const u8 = PAL.as_ptr();

const REG_BASE:u32 = 0x4000_000;
const REG_DISPSTAT:u32 = REG_BASE + 0x4;
const REG_IE:u32 = REG_BASE + 0x200;
const REG_IME:u32 = REG_BASE + 0x208;

const VRAM:u32 = 0x6000_000;
const PALETTE:u32 = 0x5000_000;
const PALETTE_OAM:u32 = PALETTE + 0x200;

#[allow(dead_code)]
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

extern "C" { fn VBlankWait(); }
extern "C" { fn InterruptMain(); }

#[no_mangle]
static InterruptTable: [fn(); 2] = [VBlankInterrupt, DummyInterrupt];

#[no_mangle]
fn VBlankInterrupt() {
  let data: u32 = 0xdeaddead;
  dma::dma_clear(&data, VRAM, 160*240);

  unsafe { (0x3007ff8 as *mut u16).write_volatile(1); }
}
#[no_mangle]
fn DummyInterrupt() { }

#[no_mangle]
extern "C" fn AgbMain() {
  let clear: u32 = 0;
  dma::dma_clear(&clear, 0x02000000, 0x40000);

  let IntrMainBuff: [u32; 0x200/4] = [0; 0x200/4];
  let IntrMainBuff_ref = &IntrMainBuff;
  let IntrMainBuff_ptr = IntrMainBuff_ref as *const u32;
  let InterruptMain_ptr = InterruptMain as *const u32;

  dma::dma_copy(InterruptMain_ptr as u32, IntrMainBuff_ptr as u32, IntrMainBuff.len() as u32);

  unsafe {
    (0x3007ffc as *mut u32).write_volatile(IntrMainBuff_ptr as u32);
    (REG_IME as *mut u16).write_volatile(1);
    (REG_IE as *mut u16).write_volatile(0x0001);
    (REG_DISPSTAT as *mut u16).write_volatile(0x0008);
  }

  dma::dma_copy(PAL_PTR as u32, PALETTE_OAM, (PAL.len() / 4) as u32);

  unsafe {
    (REG_BASE as *mut u16).write_volatile(3 | 0x400);
  }

  let mut i:isize = 0;

  loop {
    unsafe {
      VBlankWait();

      (VRAM as *mut u16).offset(240 * 160 / 2 + i).write_volatile(ARRAYTEST[0]);
      (VRAM as *mut u16).offset(240 * 160 / 2 + i + 1).write_volatile(ARRAYTEST[4]);
      (VRAM as *mut u16).offset(240 * 160 / 2 + i + 2).write_volatile(ARRAYTEST[9]);

      ARRAYTEST[0] += 0xdead;
      ARRAYTEST[4] += 1;
      ARRAYTEST[9] += 20;
    }

    i += 1;
    if i > 240 {
      i = 0
    }
  }
}
