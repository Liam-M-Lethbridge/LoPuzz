mod game_logic;
use crate::game_logic::queens::generate_grid;

extern crate rand;
fn main() {
    let size = 8;
    match generate_grid(size) {
        Some(grid) => {
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
        None => println!("No possible solution"),
    }
}
