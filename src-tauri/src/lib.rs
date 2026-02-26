// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod game_logic;
use crate::game_logic::numbers::generate_numbers_grid;
use crate::game_logic::numbers::remove_values;
use crate::game_logic::queens::check_clash;
use crate::game_logic::queens::colour_grid_recursively;
use crate::game_logic::queens::generate_grid;
use crate::game_logic::queens::get_neighbours;
use crate::game_logic::utilities::print_grid;

use rand::SeedableRng;
use rand::{rng, seq::SliceRandom};
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            create_queens_game,
            compare_solutions_queens,
            create_numbers_game,
            compare_solutions_numbers
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
/// This function creates a queens game, giving a coloured grid with a unique single solution.
/// - grid_size: the size of the grid to generate.
fn create_queens_game(grid_size: u32) -> Vec<u32> {
    loop {
        println!("new grid");
        let queens_grid = generate_grid(grid_size);

        if queens_grid.iter().sum::<u32>() == 0 {
            return queens_grid;
        }
        // queue is used to keep track of what cells to colour in next
        let mut queue: VecDeque<(u32, u32)> = VecDeque::new();
        // seen is used to keep track of what cells have already been seen so we don't have duplicates in the queue
        let mut seen: HashSet<(u32, u32)> = HashSet::new();
        // colour_grid is the grid of colours. If a cell is uncoloured it has a value 0. Otherwise its number relates to its colour
        let mut colour_grid = vec![0; (grid_size * grid_size) as usize];
        let mut counter: usize = 0;
        // working solution is the current solution of queens. Used for keeping track of any invalid solutions.
        let mut working_solution: Vec<(u32, u32, u32)> = Vec::new();

        // colour counter keeps track of the number of cells of each colour
        let mut colour_counter: HashMap<u32, u32> = HashMap::new();

        let mut impossible_routes = 0;
        // for randomising colours
        let mut colours: Vec<u32> = (1..grid_size + 1).collect();
        colours.shuffle(&mut rng());
        // find all queens and give them each a different colour value
        for row in 0..grid_size {
            for col in 0..grid_size {
                if queens_grid[(row * grid_size + col) as usize] == 1 {
                    get_neighbours(
                        &colour_grid,
                        &mut queue,
                        &mut seen,
                        row,
                        col,
                        grid_size,
                        &colour_counter,
                    );
                    colour_counter.insert(colours[counter], 1);
                    colour_grid[(row * grid_size + col) as usize] = colours[counter];
                    working_solution.push((row, col, colours[counter]));
                    counter += 1;
                }
            }
        }

        // we start colouring recursively. Using the queue containing the neighbours of the queens
        if colour_grid_recursively(
            &mut colour_grid,
            queue,
            seen,
            grid_size,
            &mut working_solution,
            &mut colour_counter,
            &mut impossible_routes,
        ) {
            return colour_grid;
        }
    }
}

#[tauri::command]
/// This function checks if the solution sent back is valid for the queens game.
/// - colour_grid: the grid of the colours.
/// - solution: the indices of each queen.
/// - size: the size of the grid.
fn compare_solutions_queens(colour_grid: Vec<u32>, solution: Vec<u32>, size: u32) -> bool {
    for i in 1..size as usize {
        let cell1 = (
            solution[i] / size,
            solution[i] % size,
            colour_grid[solution[i] as usize],
        );
        for j in 0..i {
            let cell2 = (
                solution[j] / size,
                solution[j] % size,
                colour_grid[solution[j] as usize],
            );
            if check_clash(cell1, cell2) {
                return false;
            }
        }
    }
    return true;
}

#[tauri::command]
/// This function checks if the solution sent bak is valid.
/// - grid_size: the size of the grid.
/// - difficulty: the difficulty setting.
fn create_numbers_game(grid_size: u32, difficulty: u32) -> Vec<u32> {
    let grid = remove_values(&generate_numbers_grid(grid_size), difficulty, grid_size);

    return grid;
}

#[tauri::command]
/// This function checks if the solution sent back is valid for the numbers game.
/// - number_grid: the grid of the colours.
/// - size: the size of the grid.
fn compare_solutions_numbers(numbers_grid: Vec<u32>, size: u32) -> bool {
    // check each row
    print_grid(numbers_grid.clone(), size);
    for row in 0..size {
        let mut seen: HashSet<u32> = HashSet::new();
        for i in 0..size {
            if !seen.insert(numbers_grid[(row * size + i) as usize]) {
                println!("row");
                return false;
            }
        }
    }

    // check each column
    for col in 0..size {
        let mut seen: HashSet<u32> = HashSet::new();
        for i in 0..size {
            if !seen.insert(numbers_grid[(i * size + col) as usize]) {
                println!("col");
                return false;
            }
        }
    }

    // check each diagonal
    let mut seen: HashMap<i32, HashSet<u32>> = HashMap::new();
    for i in -(size as i32)..(size as i32) {
        seen.insert(i, HashSet::new());
    }
    for row in 0..(size as i32) {
        for col in 0..(size as i32) {
            let diff = row - col;
            if let Some(thing) = seen.get_mut(&diff) {
                if !thing.insert(numbers_grid[(row * (size as i32) + col) as usize]) {
                    println!("diag");
                    return false;
                }
            }
        }
    }

    // check each other diagonal
    let mut seen: HashMap<i32, HashSet<u32>> = HashMap::new();
    for i in -(size as i32)..(size as i32) {
        seen.insert(i, HashSet::new());
    }
    for row in 0..(size as i32) {
        for col in 0..(size as i32) {
            let diff = row + col;
            if let Some(thing) = seen.get_mut(&diff) {
                if !thing.insert(numbers_grid[(row * (size as i32) + col) as usize]) {
                    println!("other diag");
                    return false;
                }
            }
        }
    }
    println!("YEAH");
    return true;
}

//  1   2   3   4   5
//  1   2   3   4   5
//  1   2   3   4   5
//  1   2   3   4   5
//  1   2   3   4   5
