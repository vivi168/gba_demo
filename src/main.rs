#![no_std]
#![no_main]

mod dma;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
  loop {}
}

// Memory MAP

const REG_BASE:u32 = 0x4000_000;
const REG_DISPCNT:u32 = REG_BASE;
const REG_DISPSTAT:u32 = REG_BASE + 0x4;
const REG_BG0CNT:u32 = REG_BASE + 0x8;
const REG_IE:u32 = REG_BASE + 0x200;
const REG_IME:u32 = REG_BASE + 0x208;
const VRAM:u32 = 0x6000_000;
const PALETTE:u32 = 0x5000_000;
const PALETTE_OAM:u32 = PALETTE + 0x200;

// Constants

const BG_PAL: &[u8; 32] = include_bytes!("../assets/bg.pal");
const BG_PAL_PTR: *const u8 = BG_PAL.as_ptr();

const OBJ_PAL: &[u8; 64] = include_bytes!("../assets/char.pal");
const OBJ_PAL_PTR: *const u8 = OBJ_PAL.as_ptr();

const BG_TILES: &[u8; 736] = include_bytes!("../assets/bg.tiles");
const BG_TILES_PTR: *const u8 = BG_TILES.as_ptr();

const OBJ_TILES: &[u8; 2048] = include_bytes!("../assets/char.tiles");
const OBJ_TILES_PTR: *const u8 = OBJ_TILES.as_ptr();

const BG_SC_DATA: &[u8; 2048] = include_bytes!("../assets/bg.map");
const BG_SC_DATA_PTR: *const u8 = BG_SC_DATA.as_ptr();

#[link_section = ".exram"]
static mut bg_sc_shadow: [u8; 2048] = [0; 2048];

extern "C" {
  fn VBlankWait();
  fn InterruptMain();
}

#[no_mangle]
static InterruptTable: [fn(); 2] = [VBlankInterrupt, DummyInterrupt];

#[no_mangle]
fn VBlankInterrupt() {
  unsafe {
    dma::dma_copy((bg_sc_shadow.as_ptr() as *const u8) as u32, VRAM, (BG_SC_DATA.len() / 4) as u32);

    (0x3007ff8 as *mut u16).write_volatile(1); // INTR_CHECK_BUF = V_BLANK_INTR_FLAG
  }
}

#[no_mangle]
fn DummyInterrupt() { }

#[no_mangle]
extern "C" fn AgbMain() {
  // clear RAM
  let clear: u32 = 0;
  dma::dma_clear(&clear, 0x02000000, 0x40000); // EX_WRAM, EX_WRAM_SIZE
  dma::dma_clear(&clear, 0x03000000, 0x8000 - 0x200); // CPU_WRAM, CPU_WRAM_SIZE - 0x200,32

  // Init interrupt
  let IntrMainBuff: [u32; 0x200/4] = [0; 0x200/4];
  let IntrMainBuff_ref = &IntrMainBuff;
  let IntrMainBuff_ptr = IntrMainBuff_ref as *const u32;
  let InterruptMain_ptr = InterruptMain as *const u32;

  dma::dma_copy(InterruptMain_ptr as u32, IntrMainBuff_ptr as u32, IntrMainBuff.len() as u32);

  // copy data
  dma::dma_copy(BG_PAL_PTR as u32, PALETTE, (BG_PAL.len() / 4) as u32);
  dma::dma_copy(OBJ_PAL_PTR as u32, PALETTE_OAM, (OBJ_PAL.len() / 4) as u32);
  dma::dma_copy(BG_TILES_PTR as u32, VRAM + 0x8000, (BG_TILES.len() / 4) as u32);
  dma::dma_copy(OBJ_TILES_PTR as u32, VRAM + 0x10000, (OBJ_TILES.len() / 4) as u32); // OBJ_MODE0_VRAM

  unsafe {
    dma::dma_copy(BG_SC_DATA_PTR as u32, (bg_sc_shadow.as_ptr() as *const u8) as u32, (BG_SC_DATA.len() / 4) as u32);
    dma::dma_copy((bg_sc_shadow.as_ptr() as *const u8) as u32, VRAM, (BG_SC_DATA.len() / 4) as u32);
  }

  // set registers
  unsafe {
    (0x3007ffc as *mut u32).write_volatile(IntrMainBuff_ptr as u32); // INTR_VECTOR_BUF
    (REG_IME as *mut u16).write_volatile(1);
    (REG_IE as *mut u16).write_volatile(0x0001); // = V_BLANK_INTR_FLAG
    (REG_DISPSTAT as *mut u16).write_volatile(0x0008); // = STAT_V_BLANK_IF_ENABLE

    // (BG_COLOR_16 | BG_SCREEN_SIZE_0 | BG_PRIORITY_0 | 0 << BG_SCREEN_BASE_SHIFT | 2 << BG_CHAR_BASE_SHIFT)
    (REG_BG0CNT as *mut u16).write_volatile(0x0000 | 0x0000 | 0x0000 | 0 << 8 | 2 << 2); //

    // turn screen on
    (REG_DISPCNT as *mut u16).write_volatile(0x0000 | 0x1000 | 0x0100); // (DISP_MODE_0 | DISP_OBJ_ON | DISP_BG0_ON)
  }

  // main loop
  loop {
    unsafe {
      VBlankWait();

      // todo
      // read input
      // move sprite
    }
  }
}
