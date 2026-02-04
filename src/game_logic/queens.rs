use std::vec;

use rand::seq::SliceRandom;
use rand::{Rng, rng};

pub fn generate_grid(grid_size: u32) -> Vec<u32> {
    let mut grid = vec![0; (grid_size * grid_size) as usize];
    if add_row(&mut grid, 0, grid_size) {
        return grid;
    } else {
        return vec![0; (grid_size * grid_size) as usize];
    }
}

fn add_row(grid: &mut Vec<u32>, row: u32, size: u32) -> bool {
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
    return false;
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
    return true;
}

pub fn create_queens_game(grid_size: u32) -> Vec<u32> {
    let queens_grid = generate_grid(grid_size);

    if queens_grid.iter().sum::<u32>() == 0 {
        return queens_grid;
    }
    let mut queue: Vec<(u32, u32)> = Vec::new();
    let mut colour_grid = vec![0; (grid_size * grid_size) as usize];
    let mut counter = 0;

    // find all queens and give them each a different colour value
    for row in 0..grid_size {
        for col in 0..grid_size{
            if queens_grid[(row*grid_size+col) as usize] == 1 {
                colour_cell(&colour_grid, &mut queue, row, col, grid_size);
                counter += 1;
                colour_grid[(row*grid_size+col) as usize] = counter;
            }
        }
    }
    
    while queue.len() >0 {
        let idx = rng().random_range(0..queue.len());
        let (row, col) = queue.remove(idx);
        let colour = find_colours(&colour_grid, row, col, grid_size);
        // ToDo: check for possible solutions allowed by completing grid
        // ToDo (maybe): assert that the colour with the smallest frequency is picked rather than random
        colour_cell(&colour_grid, &mut queue, row, col, grid_size);
        colour_grid[(row*grid_size+col) as usize] = colour;
    }
    


    return colour_grid;
}


fn colour_cell(colour_grid: &Vec<u32>, queue: &mut Vec<(u32, u32)>, row: u32, col: u32, size: u32) {
    let row = row as i32;
    let col = col as i32;
    let size = size as i32;

    // check for each directly adjacent cell if it is uncoloured, if so push to queue.
    for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let r = row + dr;
        let c = col + dc;

        // careful to check if row or column is out of bounds
        if r >= 0 && r < size && c >= 0 && c < size {
            let idx = (r * size + c) as usize;
            if colour_grid[idx] == 0 {
                queue.push((r as u32, c as u32));
            }
        }
    }
}


fn find_colours(colour_grid: &Vec<u32>, row: u32, col: u32, size: u32) -> u32{
    let row = row as i32;
    let col = col as i32;
    let size = size as i32;
    let mut colours: Vec<u32> = Vec::new();

    // check for each directly adjacent cell. If coloured, add it to queue
    for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let r = row + dr;
        let c = col + dc;

        if r >= 0 && r < size && c >= 0 && c < size {
            let idx = (r * size + c) as usize;
            let c = colour_grid[idx];
            if c != 0 {
                colours.push(c);
            }
        }
    }
    if colours.len() == 0{
        return 0;
    }
    let idx = rng().random_range(0..colours.len());
    return colours[idx];
}
