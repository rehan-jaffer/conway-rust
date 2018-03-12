extern crate rand;
use rand::Rng;

const GRID_WIDTH: i32 = 128;
const GRID_SIZE: i32 = GRID_WIDTH * GRID_WIDTH;

struct Grid {
  cells: Vec<u8>,
  history: Vec<Vec<u8>>,
  current: i32
}

fn translate(x:i32, y:i32) -> usize {

    let mut x1:i32 = 0;
    let mut y1:i32 = 0;

    if x < 0 {
      x1 = GRID_WIDTH - x;
    } else {
      x1 = x;
    } 
    if y < 0 {
      y1 = GRID_WIDTH - y;
    } else {
      y1 = y;
    }
    return (((y1 * GRID_WIDTH) + x1) % GRID_SIZE) as usize;
}

impl Grid {
  fn init(&mut self) {
    let mut rng = rand::thread_rng();
    for i in 0..GRID_SIZE {
      let c:u8 = rng.gen::<bool>() as u8;
      self.cells.push(c);
    }
    self.history.push(self.cells.clone());
  }
  fn display(&self) {
    for x in 0..GRID_WIDTH {
      for y in 0..GRID_WIDTH {
        print!("{}", self.history[self.current as usize][(GRID_WIDTH*y + x) as usize]);
      }
      println!("");
    }
  }
  fn update(&mut self) {

      let mut next_state: Vec<u8> = Vec::new();

      for x in 0..GRID_WIDTH {
        for y in 0..GRID_WIDTH {
          let mut score:u8 = 0;

          for i in -1..1 {
            for j in -1..1 {
              score += self.cells[translate(x+i,y+j)]; 
            }
          }

          if self.cells[translate(x, y)] == 1 {
            if (score > 3 || score < 2) {
              next_state.push(0);
            } else {
              next_state.push(1);
            }
          } else {
            if (score == 3) {
              next_state.push(1);
            } else {
              next_state.push(0);
            }
          }
        }

      }

      self.history.push(next_state);
      self.current += 1;
  }
}

fn main() {

  let mut grid = Grid { cells: vec!(0), history: vec!(vec!(0)), current: 0 };
  grid.init();
  for x in 0..100 {
    grid.update();
    grid.display();
  }

}
