use std::collections::HashSet;
use std::vec;
use rand::{fill, rng};
use rand::seq::{SliceRandom, index};
use serde::de::value;

pub fn generate_shapes_grid(size: u32) -> Vec<u32>{
    let mut grid = vec![0; (size * size) as usize];
    if fill_grid(&mut grid, size as usize, 0){
        return grid;
    }
    return  vec![0; (size * size) as usize];
}

/// This function fills the grid one at a time recursively.
/// - grid: the current grid state.
/// i size: the size of the grid.
fn fill_grid(grid: &mut Vec<u32>, size: usize, index:usize) -> bool{
    let mut possible_values: Vec<u32> = valid_placements(grid, index/size, index%size, size).into_iter().collect();
    possible_values.shuffle(&mut rng());
    for value in possible_values{
        grid[index] = value;
        if fill_grid(grid, size, index+1){
            return  true;
        }
        grid[index] = 0;
    }
    return false;
}

/// This function finds all the valid placements for the current index for the current grid.
/// - grid: the current grid state.
/// - row: the row of the current square.
/// - col: the column of the currrent square.
/// - size: the size of the grid.
fn valid_placements(grid: & Vec<u32>, row: usize, col: usize, size: usize) -> HashSet<u32>{
    let mut values: HashSet<u32> = HashSet::new();
    for value in 1..(size+1)as u32{
        values.insert(value);
    }
    // on same row
    for index in 0..col{
        values.remove(&grid[row*size+index]);
    }
    // on same column
    for index in 0..row{
        values.remove(&grid[index*size+col]);
    }
    // on diagonal
    for index in 1..size{
        if col < index  || row < index {
            break;
        }
        values.remove(&grid[(row-index)*size+(col-index)]);
    }
        for index in 1..size{
        if col+index >size || row+index > size{
            break;
        }
        values.remove(&grid[(row+index)*size+(col+index)]);
    }
    return values;
}