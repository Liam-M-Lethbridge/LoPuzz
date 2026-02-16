// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod game_logic;
use crate::game_logic::queens;
use crate::game_logic::queens::check_clash;
use crate::game_logic::queens::colour_grid_recursively;
use crate::game_logic::queens::generate_grid;
use crate::game_logic::queens::get_neighbours;

use rand::{rng, seq::SliceRandom};
use serde::ser::Impossible;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![create_queens_game, compare_solutions])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
/// This function creates a queens game, giving a coloured grid with a unique single solution.
/// - grid_size: the size of the grid to generate.
fn create_queens_game(grid_size: u32) -> Vec<u32> {
    while true {
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
                    get_neighbours(&colour_grid, &mut queue, &mut seen, row, col, grid_size, &colour_counter);
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
            &mut impossible_routes
        ){
            return colour_grid;
        }
    }
    return Vec::new();
}

#[tauri::command]
/// This function checks if the solution sent bak is valid.
/// - colour_grid: the grid of the colours.
/// - solution: the indices of each queen
fn compare_solutions(colour_grid: Vec<u32>, solution:Vec<u32>, size: u32) -> bool{
    for i in 1..size as usize{
        let cell1 = (solution[i]/size, solution[i]%size, colour_grid[solution[i] as usize]);
        for j in 0..i{
            let cell2 = (solution[j]/size, solution[j]%size, colour_grid[solution[j] as usize]);
            if check_clash(cell1, cell2){
                return  false;
            }
        }
    }
    return true;
}


#[tauri::command]
fn create_shapes_game(grid_size: u32){
    
}