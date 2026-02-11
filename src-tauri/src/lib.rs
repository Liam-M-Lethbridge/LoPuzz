// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod game_logic;
use crate::game_logic::queens::colour_grid_recursively;
use crate::game_logic::queens::generate_grid;
use crate::game_logic::queens::colour_cell;

use std::collections::{HashSet, VecDeque, HashMap};
use rand::seq::index;
use rand::{Rng, rng, seq::SliceRandom};



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![create_queens_game])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


#[tauri::command]
fn create_queens_game(grid_size: u32) -> Vec<u32> {
    let mut queens_grid = generate_grid(grid_size);

    if queens_grid.iter().sum::<u32>() == 0 {
        return queens_grid;
    }
    let mut queue: Vec<(u32, u32)> = Vec::new();
    let mut seen: HashSet<(u32, u32)> = HashSet::new();
    let mut colour_grid = vec![0; (grid_size * grid_size) as usize];
    let mut counter = 0;
    let mut working_solution: Vec<(u32, u32, u32)> = Vec::new();

    // find all queens and give them each a different colour value
    for row in 0..grid_size {
        for col in 0..grid_size{
            if queens_grid[(row*grid_size+col) as usize] == 1 {
                colour_cell(&colour_grid, &mut queue, &mut seen, row, col, grid_size);
                counter += 1;
                colour_grid[(row*grid_size+col) as usize] = counter;
                working_solution.push((row, col, counter));
            }
        }
    }

    
    // we start colouring recursively. Using the queue containing the neighbours of the queens
    colour_grid_recursively(&mut colour_grid, queue, seen, grid_size, &mut working_solution);
    // println!("{}", colour_grid_recursively(&mut colour_grid, queue, seen, grid_size));
    
    return colour_grid;
}

    