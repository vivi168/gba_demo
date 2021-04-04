#![no_std]
#![no_main]

mod memory;
mod dma;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
  loop {}
}

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

static mut frame_counter : u32 = 0;
static mut timer         : u32 = 0;
static mut key_press     : u16 = 0;
static mut key_held      : u16 = 0;
static mut bg_scroll_x   : i16 = 0;
static mut bg_scroll_y   : i16 = 0;
static mut IntrMainBuff  : [u32; 16] = [0; 16];

#[no_mangle]
static InterruptTable: [fn(); 2] = [VBlankInterrupt, DummyInterrupt];

extern "C" {
  fn VBlankWait();
  fn InterruptMain();
}

#[no_mangle]
fn VBlankInterrupt() {
  unsafe {
    dma::dma_copy((bg_sc_shadow.as_ptr() as *const u8) as u32, memory::VRAM, (BG_SC_DATA.len() / 4) as u32);

    frame_counter += 1;
    if (frame_counter % 60 == 0) {
      timer += 1;
    }

    (memory::REG_BG0HOFS as *mut u16).write_volatile(bg_scroll_x as u16);
    (memory::REG_BG0VOFS as *mut u16).write_volatile(bg_scroll_y as u16);

    (memory::INTR_CHECK_BUF as *mut u16).write_volatile(1); //  = V_BLANK_INTR_FLAG
  }
}

#[no_mangle]
fn DummyInterrupt() { }

fn key_read() {
  unsafe {
    let read_data: u16 = (memory::REG_KEYINPUT as *mut u16).read_volatile() ^ memory::ALL_KEY_MASK;
    key_press = read_data & (read_data ^ key_held);
    key_held = read_data;
  }
}

#[no_mangle]
extern "C" fn AgbMain() {
  // clear RAM
  let clear: u32 = 0;
  dma::dma_clear(&clear, memory::EX_WRAM, memory::EX_WRAM_SIZE);
  dma::dma_clear(&clear, memory::CPU_WRAM, memory::CPU_WRAM_SIZE - 0x200);

  // Init interrupt
  let IntrMainBuff_ptr;

  unsafe {
    IntrMainBuff_ptr = &IntrMainBuff as *const u32;
    let InterruptMain_ptr = InterruptMain as *const u32;

    dma::dma_copy(InterruptMain_ptr as u32, IntrMainBuff_ptr as u32, IntrMainBuff.len() as u32);
  }

  // copy data
  dma::dma_copy(BG_PAL_PTR as u32, memory::PALETTE, (BG_PAL.len() / 4) as u32);
  dma::dma_copy(OBJ_PAL_PTR as u32, memory::PALETTE_OAM, (OBJ_PAL.len() / 4) as u32);
  dma::dma_copy(BG_TILES_PTR as u32, memory::VRAM + 0x8000, (BG_TILES.len() / 4) as u32);
  dma::dma_copy(OBJ_TILES_PTR as u32, memory::VRAM + 0x10000, (OBJ_TILES.len() / 4) as u32);

  unsafe {
    dma::dma_copy(BG_SC_DATA_PTR as u32, (bg_sc_shadow.as_ptr() as *const u8) as u32, (BG_SC_DATA.len() / 4) as u32);
    dma::dma_copy((bg_sc_shadow.as_ptr() as *const u8) as u32, memory::VRAM, (BG_SC_DATA.len() / 4) as u32);

    // set registers
    (memory::INTR_VECTOR_BUF as *mut u32).write_volatile(IntrMainBuff_ptr as u32);
    (memory::REG_IME as *mut u16).write_volatile(1);
    (memory::REG_IE as *mut u16).write_volatile(0x0001); // = V_BLANK_INTR_FLAG
    (memory::REG_DISPSTAT as *mut u16).write_volatile(0x0008); // = STAT_V_BLANK_IF_ENABLE

    // (BG_COLOR_16 | BG_SCREEN_SIZE_0 | BG_PRIORITY_0 | 0 << BG_SCREEN_BASE_SHIFT | 2 << BG_CHAR_BASE_SHIFT)
    (memory::REG_BG0CNT as *mut u16).write_volatile(0x0000 | 0x0000 | 0x0000 | 0 << 8 | 2 << 2); //

    // turn screen on
    (memory::REG_DISPCNT as *mut u16).write_volatile(0x0000 | 0x1000 | 0x0100); // (DISP_MODE_0 | DISP_OBJ_ON | DISP_BG0_ON)
  }

  // main loop
  loop {
    unsafe {
      VBlankWait();

      key_read();

      if key_held & memory::R_KEY != 0 {
        bg_scroll_x += 1;
      } else if key_held & memory::L_KEY != 0 {
        bg_scroll_x -= 1;
      } else if key_held & memory::U_KEY != 0 {
        bg_scroll_y -= 1;
      } else if key_held & memory::D_KEY != 0 {
        bg_scroll_y += 1;
      }
      // todo
      // move sprite
    }
  }
}
