// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod game_logic;
use crate::game_logic::queens::colour_grid_recursively;
use crate::game_logic::queens::find_solutions;
use crate::game_logic::queens::generate_grid;
use crate::game_logic::queens::colour_cell;
use crate::game_logic::queens::count_solutions;
use crate::game_logic::queens::find_colours;
use crate::game_logic::queens::print_grid;

use std::collections::{HashSet, VecDeque};
use rand::seq::index;
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
    let mut seen: HashSet<(u32, u32)> = HashSet::new();
    let mut colour_grid = vec![0; (grid_size * grid_size) as usize];
    let mut counter = 0;

    // find all queens and give them each a different colour value
    for row in 0..grid_size {
        for col in 0..grid_size{
            if queens_grid[(row*grid_size+col) as usize] == 1 {
                colour_cell(&colour_grid, &mut queue, &mut seen, row, col, grid_size);
                counter += 1;
                colour_grid[(row*grid_size+col) as usize] = counter;
            }
        }
    }
    
    while queue.len() >0 {
        let idx = rng().random_range(0..queue.len());
        let (row, col) = queue.remove(idx);
        let colours = find_colours(&colour_grid, row, col, grid_size);
        let idx = rng().random_range(0..colours.len());

        // ToDo: check for possible solutions allowed by completing grid
        // ToDo (maybe): assert that the colour with the smallest frequency is picked rather than random
        colour_cell(&colour_grid, &mut queue, &mut seen, row, col, grid_size);
        colour_grid[(row*grid_size+col) as usize] = colours[idx];
    }
    // println!("{}", colour_grid_recursively(&mut colour_grid, queue, seen, grid_size));
    
    // we have our grid, we want to check if the solutions are unique
    let mut temp_grid = vec![0; (grid_size * grid_size) as usize];

    // println!("{}", count_solutions(&colour_grid, 0, grid_size, &mut temp_grid, 0));
    let mut all_solutions: Vec<Vec<u32>> = Vec::new();

    while find_solutions(&colour_grid, &mut all_solutions, grid_size, &mut temp_grid, 0, 0) > 1{
        let mut conflict_grid: Vec<u32> = vec![0; (grid_size * grid_size) as usize];
        let mut conflicts: Vec<u32> = Vec::new();
        // combine these solutions into a grid of conflicts 
        
        for i in 0..(grid_size*grid_size) as usize{
            if queens_grid[i] == 0{
                for grid in &all_solutions{
                    if grid[i] == 1{
                        conflict_grid[i] = 1;
                        conflicts.push(i as u32);
                        break;
                    }
                }
            }
        }
        print_grid(conflict_grid, grid_size);

        // now that we have the conflicts, we can ammend them by changing their colour

        let mut colour_queens_index: Vec<(u32, u32, u32)> = Vec::new();
        for row in 0..grid_size{
            for col in 0..grid_size{
                let idx = (row*grid_size+col) as usize;
                if queens_grid[idx] == 1{
                    colour_queens_index.push((colour_grid[idx], row, col));
                }
            }
        }
        
        for conflict in conflicts{
            change_colour(&mut colour_grid, conflict, grid_size, &queens_grid);
            fix_invalid_colours(&mut colour_grid, &colour_queens_index, grid_size);
            if count_solutions(&colour_grid, 0, grid_size, &mut temp_grid, 0) == 1{
                println!("nice job.");
                return colour_grid;
            }
        }

        // if we have successfully changed colour then we can try again
        // however, if we have not, then we have to widen the search until we can change colour

        return colour_grid;
    }

    return colour_grid;
    

}

/// This function fills the grid appropriately
fn fill_grid(
    colour_grid: &mut Vec<u32>,
    size: u32,
    conflict_grid: &Vec<Vec<u32>>,
) {
    loop {
        let mut queue: VecDeque<u32> = VecDeque::new();
        let mut visited: HashSet<u32> = HashSet::new();
        let mut possible_colours: Vec<u32> = Vec::new();
        let mut region: Vec<u32> = Vec::new();

        // Find first uncoloured cell
        let start = match colour_grid.iter().position(|&c| c == 0) {
            Some(i) => i as u32,
            None => return, 
        };

        queue.push_back(start);
        visited.insert(start);
        region.push(start);

        // Flood-fill zero region
        while let Some(current) = queue.pop_front() {
            let r = (current / size) as i32;
            let c = (current % size) as i32;

            for (dr, dc) in [(-1,0), (1,0), (0,-1), (0,1)] {
                let nr = r + dr;
                let nc = c + dc;

                if nr < 0 || nr >= size as i32 || nc < 0 || nc >= size as i32 {
                    continue;
                }

                let next = (nr as u32) * size + (nc as u32);
                let idx = next as usize;

                if visited.contains(&next) {
                    continue;
                }

                visited.insert(next);

                if colour_grid[idx] == 0 {
                    queue.push_back(next);
                    region.push(next);
                } else if !possible_colours.contains(&colour_grid[idx]) {
                    possible_colours.push(colour_grid[idx]);
                }
            }
        }

        // Try to colour the whole region
        let mut applied = false;

        for colour in possible_colours {
            if region.iter().all(|&i| !conflict_grid[i as usize].contains(&colour)) {
                for &i in &region {
                    colour_grid[i as usize] = colour;
                }
                applied = true;
                break;
            }
        }

        if !applied {
            return;
        }
    }
}



