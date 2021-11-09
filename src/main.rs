fn main() {
    let grid: Vec<Vec<char>> = gen_grid();
    std_out_grid(&grid);
    let x = 1;
    let y = 1;
    let edge_cat = check_edge(x, y, grid.len());
    find_neighbour(x, y, &grid, edge_cat);

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

fn find_neighbour(x: usize, y: usize, grid: &Vec<Vec<char>>, edge_cat: usize) -> () {
    let focus_cell = grid[y][x];
    println!("focus_cell ~~> {}", focus_cell);

    // let r_cell = grid[y][x + 1];
    // let l_cell = grid[y][x - 1];
    // let d_cell = grid[y + 1][x];
    // let u_cell = grid[y - 1][x];
    // let rd_cell = grid[y + 1][x + 1];
    // let ru_cell = grid[y - 1][x + 1];
    // let ld_cell = grid[y + 1][x - 1];
    // let lu_cell = grid[y - 1][x - 1];

    // println!("r_cell  ~~> {}", r_cell);                 
    // println!("l_cell  ~~> {}", l_cell);
    // println!("d_cell  ~~> {}", d_cell);
    // println!("u_cell  ~~> {}", u_cell);
    // println!("rd_cell ~~> {}", rd_cell);
    // println!("ru_cell ~~> {}", ru_cell);
    // println!("ld_cell ~~> {}", ld_cell);
    // println!("lu_cell ~~> {}", lu_cell);

        // if x == 0 && y == 0 {
        // println!("jshdjks");
        // return 0;

    if edge_cat == 0 {
        let r_cell = grid[y][x+1];
        let d_cell = grid[y + 1][x];
        let rd_cell = grid[y + 1][x + 1];
        println!("r_cell ~~~> {}", r_cell);
    } else if edge_cat == 1 {
        let r_cell = grid[y][x + 1];
        let d_cell = grid[y + 1][x];
        let u_cell = grid[y - 1][x];
        let rd_cell = grid[y + 1][x + 1];
        let ru_cell = grid[y - 1][x + 1];
    } else if edge_cat == 2 {
        let r_cell = grid[y][x + 1];
        let l_cell = grid[y][x - 1];
        let d_cell = grid[y + 1][x];
        let rd_cell = grid[y + 1][x + 1];
        let ld_cell = grid[y + 1][x - 1];
    } else if edge_cat == 3 {
        let l_cell = grid[y][x - 1];
        let u_cell = grid[y - 1][x];
        let lu_cell = grid[y - 1][x - 1];
    } else if edge_cat == 4 {
        let l_cell = grid[y][x - 1];
        let d_cell = grid[y + 1][x];
        let u_cell = grid[y - 1][x];
        let ld_cell = grid[y + 1][x - 1];
        let lu_cell = grid[y - 1][x - 1];
    } else if edge_cat ==5 {
        let r_cell = grid[y][x + 1];
        let l_cell = grid[y][x - 1];
        let ru_cell = grid[y - 1][x + 1];
        let lu_cell = grid[y - 1][x - 1];
    } else {
        let r_cell = grid[y][x + 1];
        let l_cell = grid[y][x - 1];
        let d_cell = grid[y + 1][x];
        let u_cell = grid[y - 1][x];
        let rd_cell = grid[y + 1][x + 1];
        let ru_cell = grid[y - 1][x + 1];
        let ld_cell = grid[y + 1][x - 1];
        let lu_cell = grid[y - 1][x - 1];
        println!("r_cell ~~~> {}", r_cell);
    }

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
    //
    // For edge cases x=0 and y=0 =>
    // IGNORE
    // y-1
    // x+1 y-1
    // x-1 y-1
    // x-1
    // x-1 y+1
    // x-1 y-1
    //
    // For edge cases x=len-1 and y=len-1 =>
    // IGNORE
    // x-1
    // x-1 y+1
    // x-1 y-1
    // y+1
    // x+1 y+1
    // x-1 y+1
}

    // create function that checks coordinates and notes which edge case is spawned and returns it
    // find_neighbour() should return the element inside each neighbouring cell
    // find_neighbour() checks each neighbouring elem through offsets
    // find_neighbour() panics at edge cases
    // Find way to stop edge case panic
    // We have 6 unique edge cases
    // Each edge case is dependant on coordinates

fn check_edge(x: usize, y: usize, len: usize) -> usize {

    if x == 0 && y == 0 {
        println!("return = 0");
        return 0;
    } else if x == 0 && y > 0 && y < len-1 {
        println!("y is > 0");
        return 1;
    } else if x > 0 && x < len-1 && y == 0 {
        return 2;
    } else if x == len-1 && y == len-1 {
        return 3;
    } else if x == len-1 && y < len-1 && y > 0 {
        return 4;
    } else if x < len-1 && x > 0 && y == len-1 {
        return 5;
    } else {    // x < len-1 && x > 0 && y < len-1 && y > 0 {
        println!("mid val");
        return 6;
    }
}