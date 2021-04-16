use random;

const MAP_WIDTH   : u16 = 48;
const MAP_HEIGHT  : u16 = 48;
const ZONE_WIDTH  : u16 = MAP_WIDTH / 3;
const ZONE_HEIGHT : u16 = MAP_HEIGHT / 3;

const X_OFFSET   : u16 = 3;
const Y_OFFSET   : u16 = 3;
const MIN_WIDTH  : u16 = 5;
const MIN_HEIGHT : u16 = 4;

#[derive(Copy, Clone)]
enum Tile {
  Empty,
  HorWall,
  VertWall,
  TopLeWall,
  TopRiWall,
  BotLeWall,
  BotRiWall,
  FloorTile,
  CorridorTile,
  StairCase
}

#[derive(Copy, Clone)]
struct Rect {
  x1 : u16,
  x2 : u16,
  y1 : u16,
  y2 : u16
}

impl Rect {
  pub const fn default() -> Rect {
    return Rect {
      x1: 0,
      y1: 0,
      x2: 0,
      y2: 0
    };
  }

  pub const fn new(x : u16, y : u16, w : u16, h : u16) -> Rect {
    return Rect {
      x1: x,
      y1: y,
      x2: x + w,
      y2: y + h
    };
  }

  fn width(&self) -> u16 {
    return &self.x2 - &self.x1;
  }

  fn height(&self) -> u16 {
    return &self.y2 - &self.y1;
  }

  fn center_x(&self) -> u16 {
    return (&self.x1 + &self.x2) / 2;
  }

  fn center_y(&self) -> u16 {
    return (&self.y1 + &self.y2) / 2;
  }
}

pub struct Dungeon {
  tile_map: [Tile; (MAP_WIDTH * MAP_HEIGHT) as usize],
  rooms: [Rect; 9],
  corridors: [i8; 9]
}

impl Dungeon {
  pub const fn default() -> Dungeon {
    return Dungeon {
      tile_map: [Tile::Empty; (MAP_WIDTH * MAP_HEIGHT) as usize],
      rooms: [Rect::default(); 9],
      corridors: [-1; 9]
    };
  }

  pub fn make_dungeon(&mut self, rng : &mut random::Random) {
    self.make_rooms(rng);
    self.make_corridors(rng);

    self.dig_rooms();
    self.dig_corridors();
  }

  fn make_rooms(&mut self, rng : &mut random::Random) {
    for y in 0..3 {
      for x in 0..3 {
        let x1 = (x as u16) * ZONE_WIDTH;
        let y1 = (y as u16) * ZONE_HEIGHT;
        let x2 = x1 + ZONE_WIDTH;
        let y2 = y1 + ZONE_HEIGHT;

        self.rooms[3 * y + x].x1 = rng.rand(x1 + X_OFFSET, (x1 + x2) / 2);
        self.rooms[3 * y + x].x2 = rng.rand(self.rooms[3 * y + x].x1 + MIN_WIDTH, x2 - X_OFFSET);
        self.rooms[3 * y + x].y1 = rng.rand(y1 + Y_OFFSET, (y1 + y2) / 2);
        self.rooms[3 * y + x].y2 = rng.rand(self.rooms[3 * y + x].y1 + MIN_HEIGHT, y2 - Y_OFFSET);
      }
    }
  }

  fn make_corridors(&mut self, rng : &mut random::Random) {
    const CONNECTIONS : [[i8; 4]; 9] = [
      [1, 3, -1, -1],
      [0, 2, 4, -1],
      [1, 5, -1, -1],
      [0, 4, 6, -1],
      [1, 3, 5, 7],
      [2, 4, 8, -1],
      [3, 7, -1, -1],
      [4, 6, 8, -1],
      [5, 7, -1, -1]];

    for (i, conn) in CONNECTIONS.iter().enumerate() {
      loop {
        let target_idx = rng.rand(0, 3) as usize;
        if conn[target_idx] < 0 { continue }

        let target = conn[target_idx];
        self.corridors[i] = target;

        if i == 8 {
          if self.corridors[5] == 8 && self.corridors[7] == 8 {
            self.corridors[i] = -1;
          }
          else if self.corridors[5] == 8 {
            self.corridors[i] = 7;
          }
          else if self.corridors[7] == 8 {
            self.corridors[i] = 5;
          }
          break;
        }

        if self.corridors[target as usize] != (i as i8) { break }
      }
    }
  }

