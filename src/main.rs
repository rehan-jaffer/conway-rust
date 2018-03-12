extern crate rand;
use rand::Rng;

const GRID_WIDTH: i32 = 64;
const GRID_SIZE: i32 = GRID_WIDTH * GRID_WIDTH;

fn translate(x: i32, y: i32) -> usize {
    let mut x1: i32 = 0;
    let mut y1: i32 = 0;

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

fn init_grid() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut grid = Vec::new();
    for i in 0..GRID_SIZE {
        let c: u8 = rng.gen::<bool>() as u8;
        grid.push(c);
    }
    return grid;
}

fn display_grid(grid: &Vec<u8>) {
    for x in 0..GRID_WIDTH {
        for y in 0..GRID_WIDTH {
            print!("{}", grid[((GRID_WIDTH * y) + x) as usize]);
        }
        println!("");
    }
}

fn update_grid(grid: &Vec<u8>) -> Vec<u8> {
    let mut next_state: Vec<u8> = Vec::new();

    for x in 0..GRID_WIDTH {
        for y in 0..GRID_WIDTH {
            let mut score: u8 = 0;

            for i in -1..1 {
                for j in -1..1 {
                    score += grid[translate(x + i, y + j)];
                }
            }

            if grid[translate(x, y)] == 1 {
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

    return next_state;
}

fn main() {
    let grid = init_grid();
    display_grid(&grid);
    let new_grid = update_grid(&grid);
    display_grid(&new_grid);
}
