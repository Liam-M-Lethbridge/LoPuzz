use rand::seq::SliceRandom;
use rand::{random, random_range, rng};
use std::collections::{HashMap, HashSet};
use std::vec;

/// Thisnfunction generates a numbers grid.
/// - size: the size of the grid generated.
pub fn generate_numbers_grid(size: u32) -> Vec<u32> {
    let mut grid = vec![0; (size * size) as usize];
    if fill_grid(&mut grid, size as usize, 0) {
        return grid;
    }
    return vec![0; (size * size) as usize];
}

/// This function fills the grid one at a time recursively.
/// - grid: the current grid state.
/// i size: the size of the grid.
fn fill_grid(grid: &mut Vec<u32>, size: usize, index: usize) -> bool {
    if index == size * size {
        return true;
    }
    let mut possible_values: Vec<u32> = valid_placements(grid, index / size, index % size, size)
        .into_iter()
        .collect();
    possible_values.shuffle(&mut rng());
    for value in possible_values {
        grid[index] = value;
        if fill_grid(grid, size, index + 1) {
            return true;
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
fn valid_placements(grid: &Vec<u32>, row: usize, col: usize, size: usize) -> HashSet<u32> {
    let mut values: HashSet<u32> = HashSet::new();
    for value in 1..(size + 1) as u32 {
        values.insert(value);
    }
    // on same row
    for index in 0..col {
        values.remove(&grid[row * size + index]);
    }
    // on same column
    for index in 0..row {
        values.remove(&grid[index * size + col]);
    }
    // on diagonal
    for index in 1..size {
        if col < index || row < index {
            break;
        }
        values.remove(&grid[(row - index) * size + (col - index)]);
    }
    for index in 1..size {
        if col + index >= size || row + index >= size {
            break;
        }
        values.remove(&grid[(row + index) * size + (col + index)]);
    }
    for index in 1..size {
        if col < index || row + index >= size {
            break;
        }
        values.remove(&grid[(row + index) * size + (col - index)]);
    }
    for index in 1..size {
        if col + index >= size || row < index {
            break;
        }
        values.remove(&grid[(row - index) * size + (col + index)]);
    }
    return values;
}

/// This function removes a certain number of entries according to the difficulty.
/// - grid: the grid to work with.
/// - diffuculty: the difficulty setting.
///  - size: the size of the grid.
pub fn remove_values(grid: &Vec<u32>, difficulty: u32, size: u32) -> Vec<u32> {
    // we first find a configuration for the minimum number of numbers.
    // as long as it isn't symmetrical, the solution should be unique.

    let shape_row_columns = get_row_columns(grid, size);

    let mut solution: Vec<(u32, u32)> = Vec::new();
    asymmetric_grid_fill(&grid, &shape_row_columns, &mut solution, size);

    let mut return_grid: Vec<u32> = vec![0; (size * size) as usize];
    for (row, col) in solution {
        return_grid[(row * size + col) as usize] = grid[(row * size + col) as usize];
    }
    let mut n_additions = 0;
    // if difficulty is easy
    if difficulty == 0 {
        n_additions = random_range(size..(2 * size));
    }
    // if difficulty is medium
    else if difficulty == 1 {
        n_additions = random_range(size / 2..size);
    }
    // if difficultty is hard
    else if difficulty == 2 {
        n_additions = random_range(0..size / 2);
    }

    for mut i in 0..n_additions as i32 {
        let row = random_range(0..size);
        let col = random_range(0..size);
        if return_grid[(row * size + col) as usize] != 0 {
            // if we've selected an already chosen one, decrement i and keep going
            i -= 1;
        } else {
            return_grid[(row * size + col) as usize] = grid[(row * size + col) as usize];
        }
    }

    return return_grid;
}

/// This function finds the row and column indicies for each of the numbers
fn get_row_columns(grid: &Vec<u32>, size: u32) -> HashMap<u32, Vec<(u32, u32)>> {
    let mut row_columns: HashMap<u32, Vec<(u32, u32)>> = HashMap::new();
    for i in 1..size + 1 {
        row_columns.insert(i, Vec::new());
    }
    for i in 0..size * size {
        let row = i / size;
        let col = i % size;
        if let Some(val) = row_columns.get_mut(&grid[i as usize]) {
            val.push((row, col));
        };
    }
    return row_columns;
}

/// This function recursively adds new numbers to an empty grid until we have a non-symmetric grid with one of each shape.
fn asymmetric_grid_fill(
    grid: &Vec<u32>,
    shape_row_columns: &HashMap<u32, Vec<(u32, u32)>>,
    solution: &mut Vec<(u32, u32)>,
    size: u32,
) -> bool {
    if solution.len() == size as usize {
        if check_symmetry(&solution, size) {
            return false;
        }
        return true;
    }
    let mut current_shape_row_cols = shape_row_columns[&((solution.len() + 1) as u32)].clone();
    current_shape_row_cols.shuffle(&mut rng());
    for (row, col) in current_shape_row_cols {
        solution.push((row, col));
        if asymmetric_grid_fill(grid, shape_row_columns, solution, size) {
            return true;
        }
        solution.remove(solution.len() - 1);
    }

    return false;
}
/// This function checks if the solution is symmetric
fn check_symmetry(solution: &Vec<(u32, u32)>, size: u32) -> bool {
    // checking if symmetric in diagonal
    let mut symmetric = true;
    for (row, col) in solution {
        let mut pair_found = false;
        for (row2, col2) in solution {
            // we look for one example of asymmetry to prove it isn't symmetrical in the diagonal
            if row == col2 && col == row2 {
                pair_found = true;
            }
        }
        if !pair_found {
            symmetric = false;
            break;
        }
    }
    if symmetric {
        return true;
    }

    // checking if symmetric in opposite diagonal
    symmetric = true;
    for (row, col) in solution {
        let mut pair_found = false;
        for (row2, col2) in solution {
            // we look for one example of asymmetry to prove it isn't symmetrical in the diagonal
            if *row == size - *col2 - 1 && *col == size - *row2 - 1 {
                pair_found = true;
                break;
            }
        }
        if !pair_found {
            symmetric = false;
            break;
        }
    }
    if symmetric {
        return true;
    }

    // checking if symmetric along y
    symmetric = true;
    for (row, col) in solution {
        let mut pair_found = false;
        for (row2, col2) in solution {
            // we look for one example of asymmetry to prove it isn't symmetrical along y
            if row == row2 {
                if *col == size - *col2 - 1 {
                    pair_found = true;
                    break;
                }
            }
        }
        if !pair_found {
            symmetric = false;
            break;
        }
    }
    if symmetric {
        return true;
    }

    // checking if symmetric along x
    symmetric = true;
    for (row, col) in solution {
        let mut pair_found = false;
        for (row2, col2) in solution {
            // we look for one example of asymmetry to prove it isn't symmetrical along y
            if col == col2 {
                if *row == size - *row2 - 1 {
                    pair_found = true;
                    break;
                }
            }
        }
        if !pair_found {
            symmetric = false;
            break;
        }
    }
    if symmetric {
        return true;
    }
    return false;
}

#[cfg(test)]
mod tests {
    use crate::game_logic::numbers::check_symmetry;

    use super::*;
    #[test]
    fn test_check_symmetry() {
        // simple symmetry cases
        let test_input: Vec<(u32, u32)> = vec![(1, 1), (1, 2), (2, 1)];
        assert!(check_symmetry(&test_input, 3));

        let test_input: Vec<(u32, u32)> = vec![(0, 1), (1, 2), (2, 1)];
        assert!(check_symmetry(&test_input, 3));

        let test_input: Vec<(u32, u32)> = vec![(1, 0), (1, 2), (1, 1)];
        assert!(check_symmetry(&test_input, 3));

        let test_input: Vec<(u32, u32)> = vec![(0, 2), (2, 0)];
        assert!(check_symmetry(&test_input, 3));

        // simple assymetry cases

        let test_input: Vec<(u32, u32)> = vec![(1, 1), (0, 2), (2, 1)];
        assert!(!check_symmetry(&test_input, 3));

        let test_input: Vec<(u32, u32)> = vec![(0, 1), (2, 2), (2, 1)];
        assert!(!check_symmetry(&test_input, 3));

        let test_input: Vec<(u32, u32)> = vec![(1, 0), (1, 2), (2, 2)];
        assert!(!check_symmetry(&test_input, 3));

        let test_input: Vec<(u32, u32)> = vec![(0, 2), (2, 0), (1, 2)];
        assert!(!check_symmetry(&test_input, 3));
    }
}
