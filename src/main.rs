use std::{thread, time};

fn main() {
    let mut grid: Vec<Vec<char>> = gen_grid();
    let mut len = grid.len();
    
    // std_out_grid(&grid);
    let mut x = 0;
    let mut y = 0;

    loop {
        thread::sleep(time::Duration::from_millis(500));
        std::process::Command::new("clear").status().unwrap();
        std_out_grid(&grid);
        let focus_cell = grid[y][x];

        let edge_cat = check_edge(x, y, grid.len());

        let neighbours = find_neighbour(x, y, &grid, edge_cat);
        println!("[0] index of neighbours ~~> {}", neighbours[0]);

        let life_status = check_life(&focus_cell);

        grid = check_fate(x, y, &neighbours, grid);

        if x < len {
            x = x + 1;
        } else if x == len {
            y = y + 1;
        }

        // for i in grid.iter() {
        //     for ii in i.iter() {
        //         if y < len-1 {
        //             y = y + 1;
        //             break;
        //         } else {
        //             break;
        //         }
                
        //     }
        //     if y == len-1 {

        //     }
        // }

    }
    
    // println!("grid ~~> {}", grid[y][x]);
    // println!("r_cell ~~> {}", neighbours[0]);
}

fn gen_grid() -> Vec<Vec<char>> {
    let row_0: Vec<char> = vec!['*', '-', '*', '*', '*', '*', '*', '-', '-', '-'];
    let row_1: Vec<char> = vec!['-', '*', '-', '*', '-', '*', '*', '*', '-', '-'];
    let row_2: Vec<char> = vec!['*', '-', '*', '*', '-', '*', '-', '*', '-', '-'];
    let row_3: Vec<char> = vec!['-', '*', '-', '*', '-', '*', '*', '*', '-', '-'];
    let row_4: Vec<char> = vec!['*', '*', '-', '*', '-', '*', '-', '-', '*', '-'];
    let row_5: Vec<char> = vec!['*', '*', '-', '-', '*', '*', '*', '-', '*', '-'];
    let row_6: Vec<char> = vec!['*', '-', '-', '*', '*', '-', '-', '*', '*', '-'];
    let row_7: Vec<char> = vec!['-', '*', '*', '*', '*', '-', '*', '-', '-', '*'];
    let row_8: Vec<char> = vec!['-', '*', '*', '-', '*', '-', '*', '-', '-', '*'];
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

fn find_neighbour(
    x: usize,
    y: usize,
    grid: &Vec<Vec<char>>,
    edge_cat: usize,
) -> [char; 8] {
    let focus_cell = grid[y][x];
    println!("focus_cell ~~> {}", focus_cell);

    // *** My tuple template *** PS: Now an array!
    // (r_cell, l_cell, d_cell, u_cell, rd_cell, ld_cell, ru_cell, lu_cell)

    if edge_cat == 0 {       // Top-left corner
        let r_cell = grid[y][x + 1];
        let l_cell = 'v';
        let d_cell = grid[y + 1][x];
        let u_cell = 'v';
        let rd_cell = grid[y + 1][x + 1];
        let ld_cell = 'v';
        let ru_cell = grid[y + 1][x + 1];
        let lu_cell = 'v';

        return [
            r_cell, l_cell, d_cell, u_cell, rd_cell, ld_cell, ru_cell, lu_cell,
        ];
    } else if edge_cat == 1 {        // Top-Right corner
        let r_cell = 'v';
        let l_cell = grid[y][x - 1];
        let d_cell = grid[y+1][x];
        let u_cell = 'v';
        let rd_cell = 'v';
        let ld_cell = grid[y+1][x-1];
        let ru_cell = 'v';
        let lu_cell = 'v';

        return [
            r_cell, l_cell, d_cell, u_cell, rd_cell, ld_cell, ru_cell, lu_cell,
        ];
    } else if edge_cat == 2 {        // Bot-Right corner
        let r_cell = 'v';
        let l_cell = grid[y][x - 1];
        let d_cell = 'v';
        let u_cell = grid[y - 1][x];
        let rd_cell = 'v';
        let ld_cell = 'v';
        let ru_cell = 'v';
        let lu_cell = grid[y - 1][x - 1];

        return [
            r_cell, l_cell, d_cell, u_cell, rd_cell, ld_cell, ru_cell, lu_cell,
        ];
    } else if edge_cat == 3 {        // Bot-Left corner
        let r_cell = grid[y][x + 1];
        let l_cell = 'v';
        let d_cell = 'v';
        let u_cell = grid[y - 1][x];
        let rd_cell = 'v';
        let ld_cell = 'v';
        let ru_cell = grid[y-1][x + 1];
        let lu_cell = 'v';

        return [
            r_cell, l_cell, d_cell, u_cell, rd_cell, ld_cell, ru_cell, lu_cell,
        ];
    } else if edge_cat == 4 {      // Left-Edge
        let r_cell = grid[y][x + 1];
        let l_cell = 'v';
        let d_cell = grid[y + 1][x];
        let u_cell = grid[y - 1][x];
        let rd_cell = grid[y + 1][x + 1];
        let ld_cell = 'v';
        let ru_cell = grid[y - 1][x + 1];
        let lu_cell = 'v';

        return [
            r_cell, l_cell, d_cell, u_cell, rd_cell, ld_cell, ru_cell, lu_cell,
        ];
    } else if edge_cat == 5 {      // Top-Edge
        let r_cell = grid[y][x + 1];
        let l_cell = grid[y][x - 1];
        let d_cell = grid[y + 1][x];
        let u_cell = 'v';
        let rd_cell = grid[y + 1][x + 1];
        let ld_cell = grid[y + 1][x - 1];
        let ru_cell = 'v';
        let lu_cell = 'v';

        return [
            r_cell, l_cell, d_cell, u_cell, rd_cell, ld_cell, ru_cell, lu_cell,
        ];
    } else if edge_cat == 6 {        // Right-Edge
        let r_cell = 'v';
        let l_cell = grid[y][x - 1];
        let d_cell = grid[y + 1][x];
        let u_cell = grid[y - 1][x];
        let rd_cell = 'v';
        let ld_cell = grid[y + 1][x - 1];
        let ru_cell = 'v';
        let lu_cell = grid[y - 1][x - 1];

        return [
            r_cell, l_cell, d_cell, u_cell, rd_cell, ld_cell, ru_cell, lu_cell,
        ];
    } else if edge_cat == 7 {        // Bottom-Edge
        let r_cell = grid[y][x + 1];
        let l_cell = grid[y][x - 1];
        let d_cell = 'v';
        let u_cell = 'v';
        let rd_cell = 'v';
        let ld_cell = 'v';
        let ru_cell = grid[y - 1][x + 1];
        let lu_cell = grid[y - 1][x - 1];

        return [
            r_cell, l_cell, d_cell, u_cell, rd_cell, ld_cell, ru_cell, lu_cell,
        ];
    } else {        // Middle Values
        println!("HALLLP IMMA PANICK");
        let r_cell = grid[y][x + 1];
        let l_cell = grid[y][x - 1];
        let d_cell = grid[y + 1][x];
        let u_cell = grid[y - 1][x];
        let rd_cell = grid[y + 1][x + 1];
        let ru_cell = grid[y - 1][x + 1];
        let ld_cell = grid[y + 1][x - 1];
        let lu_cell = grid[y - 1][x - 1];

        return [
            r_cell, l_cell, d_cell, u_cell, rd_cell, ld_cell, ru_cell, lu_cell,
        ];
    }
}

fn check_edge(x: usize, y: usize, len: usize) -> usize {
    if x == 0 && y == 0 {       // Top-left corner
        println!("return = 0");
        return 0;
    } else if x == len -1 && y == 0 {
        return 1;       // Top-Right corner | done
    } else if x == len - 1 && y == len - 1 {        // Bot-Right corner
        return 2;
    } else if x == 0  && y == len -1 {      // Bot-Left corner
        return 3; // | 3
    } else if x == 0 && y > 0 && y < len - 1 {      // Left-Edge
        println!("y is > 0");
        return 4;
    } else if x > 0 && x < len - 1 && y == 0 {      // Top-Edge
        return 5;
    } else if x == len - 1 && y < len - 1 && y > 0 {        // Right-Edge
        return 6;
    } else if x < len - 1 && x > 0 && y == len - 1 {        // Bottom-Edge
        return 7;
    } else {        // Middle Values
        // Condition in this step =>
        // x < len-1 && x > 0 && y < len-1 && y > 0 
        println!("mid val");
        return 8;
    }
}

fn check_life(focus_cell: &char) -> i8 {
    if focus_cell == &'*' {
        return 1;

    } else if focus_cell == &'-'{
        return 0;

    } else {
        return -1;
    }
}

// *** TODAY ***
// check_life()
// check_fate()
    // Takes (x, y, neighbours, check_life())
        // * Rules
        //  + Any live cell with fewer than two live neighbours dies, as if by underpopulation | 
        //  + Any live cell with two or three live neighbours lives on to the next generation |
        //  + Any live cell with more than three live neighbours dies, as if by overpopulation 
        //  + Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction
    // iterate through check_life((..)) { return i_sum value }
    // if i_sum < 2 ~~> SET grid[y][x] = "-"
    // else if ...
    // else | i_sum > 3 |

fn check_fate(x: usize, y: usize, neighbours: &[char; 8], mut grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut i_sum = 0;
    for i in neighbours.iter() {
        if check_life(i) == 1 {
            i_sum = i_sum + 1;
            println!("i_sum ~~> {}", i_sum);
        }
    }
    if i_sum < 2 {
        grid[y][x] = '0';

    } else if i_sum >= 2 && i_sum <= 3 {
        grid[y][x] = '1';

    } else if i_sum > 3 {
        grid[y][x] = '0';

    } else if i_sum == 3 {
        grid[y][x] = '1';

    }
    return grid;
}
