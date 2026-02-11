use std::{collections::{HashMap, VecDeque}, hash::Hash, iter::Sum, vec};

use rand::{random, random_range, seq::SliceRandom};
use serde::ser::Impossible;
use tauri::window::Color;
use std::collections::HashSet;
use rand::{Rng, rng};

use crate::{game_logic::queens};

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
pub fn colour_grid_recursively(colour_grid: &mut Vec<u32>, mut queue: Vec<(u32, u32)>, mut seen: HashSet<(u32, u32)>, size: u32, working_solution: &mut Vec<(u32,u32,u32)>) -> bool {
    // For each new colour, we need to QUICKLY check if it adds a new solution.
    // to quickly check, force the cell to have queen. From working_solution, construct subset which only contain possible queens.
    // initialise construction and push cell (row, column, colour) to it
    // From this subset perform check_valid_addition(building_solution, rows, possible_queens, size):
        // if rows.len().== size{
            // return false;
        // }

        
        // May not even have to be recursive?
        
    // to restrict subset, take an iterable() and filter according to !check_clash(|cell|, new_queen)
    // 
    // because we are always checking with the first queen being filled in as the new cell, any solution found using this queen is guaranteed to not be the true solution
    if queue.is_empty(){
        return true;
    }

    
    while let Some((row, col)) = queue.pop(){
        // randomly skip a cell
        if random_range(0..20/(size))==0{
            queue.push((row, col));
            continue;
        }
        let mut colours: Vec<u32> = find_colours(colour_grid, row, col, size);
        colours.shuffle(& mut rng());
        for colour in colours{
            // if we find a valid layout of colours with the current (row, col, colour) trio, we want to return true

            // obtain queens by filtering working_solution into only those that don't clash with the current trio

            let mut candidates = Vec::new();

            for r in 0..size {
                for c in 0..size {
                    let colour = colour_grid[(r*size + c) as usize];
                    if colour != 0 {
                        candidates.push((r, c, colour));
                    }
                }
            }


            // obtain rows as a list of rows
            // let mut rows:VecDeque<u32> = VecDeque::new();
            // let mut building_solution:VecDeque<(u32, u32, u32)> = VecDeque::new();
            // building_solution.push_back((row, col, colour));

            // let temp_bool = false;
            // if check_valid_addition(&mut building_solution, &mut possible_queens, size, &mut rows){

            let mut placed = vec![(row, col, colour)];
            let mut solution_count = 0;

            count_solutions(0, size , &candidates,& mut placed, &mut solution_count);
            // if solution is 1 then we 
            if solution_count == 0{
                colour_grid[(row*size+col) as usize] = colour;
                working_solution.push((row, col, colour));
                colour_cell(colour_grid, &mut queue, &mut seen, row, col, size);
                if colour_grid_recursively(colour_grid, queue.clone(), seen.clone(), size, working_solution){
                    return true;
                }
            }
            // if we don't find a valid layout with this colour, we try other colours
        }
        // if we don't find a valid layout with this cell and the cell's neighbours all exist then we have found a dead end, and must go back
        if all_neighbours_found(&colour_grid, row, col, size){
            return  false;
        }
        //otherwise we push it to the back the the queue to be checked later
        queue.push((row, col));
    }
    // if we get to this point, we have checked everything in the queue and assumedly have coloured in the grid appropriately
    return true;
}


fn count_solutions(
    row: u32,
    size: u32,
    candidates: &Vec<(u32,u32,u32)>,
    placed: &mut Vec<(u32,u32,u32)>,
    solution_count: &mut u32,
) {
    if *solution_count > 1 {
        return;
    }

    // Skip rows already filled
    let mut current_row = row;
    while current_row < size &&
        placed.iter().any(|&(r,_,_)| r == current_row)
    {
        current_row += 1;
    }

    // All rows filled → found solution
    if current_row == size {
        *solution_count += 1;
        return;
    }

    for &(r, c, clr) in candidates.iter().filter(|&&(r,_,_)| r == current_row) {
        if placed.iter().all(|&q| !check_clash(q, (r,c,clr))) {

            placed.push((r,c,clr));
            count_solutions(current_row + 1, size, candidates, placed, solution_count);
            placed.pop();
        }
    }
}



/// This function checks if two cells clash with one another
/// cell1: the first cell (row, col, colour)
/// cell2: the second cell (row, col, colour)
fn check_clash(cell1: (u32, u32, u32), cell2: (u32, u32, u32)) -> bool{
    let (row1, col1, colour1) = cell1;
    let (row2, col2, colour2) = cell2;

    // same row
    if row1 == row2{
        return true;
    }

    // same column
    if col1 == col2{
        return true;
    }

    // same colour
    if colour1 == colour2{
        return true;
    }

    // too close
    if (row1 == row2+1 || row2 == row1+1) && (col1 == col2+1 || col2 == col1+1){
        return true;
    }  

    return false;
}




/// This function checks if all possible neighbours of a cell have been found
fn all_neighbours_found(colour_grid: & Vec<u32>, row: u32, col: u32, size: u32) -> bool{
    let size_i32 = size as i32;
    for (dr, dc) in [(-1,0), (1,0), (0,-1), (0,1)] {
        let nr = row as i32+ dr;
        let nc = col as i32 + dc;

        // skip any out-of-bounds neighbours        
        if nr < 0 || nr >= size_i32 || nc < 0 || nc >= size_i32 {
            continue;
        }
        // if neighbour not visited, return false
        if colour_grid[(nr*size_i32 + nc) as usize] == 0{
            return false;
        }
    }
    // if all visited, return true
    return true;
}