  fn dig_rooms(&mut self) {
    for room in self.rooms.iter() {
      for y in 0..room.height() {
        for x in 0..room.width() {
          let map_x = x + room.x1;
          let map_y = y + room.y1;
          let idx = map_x + MAP_WIDTH * map_y;

          if x == 0 && y == 0 {
            self.tile_map[idx as usize] = Tile::TopLeWall;
          } else if x == room.width() - 1 && y == 0 {
            self.tile_map[idx as usize] = Tile::TopRiWall;
          } else if x == 0 && y == room.height() - 1 {
            self.tile_map[idx as usize] = Tile::BotLeWall;
          } else if x == room.width() - 1 && y == room.height() - 1 {
            self.tile_map[idx as usize] = Tile::BotRiWall;
          } else if x == 0 || x == room.width() - 1 {
            self.tile_map[idx as usize] = Tile::VertWall;
          } else if y == 0 || y == room.height() - 1 {
            self.tile_map[idx as usize] = Tile::HorWall;
          } else {
            self.tile_map[idx as usize] = Tile::FloorTile;
          }
        }
      }
    }
  }

  fn dig_corridors(&mut self) {
    let corridors = self.corridors;
    for (a, &b) in corridors.iter().enumerate() {
      if b == -1 { continue }

      let org : usize;
      let dest : usize;

      if a < (b as usize) {
        org = a;
        dest = b as usize;
      } else {
        org = b as usize;
        dest = a;
      }

      let room_org = self.rooms[org];
      let room_dest = self.rooms[dest];

      if same_col(org as u8, dest as u8) {
        let ox = (room_org.x1 + room_org.x2) / 2;
        let oy = room_org.y2 - 1;

        let dx = (room_dest.x1 + room_dest.x2) / 2;
        let dy = room_dest.y1 + 1;

        let py = (oy + dy) / 2;

        self.dig_vert_line(ox, oy, py);
        if ox < dx { self.dig_hor_line(ox, py, dx) }
        else { self.dig_hor_line(dx, py, ox+1) }
        self.dig_vert_line(dx, py, dy);
      } else {
        let ox = room_org.x2 - 1;
        let oy = (room_org.y1 + room_org.y2) / 2;

        let dx = room_dest.x1 + 1;
        let dy = (room_dest.y1 + room_dest.y2) / 2;

        let px = (ox + dx) / 2;

        self.dig_hor_line(ox, oy, px);
        if oy < dy { self.dig_vert_line(px, oy, dy) }
        else { self.dig_vert_line(px, dy, oy+1) }
        self.dig_hor_line(px, dy, dx);
      }
    }
  }

  fn dig_vert_line(&mut self, ox : u16, oy : u16, dy : u16) {
    for y in 0..(dy - oy) {
      let map_y = y + oy;
      let idx = ox + MAP_WIDTH * map_y;
      self.tile_map[idx as usize] = Tile::CorridorTile;
    }
  }

  fn dig_hor_line(&mut self, ox : u16, oy : u16, dx : u16) {
    for x in 0..(dx - ox) {
      let map_x = x + ox;
      let idx = map_x + MAP_WIDTH * oy;
      self.tile_map[idx as usize] = Tile::CorridorTile;
    }
  }
}

fn same_col(a: u8, b : u8) -> bool {
  return col(a) == col(b);
}

fn col(a : u8) -> u8 {
  return a % 3;
}
