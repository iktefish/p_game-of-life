fn main() {
    let grid: Vec<Vec<char>> = gen_grid();
    std_out_grid(&grid);
    find_neighbour(1, 1, &grid);
}

fn gen_grid() -> Vec<Vec<char>> {
    let row_0: Vec<char> = vec!['7', '8', '9', '*', '*', '*', '*', '-', '-', '-'];
    let row_1: Vec<char> = vec!['4', '.', '6', '*', '-', '*', '*', '-', '-', '-'];
    let row_2: Vec<char> = vec!['1', '2', '3', '*', '-', '*', '-', '-', '-', '-'];
    let row_3: Vec<char> = vec!['-', '*', '-', '*', '-', '*', '*', '-', '-', '-'];
    let row_4: Vec<char> = vec!['-', '*', '-', '*', '-', '*', '*', '-', '*', '-'];
    let row_5: Vec<char> = vec!['-', '*', '-', '*', '*', '*', '*', '-', '*', '-'];
    let row_6: Vec<char> = vec!['-', '*', '-', '*', '*', '-', '-', '-', '*', '-'];
    let row_7: Vec<char> = vec!['-', '*', '*', '*', '*', '-', '*', '-', '-', '*'];
    let row_8: Vec<char> = vec!['-', '*', '*', '*', '*', '-', '*', '-', '-', '*'];
    let row_9: Vec<char> = vec!['-', '*', '*', '*', '*', '-', '*', '-', '-', '*'];

    let grid: Vec<Vec<char>> = vec![
        row_0, row_1, row_2, row_3, row_4, row_5, row_6, row_7, row_8, row_9,
    ];
    return grid;
}

fn std_out_grid(grid: &Vec<Vec<char>>) -> () {
    for i in grid.iter() {
        for ii in i.iter() {
            print!("{} ", ii);
        }
        println!();
    }
}

fn find_neighbour(x: usize, y: usize, grid: &Vec<Vec<char>>) -> () {
    let focus_cell = grid[y][x];
    println!("focus_cell ~~> {}", focus_cell);

    let r_cell = grid[y][x + 1];
    let l_cell = grid[y][x - 1];
    let d_cell = grid[y + 1][x];
    let u_cell = grid[y - 1][x];
    let rd_cell = grid[y + 1][x + 1];
    let ru_cell = grid[y - 1][x + 1];
    let ld_cell = grid[y + 1][x - 1];
    let lu_cell = grid[y - 1][x - 1];

    println!("r_cell  ~~> {}", r_cell);
    println!("l_cell  ~~> {}", l_cell);
    println!("d_cell  ~~> {}", d_cell);
    println!("u_cell  ~~> {}", u_cell);
    println!("rd_cell ~~> {}", rd_cell);
    println!("ru_cell ~~> {}", ru_cell);
    println!("ld_cell ~~> {}", ld_cell);
    println!("lu_cell ~~> {}", lu_cell);

    // Our offsets =>
    // x+1
    // x-1
    // y+1
    // y-1
    // x+1 y+1
    // x+1 y-1
    // x-1 y+1
    // x-1 y-1
    //
    // For edge cases y=0 =>
    // IGNORE
    // y-1
    // x+1 y-1
    // x-1 y-1
    // For edge cases y=len-1 =>
    // IGNORE
    // y+1
    // x+1 y+1
    // x-1 y+1
    //
    // For edge cases x=0 =>
    // IGNORE
    // x-1
    // x-1 y+1
    // x-1 y-1
    // For edge cases x=len-1 =>
    // IGNORE
    // x+1
    // x+1 y+1
    // x+1 y-1
}
