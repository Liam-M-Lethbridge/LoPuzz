/// this function prints the grid.
/// - grid: the grid.
/// - size: the size of the grid.
pub fn print_grid(grid: Vec<u32>, size: u32){
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