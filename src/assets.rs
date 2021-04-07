// Assets

pub const BG_PAL: &[u8; 32] = include_bytes!("../assets/bg.pal");
pub const BG_PAL_PTR: *const u8 = BG_PAL.as_ptr();

pub const OBJ_PAL: &[u8; 64] = include_bytes!("../assets/char.pal");
pub const OBJ_PAL_PTR: *const u8 = OBJ_PAL.as_ptr();

pub const BG_TILES: &[u8; 736] = include_bytes!("../assets/bg.tiles");
pub const BG_TILES_PTR: *const u8 = BG_TILES.as_ptr();

pub const OBJ_TILES: &[u8; 2048] = include_bytes!("../assets/char.tiles");
pub const OBJ_TILES_PTR: *const u8 = OBJ_TILES.as_ptr();

pub const BG_SC_DATA: &[u8; 2048] = include_bytes!("../assets/bg.map");
pub const BG_SC_DATA_PTR: *const u8 = BG_SC_DATA.as_ptr();
