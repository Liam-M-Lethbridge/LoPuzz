use std::{iter::Sum, vec};

use rand::seq::SliceRandom;
use tauri::window::Color;
use std::collections::HashSet;
use rand::{Rng, rng};

use crate::{check_grids, game_logic::queens};

pub fn generate_grid(grid_size: u32) -> Vec<u32> {
    // This function generates a grid of queen locations in which no two queens exist in the same row or column and no two queens lie within one square of one another.
    let mut grid = vec![0; (grid_size * grid_size) as usize];
    if add_row(&mut grid, 0, grid_size) {
        return grid;
    } else {
        return vec![0; (grid_size * grid_size) as usize];
    }
}

fn add_row(grid: &mut Vec<u32>, row: u32, size: u32) -> bool {
    // This function recursively adds a new queen along the current row, asserting it does not break the rules.
    if row == size {
        return true;
    }

    let mut cols: Vec<u32> = (0..size).collect();
    cols.shuffle(&mut rng());

    for col in cols {
        let index = row * size + col;
        if is_valid(grid.to_vec(), row, col, size) {
            grid[index as usize] = 1;
            if add_row(grid, row + 1, size) {
                return true;
            }
            grid[index as usize] = 0;
        }
    }
    return false;
}

fn is_valid(grid: Vec<u32>, row: u32, col: u32, size: u32) -> bool {
    // this function checks if the set of queens is a valid solution
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
    // this function locates the nearby uncoloured cells
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
    // this function pushes to the queue so long as the cell has not been seen.
    if seen.insert((row, col)) {
        queue.push((row, col));
    }
}

fn get_queens(grid: Vec<u32>, size: u32) -> Vec<(u32, u32)> {
    // this function finds the locations of queens
    let mut queens = Vec::new();
    for r in 0..size {
        for c in 0..size {
            if grid[(r * size + c) as usize] == 1 {
                queens.push((r, c));
            }
        }
    }

    queens
}



pub fn find_colours(colour_grid: &Vec<u32>, row: u32, col: u32, size: u32) -> Vec<u32>{
    // This function gets the colours of the adjacent cells
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
    
    // let idx = rng().random_range(0..colours.len());
    return colours;

}

