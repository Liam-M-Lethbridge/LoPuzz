mod game_logic;
use crate::game_logic::queens::create_queens_game;

extern crate rand;
fn main() {
    let size = 8;
    let grid = create_queens_game(size);
    let sum: u32 = grid.iter().sum();
    if sum == 0{
        println!("No possible solution");
        return;
    }
    

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
