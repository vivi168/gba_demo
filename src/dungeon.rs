use random;

const MAP_WIDTH   : u16 = 96;
const MAP_HEIGHT  : u16 = 48;
const ZONE_WIDTH  : u16 = MAP_WIDTH / 3;
const ZONE_HEIGHT : u16 = MAP_HEIGHT / 3;

const X_OFFSET   : u16 = 3;
const Y_OFFSET   : u16 = 3;
const MIN_WIDTH  : u16 = 4;
const MIN_HEIGHT : u16 = 3;

#[derive(Copy, Clone)]
enum Tile {
  Empty,
  RoomFloor,
  Corridor
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

        if self.corridors[target as usize] != 0 { break }
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

          self.tile_map[idx as usize] = Tile::RoomFloor;
        }
      }
    }
  }

  fn dig_corridors(&mut self) {

  }

  fn dig_vert_line(&mut self) {

  }

  fn dig_hor_line(&mut self) {

  }
}
