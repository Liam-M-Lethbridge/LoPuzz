use rand::seq::SliceRandom;
use rand::rng;

pub fn generate_grid(grid_size: u32) -> Option<Vec<u32>> {
    let mut grid = vec![0; (grid_size * grid_size) as usize];
    if add_row(&mut grid, 0, grid_size) {
        Some(grid)
    } else {
        None
    }
}

pub fn add_row(grid: &mut Vec<u32>, row: u32, size: u32) -> bool {
    if row == size {
        return true;
    }

    let mut cols: Vec<u32> = (0..size).collect();
    cols.shuffle(&mut rng());

    for col in cols {
        let index = row * size + col;
        if is_valid(grid, row, col, size) {
            grid[index as usize] = 1;
            if add_row(grid, row + 1, size) {
                return true;
            }
            grid[index as usize] = 0;
        }
    }
    false
}

fn is_valid(grid: &Vec<u32>, row: u32, col: u32, size: u32) -> bool {
    for r in 0..row {
        for c in 0..size {
            let i = (r * size + c) as usize;
            if grid[i] == 1 {
                // same column anywhere
                if c == col {
                    return false;
                }

                // 3×3 proximity
                if (r as i32 - row as i32).abs() <= 1
                    && (c as i32 - col as i32).abs() <= 1
                {
                    return false;
                }
            }
        }
    }
    true
}


