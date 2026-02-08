use std::vec;

use rand::seq::SliceRandom;
use std::collections::HashSet;
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


pub fn colour_cell(colour_grid: &Vec<u32>, queue: &mut Vec<(u32, u32)>, seen: &mut HashSet<(u32, u32)>, row: u32, col: u32, size: u32) {
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
                push_if_not_seen(queue, seen, r as u32, c as u32)
                
            }
        }
    }
}

fn push_if_not_seen(queue: &mut Vec<(u32, u32)>, seen: &mut HashSet<(u32, u32)>, row: u32, col: u32){
    if seen.insert((row, col)) {
        queue.push((row, col));
    }
}

pub fn find_colours(colour_grid: &Vec<u32>, row: u32, col: u32, size: u32) -> u32{
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

pub fn print_grid(grid: &Vec<u32>, size: u32){
    println!();
    for r in 0..size {
        for c in 0..size {
            print!(
                "{} ",
                grid[(r * size + c) as usize]
            );
        }
        println!();
    }
}

pub fn check_grids(grid1: Vec<u32>, grid2: Vec<u32>) -> bool{

    if grid1.len() != grid2.len(){
        return false;
    }

    for i in 0..grid1.len(){
        if grid1[i] != grid2[i]{
            return false;
        }
    }
    return true;
}