// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod game_logic;
use crate::game_logic::queens::generate_grid;
use crate::game_logic::queens::find_colours;
use crate::game_logic::queens::colour_cell;
use rand::{Rng, rng};



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