/// This function changes colour to remove possible clashes

fn change_colour(
    colour_grid: &mut Vec<u32>,
    start: u32,
    size: u32,
    actual_solution: &Vec<u32>,
    banned_colours: &Vec<u32>
) {
    let size_i32 = size as i32;


    let mut queue = VecDeque::new();
    let mut parent: HashMap<u32, u32> = HashMap::new();

    queue.push_back(start);
    parent.insert(start, start);

    let mut found_colour: Option<(u32, u32)> = None;

    while let Some(current) = queue.pop_front() {
        let r = (current / size) as i32;
        let c = (current % size) as i32;

        let idx = current as usize;

        // Found a cell (not the start) which does not contain any of the banned colours
        if current != start && !banned_colours.iter().any(|&i| i== colour_grid[idx]) {
            found_colour = Some((current, colour_grid[idx]));
            break;
        }

        for (dr, dc) in [(-1,0), (1,0), (0,-1), (0,1)] {
            let nr = r + dr;
            let nc = c + dc;

            if nr < 0 || nr >= size_i32 || nc < 0 || nc >= size_i32 {
                continue;
            }

            let next = (nr as u32) * size + (nc as u32);
            let next_idx = next as usize;

            // Blocked by a queen
            if actual_solution[next_idx] == 1 {
                continue;
            }

            if parent.contains_key(&next) {
                continue;
            }

            parent.insert(next, current);
            queue.push_back(next);
        }
    }

    // No reachable colour → nothing to propagate
    let (end, colour) = match found_colour {
        Some(v) => v,
        None => return,
    };

    // Walk backwards and fill colour
    let mut current = end;
    while current != start {
        let prev = parent[&current];
        colour_grid[prev as usize] = colour;
        current = prev;
    }
}



 fn check_grids(grid1: &[u32], grid2: &[u32]) -> bool{
    // this function checks if two grids are the same.
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


/// Fix invalid colours in the grid:
/// colour_grid: the current colouring (0 = uncoloured)
/// colour_queen_indices: the queen positions grid for each colour (colour, row, col)
/// n_colours: number of colours used
pub fn fix_invalid_colours(colour_grid: &mut Vec<u32>, colour_queen_indices: &Vec<(u32, u32, u32)>, size: u32){
    // println!("fixing colours");

    let size_i32 = size as i32;

    for colour in 1..=size {
        let mut cells: Vec<(u32, u32)> = Vec::new();

        // Find all cells with this colour
        let mut qrow = 0;
        let mut qcol = 0;
        for q in colour_queen_indices{      
            if q.0 == colour{
                qrow = q.1 as u32;
                qcol = q.2;
                cells.push((qrow, qcol));
            }
        }

        for r in 0..size {
            for c in 0..size {
                if qrow == r && qcol == c{
                    continue;
                }
                let idx = (r * size + c) as usize;
                if colour_grid[idx] == colour {
                    cells.push((r, c));
                }
            }
        }

        if cells.is_empty() {
            continue; // nothing to do
        }

        // Flood-fill from the first cell
        let mut seen: HashSet<(u32, u32)> = HashSet::new();
        let mut queue: VecDeque<(u32, u32)> = VecDeque::new();
        queue.push_back(cells[0]);
        seen.insert(cells[0]);

        while let Some((r, c)) = queue.pop_front() {
            for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr >= 0 && nr < size_i32 && nc >= 0 && nc < size_i32 {
                    let nidx = (nr as u32 * size + nc as u32) as usize;
                    if colour_grid[nidx] == colour && seen.insert((nr as u32, nc as u32)) {
                        queue.push_back((nr as u32, nc as u32));
                    }
                }
            }
        }

        // If there are cells not reached by flood-fill, reset them
        for &(r, c) in &cells {
            if !seen.contains(&(r, c)) {
                let idx = (r * size + c) as usize;
                colour_grid[idx] = 0;
            }
        }
    }
}