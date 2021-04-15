#![no_std]
#![no_main]

use core::mem::size_of;

mod assets;
mod memory;
mod define;
mod input;
mod dma;
mod oam;
mod random;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
  loop {}
}

#[link_section = ".exram"]
static mut bg_sc_shadow: [u8; 2048] = [0; 2048];
#[link_section = ".exram"]
static mut oam_shadow: [oam::OamData; oam::OAM_SIZE] = [oam::OamData::default(); oam::OAM_SIZE];

static mut frame_counter : u32 = 0;
static mut timer         : u32 = 0;
static mut bg_scroll_x   : u8 = 0;
static mut bg_scroll_y   : u8 = 0;
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
    dma::dma_copy((bg_sc_shadow.as_ptr() as *const u8) as u32, memory::VRAM, (assets::BG_SC_DATA.len() / 4) as u32);
    dma::dma_copy((oam_shadow.as_ptr() as *const u8) as u32, memory::OAM, (size_of::<oam::OamData>() * oam::OAM_SIZE / 4) as u32);

    frame_counter += 1;
    if frame_counter % 60 == 0 {
      timer += 1;
    }

    (memory::REG_BG0HOFS as *mut u16).write_volatile(bg_scroll_x as u16);
    (memory::REG_BG0VOFS as *mut u16).write_volatile(bg_scroll_y as u16);

    (memory::INTR_CHECK_BUF as *mut u16).write_volatile(1); //  = V_BLANK_INTR_FLAG
  }
}

#[no_mangle]
fn DummyInterrupt() { }


fn init_oam() {
  unsafe {
    for obj in oam_shadow.iter_mut() {
      obj.set_x_coord(256);
      obj.set_obj_size(1);
    }

    oam_shadow[0].set_x_coord(0);
    oam_shadow[0].set_y_coord(0);
    oam_shadow[0].set_obj_size(1);

    dma::dma_copy((oam_shadow.as_ptr() as *const u8) as u32, memory::OAM, (size_of::<oam::OamData>() * oam::OAM_SIZE / 4) as u32);
  }
}

const VEL: i16 = 1;
const MAP_W: i16 = 32;
const MAP_H: i16 = 32;
const SCREEN_W: i16 = 240;
const SCREEN_H: i16 = 160;
const CELL_SIZE: i16 = 16;

struct Camera {
  x: i16,
  y: i16
}

impl Camera {
  fn center_on(&mut self, actor: &Actor) {
    let px = actor.x * CELL_SIZE;
    let py = actor.y * CELL_SIZE;

    self.x = px - (SCREEN_W / 2 - CELL_SIZE / 2);
    self.y = py - (SCREEN_H / 2 - CELL_SIZE);

    // keep camera in bound
    if self.x < 0 {
      self.x = 0;
    } else if (self.x + SCREEN_W) > (MAP_W * CELL_SIZE) {
      self.x = (MAP_W * CELL_SIZE) - SCREEN_W;
    }
    if self.y < 0 {
      self.y = 0;
    } else if (self.y + SCREEN_H) > (MAP_H * CELL_SIZE) {
      self.y = (MAP_H * CELL_SIZE) - SCREEN_H;
    }

    unsafe {
      bg_scroll_x = self.x as u8;
      bg_scroll_y = self.y as u8;
    }
  }
}

struct Actor {
  x: i16,
  y: i16,
  sprite: u16
}

fn update_oam(actor: &Actor, camera: &Camera, idx: usize) {
  let mut screen_x = actor.x * CELL_SIZE - camera.x;
  let screen_y = actor.y * CELL_SIZE - camera.y;

  // keep sprite off screen
  if screen_x < 0 || screen_x > SCREEN_W - CELL_SIZE {
    screen_x = 256;
  }
  if screen_y < 0 || screen_y > SCREEN_H - CELL_SIZE {
    screen_x = 256;
  }

  unsafe {
    oam_shadow[idx].set_x_coord(screen_x as u16);
    oam_shadow[idx].set_y_coord(screen_y as u16);
    oam_shadow[idx].set_char_no(actor.sprite);
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
  dma::dma_copy(assets::BG_PAL_PTR as u32, memory::PALETTE, (assets::BG_PAL.len() / 4) as u32);
  dma::dma_copy(assets::OBJ_PAL_PTR as u32, memory::PALETTE_OAM, (assets::OBJ_PAL.len() / 4) as u32);
  dma::dma_copy(assets::BG_TILES_PTR as u32, memory::BG_CH_BLOCK_1, (assets::BG_TILES.len() / 4) as u32);
  dma::dma_copy(assets::OBJ_TILES_PTR as u32, memory::OBJ_MODE0_VRAM, (assets::OBJ_TILES.len() / 4) as u32);

  // init oam
  init_oam();

  let mut rng = random::Random::default();
  let mut player = Actor { x: 18, y: 25, sprite: 0 };
  let mut knight = Actor { x: 2, y: 3, sprite: 2 };
  let mut camera = Camera { x: 0, y: 0 };
  camera.center_on(&player);
  update_oam(&player, &camera, 0);
  update_oam(&knight, &camera, 1);

  unsafe {
    dma::dma_copy(assets::BG_SC_DATA_PTR as u32, (bg_sc_shadow.as_ptr() as *const u8) as u32, (assets::BG_SC_DATA.len() / 4) as u32);
    dma::dma_copy((bg_sc_shadow.as_ptr() as *const u8) as u32, memory::VRAM, (assets::BG_SC_DATA.len() / 4) as u32);

    // set registers
    (memory::INTR_VECTOR_BUF as *mut u32).write_volatile(IntrMainBuff_ptr as u32);
    (memory::REG_IME as *mut u16).write_volatile(1);
    (memory::REG_IE as *mut u16).write_volatile(define::V_BLANK_INTR_FLAG);
    (memory::REG_DISPSTAT as *mut u16).write_volatile(define::STAT_V_BLANK_IF_ENABLE);

    (memory::REG_BG0CNT as *mut u16).write_volatile(define::BG_COLOR_16 | define::BG_SCREEN_SIZE_0 | define::BG_PRIORITY_0 |
                                                    0 << define::BG_SCREEN_BASE_SHIFT | 1 << define::BG_CHAR_BASE_SHIFT);

    // turn screen on
    (memory::REG_DISPCNT as *mut u16).write_volatile(define::DISP_MODE_0 | define::DISP_OBJ_ON | define::DISP_BG0_ON);
  }

  let mut joypad = input::JoyPad { key_held: 0, key_press: 0 };

  // main loop
  loop {
    unsafe {
      VBlankWait();

      joypad.read();

      let prev_x = player.x;
      let prev_y = player.y;

      if joypad.is_pressed(input::R_KEY) {
        player.x += VEL;
      } else if joypad.is_pressed(input::L_KEY) {
        player.x -= VEL;
      } else if joypad.is_pressed(input::U_KEY) {
        player.y -= VEL;
      } else if joypad.is_pressed(input::D_KEY) {
        player.y += VEL;
      }

      // keep player in bound
      if player.x < 0 || player.x > MAP_W - 1 {
        player.x = prev_x;
      }
      if player.y < 0 || player.y > MAP_H - 1 {
        player.y = prev_y;
      }

      camera.center_on(&player);

      // update actors animation every other frame
      if frame_counter % 30 == 0 {
        player.sprite ^= 16;
        knight.sprite ^= 16;
      }

      update_oam(&player, &camera, 0);
      update_oam(&knight, &camera, 1);
    }
  }
}