/// This function checks if the current building solution can result in a solution. Returns false if solution is found, true otherwise
fn check_valid_addition(building_solution: &mut VecDeque<(u32,u32,u32)>, possible_queens: &mut Vec<(u32,u32,u32)>, size: u32, rows: &mut VecDeque<u32>) -> bool{
// queue rows by size, removing any rows which are already in building_solution 
    // to speed up, check first if any rows or columns have 0 possible queens, if so then return true.
    // then check if any rows or columns have only one possible queen. Add that to the building_solution and add the row to rows. Filter the possible things
    
    // if we have obtained a valid solution, return false
    if rows.len() == size as usize{
        return false;
    }

    // initialise row_queue as all the rows not in rows
    let mut row_queue: HashMap<u32, Vec<(u32, u32)>> = HashMap::new();
    for row in 0..size{
        if !rows.iter().any(|&i| i == row){
            row_queue.insert(row, Vec::new());
        }
    }
    // insert into rows
    for (r, c, clr) in &mut *possible_queens{
        // we know that any row is empty if it's already in rows
        if let Some(val) = row_queue.get_mut(r) { val.push((*c, *clr)); };
    }

    // now check if any row is empty 
    for row in 0..size{
        if row_queue[&row].is_empty(){
            // solution is impossible
            // impossible_solution = true;
            return true;
        }
        
    }


    for row in row_queue.keys(){
        rows.push_back(*row);

        for (col, clr) in row_queue[row].clone() {
            // restrict subset further and recurse;
            building_solution.push_back((*row, col, clr));

            if !check_valid_addition(building_solution,  &mut possible_queens.iter().copied().filter(|&cell| !check_clash(cell, (*row, col, clr))).collect::<Vec<(u32,u32,u32)>>(), size,  rows){
                return false;
            }
            // if impossible_solution{
            //     return true;
            // }
            // if no queen solution found and no, try next in queue
            building_solution.pop_back();
        }
        rows.pop_back();

    }
    // don't even think it's possible to reach this?
    return true;
}

// /// this function counts the number of solutions
// /// colour_grid: the grid of coloured boxes, a 0 value means the cell is uncoloured
// /// current_count: the current number of solutions found (only need two to know it isn't unique)
// /// current_grid: the current grid of queen placements
// pub fn count_solutions(colour_grid: &Vec<u32>, mut current_count : u32, size: u32, current_grid: &mut Vec<u32>, current_row: u32) -> u32{
//     // print_grid(colour_grid.to_vec(), size);
//     let colours: Vec<u32> =  (0..size*size).filter(|x: &u32| current_grid[*x as usize] == 1).map(|x| colour_grid[x as usize]).collect::<Vec<u32>>();
    
//     if current_row == size {
//         // print_grid(current_grid.to_vec(), size);
//         if valid_solution(current_grid.to_vec(), size, &colours) {
//             // println!("VALID");

//             return current_count + 1;
//         } else {
//             // println!("NOT VALID");
//             return current_count;
//         }
//     }


//     let mut queue: Vec<u32> = Vec::new();
//     // for each row, find the number of queens we can use
//     for j in 0..size{
//         if colour_grid[(current_row*size+j) as usize] >0{
//             queue.push(current_row*size +j);
//         }
//     }

//     // for each queen in the row, fill out the square with the queen and recurse
//     for potential_queen in queue {
//         let mut new_grid = current_grid.clone();
//         new_grid[potential_queen as usize] = 1;

//         current_count = count_solutions(colour_grid,current_count,size, &mut new_grid, current_row + 1);

        
//         if current_count > 1 {
//             return current_count;
//         }
//     }

//     return current_count;
// }


/// this function finds all solutions
/// colour_grid: the grid of coloured boxes, a 0 value means the cell is uncoloured
/// current_count: the current number of solutions found (only need two to know it isn't unique)
/// current_grid: the current grid of queen placements
pub fn find_solutions(colour_grid: &Vec<u32>,  all_solutions: &mut Vec<Vec<u32>>, size: u32, current_grid: &mut Vec<u32>, current_row: u32) -> u32{
    // print_grid(colour_grid.to_vec(), size);
    let colours: Vec<u32> =  (0..size*size).filter(|x: &u32| current_grid[*x as usize] == 1).map(|x| colour_grid[x as usize]).collect::<Vec<u32>>();
    let mut n_solutions = 0;
    if current_row == size {
        // print_grid(current_grid.to_vec(), size);
        if valid_solution(current_grid.to_vec(), size, &colours) {
            // println!("VALID");
            all_solutions.push(current_grid.clone());
            return 1;
        } else {
            // println!("NOT VALID");
            return 0;
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

        n_solutions += find_solutions(colour_grid,all_solutions,size, &mut new_grid, current_row + 1);
    }
    return n_solutions;
}

/// this function checks if a solution is valid
/// grid: the grid of queen placements
/// size: the desired number of queens
/// colours: the list colours attributed to each queen
fn valid_solution(grid: Vec<u32>, size: u32, colours: &Vec<u32>) -> bool{
    // need the right number of queens
    // check rows
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