pub fn print_grid(grid: Vec<u32>, size: u32){
    // this function prints the grid
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



/// This function colours in the grid recursively, resulting in a single solution.
/// colour_grid: the coloured grid we use.
/// queue: the queue of indicies to colour.
/// seen: the hashset of all of the seen values. Used to avoid duplicates in the queue.
/// size: the size of the grid.
pub fn colour_grid_recursively(colour_grid: &mut Vec<u32>, mut queue: Vec<(u32, u32)>, seen: HashSet<(u32, u32)>, size: u32) -> bool {
    if queue.is_empty() {
        let mut temp_grid = vec![0; (size * size) as usize];
        return count_solutions(colour_grid, 0, size, &mut temp_grid, 0) == 1;
    }

    let idx = rng().random_range(0..queue.len());
    let (row, col) = queue.remove(idx);

    colour_cell_reursively(colour_grid, queue, seen, row, col, size)
}



pub fn colour_cell_reursively(colour_grid: &mut Vec<u32>, queue: Vec<(u32, u32)>, seen: HashSet<(u32, u32)>, row: u32, col: u32, size: u32) -> bool {
    let idx = (row * size + col) as usize;
    // already coloured → move on
    if colour_grid[idx] != 0 {
        return colour_grid_recursively(colour_grid, queue, seen, size);
    }

    let colours = find_colours(colour_grid, row, col, size);
    if colours.is_empty(){
        println!("NO COLOURS");
    }
    for c in colours {
        // clone state for THIS branch
        let mut new_queue = queue.clone();
        let mut new_seen = seen.clone();

        colour_grid[idx] = c;
        colour_cell(colour_grid, &mut new_queue, &mut new_seen, row, col, size);

        

        if colour_grid_recursively(colour_grid, new_queue, new_seen, size) {
            return true;
        }
        

        // backtrack
        colour_grid[idx] = 0;
    }

    false
}


/// this function counts the number of solutions
/// colour_grid: the grid of coloured boxes, a 0 value means the cell is uncoloured
/// current_count: the current number of solutions found (only need two to know it isn't unique)
/// current_grid: the current grid of queen placements
pub fn count_solutions(colour_grid: &Vec<u32>, mut current_count : u32, size: u32, current_grid: &mut Vec<u32>, current_row: u32) -> u32{
    // print_grid(colour_grid.to_vec(), size);
        // println!("counting solutions");

    let colours: Vec<u32> =  (0..size*size).filter(|x: &u32| current_grid[*x as usize] == 1).map(|x| colour_grid[x as usize]).collect::<Vec<u32>>();
    
    if current_row == size {
        // print_grid(current_grid.to_vec(), size);
        if valid_solution(current_grid.to_vec(), size, &colours) {
            // println!("VALID");

            return current_count + 1;
        } else {
            // println!("NOT VALID");
            return current_count;
        }
    }


    let mut queue: Vec<u32> = Vec::new();
    // for each row, find the number of queens we can use
    for j in 0..size{
        if colour_grid[(current_row*size+j) as usize] >0{
            queue.push(current_row*size +j);
        }
    }

    // for each queen in the row, fill out the square with the queen and recurse
    for potential_queen in queue {
        let mut new_grid = current_grid.clone();
        new_grid[potential_queen as usize] = 1;

        current_count = count_solutions(colour_grid,current_count,size, &mut new_grid, current_row + 1);

        
        if current_count > 1 {
            return current_count;
        }
    }

    return current_count;
}


/// this function finds all solutions
/// colour_grid: the grid of coloured boxes, a 0 value means the cell is uncoloured
/// current_count: the current number of solutions found (only need two to know it isn't unique)
/// current_grid: the current grid of queen placements
pub fn find_solutions(
    colour_grid: &Vec<u32>,
    new_solution: &mut Vec<u32>,
    size: u32,
    current_grid: &mut Vec<u32>,
    queens_grid: &Vec<u32>,
    current_row: u32
) -> bool {
    // println!("finding solutions");
    if current_row == size {
        let colours: Vec<u32> = (0..size*size)
            .filter(|x| current_grid[*x as usize] == 1)
            .map(|x| colour_grid[x as usize])
            .collect();

       if valid_solution(current_grid.clone(), size, &colours) {
            if check_grids(current_grid, queens_grid) {
                return false; // same solution, ignore it
            }
            *new_solution = current_grid.clone();
            return true;
        }

        return false;
    }

    // Try all valid positions in this row
    for col in 0..size {
        let idx = (current_row * size + col) as usize;

        if colour_grid[idx] > 0 {
            current_grid[idx] = 1;

            if find_solutions(
                colour_grid,
                new_solution,
                size,
                current_grid,
                queens_grid,
                current_row + 1
            ) {
                return true; // 🚀 propagate success
            }

            current_grid[idx] = 0; // backtrack
        }
    }

    false
}


/// this function checks if a solution is valid
/// grid: the grid of queen placements
/// size: the desired number of queens
/// colours: the list colours attributed to each queen
fn valid_solution(grid: Vec<u32>, size: u32, colours: &Vec<u32>) -> bool{
    // need the right number of queens
    // check rows
    // println!("checking if valid solution");
    for row in 0..size{
        let mut count = 0;
        for col in 0..size {
            if grid[(row * size + col) as usize] == 1 {
                count += 1;
            }
        }
        if count != 1 {
            // println!("row issue. row {}", row);
            return false;
        }

    }
    // println!("No row issues.");

    // check columns
    for col in 0..size{
        let mut count = 0;
        for row in 0..size {
            if grid[(row * size + col) as usize] == 1 {
                count += 1;
            }
        }
        if count != 1 {
            // println!("column issue. Col {}", col);
            return false;
        }

    }
    // println!("No column issues.");

    // check colours
    for cindex1 in 0..colours.len(){
        for cindex2 in 0..cindex1{
            if colours[cindex1] == colours[cindex2]{
                // println!("colour issue. Queens {} and {}",cindex2, cindex2);
                return false;
            }
        }
    }
    // println!("No colour issues.");


    // check proximities and colours
    // first find queen locations:
    let queens = get_queens(grid, size as u32);

    for qi1 in 0..(size as usize){
        for qi2 in (qi1+1)..(size as usize){
            let (r1, c1) = (queens[qi1].0 as i32, queens[qi1].1 as i32);
            let (r2, c2) = (queens[qi2].0 as i32, queens[qi2].1 as i32);

            if (r1 == r2 || r1 == r2 + 1 || r1 == r2 - 1) &&
                (c1 == c2 || c1 == c2 + 1 || c1 == c2 - 1) 
            {
                // println!("proximity issue at ({}, {}) and ({}, {})", r1, c1, r2, c2);
                return false;
            }

        }
    } 
    // println!("No proximity issues.");

    return true;
}