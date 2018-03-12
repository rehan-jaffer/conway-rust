const GRID_WIDTH: i32 = 64;
const GRID_SIZE: i32 = GRID_WIDTH * GRID_WIDTH;

struct Grid {
  cells: Vec<u8>
}

impl Grid {
  fn init(&mut self) {
    for i in 0..GRID_SIZE {
      self.cells.push(0);
    }
  }
  fn display(self) {
    for x in 0..GRID_WIDTH {
      for y in 0..GRID_WIDTH {
        println!("{}", self.cells[(GRID_WIDTH*y + x) as usize]);
      }
    }
  }
}

fn main() {

  let mut grid = Grid { cells: vec!(0) };
  grid.init();
  grid.display();

}
