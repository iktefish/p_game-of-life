fn main() {
    let grid: Vec<Vec<char>> = gen_grid();
    std_out_grid(&grid);

    let x = 1;
    let y = 1;
    let focus_cell: char = grid[y][x];

    let edge_cat = check_edge(x, y, grid.len());

    let (r_cell, l_cell, d_cell, u_cell, rd_cell, ld_cell, ru_cell, lu_cell) =
        find_neighbour(x, y, &grid, edge_cat);
    let life_status: i8 = 0;

    println!("r_cell ~~> {}", r_cell);
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
) -> (char, char, char, char, char, char, char, char) {
    let focus_cell = grid[y][x];
    println!("focus_cell ~~> {}", focus_cell);

    // *** My tuple template ***
    // (r_cell, l_cell, d_cell, u_cell, rd_cell, ld_cell, ru_cell, lu_cell)

    if edge_cat == 0 {
        let r_cell = grid[y][x + 1];
        let l_cell = 'v';
        let d_cell = grid[y + 1][x];
        let u_cell = 'v';
        let rd_cell = grid[y + 1][x + 1];
        let ld_cell = 'v';
        let ru_cell = grid[y + 1][x + 1];
        let lu_cell = 'v';

        return (
            r_cell, l_cell, d_cell, u_cell, rd_cell, ld_cell, ru_cell, lu_cell,
        );
    } else if edge_cat == 1 {
        let r_cell = grid[y][x + 1];
        let l_cell = 'v';
        let d_cell = grid[y + 1][x];
        let u_cell = grid[y - 1][x];
        let rd_cell = grid[y + 1][x + 1];
        let ld_cell = 'v';
        let ru_cell = grid[y - 1][x + 1];
        let lu_cell = 'v';

        return (
            r_cell, l_cell, d_cell, u_cell, rd_cell, ld_cell, ru_cell, lu_cell,
        );
    } else if edge_cat == 2 {
        let r_cell = grid[y][x + 1];
        let l_cell = grid[y][x - 1];
        let d_cell = grid[y + 1][x];
        let u_cell = 'v';
        let rd_cell = grid[y + 1][x + 1];
        let ld_cell = grid[y + 1][x - 1];
        let ru_cell = 'v';
        let lu_cell = 'v';

        return (
            r_cell, l_cell, d_cell, u_cell, rd_cell, ld_cell, ru_cell, lu_cell,
        );
    } else if edge_cat == 3 {
        let r_cell = 'v';
        let l_cell = grid[y][x - 1];
        let d_cell = 'v';
        let u_cell = grid[y - 1][x];
        let rd_cell = 'v';
        let ld_cell = 'v';
        let ru_cell = 'v';
        let lu_cell = grid[y - 1][x - 1];

        return (
            r_cell, l_cell, d_cell, u_cell, rd_cell, ld_cell, ru_cell, lu_cell,
        );
    } else if edge_cat == 4 {
        let r_cell = 'v';
        let l_cell = grid[y][x - 1];
        let d_cell = grid[y + 1][x];
        let u_cell = grid[y - 1][x];
        let rd_cell = 'v';
        let ld_cell = grid[y + 1][x - 1];
        let ru_cell = 'v';
        let lu_cell = grid[y - 1][x - 1];

        return (
            r_cell, l_cell, d_cell, u_cell, rd_cell, ld_cell, ru_cell, lu_cell,
        );
    } else if edge_cat == 5 {
        let r_cell = grid[y][x + 1];
        let l_cell = grid[y][x - 1];
        let d_cell = 'v';
        let u_cell = 'v';
        let rd_cell = 'v';
        let ld_cell = 'v';
        let ru_cell = grid[y - 1][x + 1];
        let lu_cell = grid[y - 1][x - 1];

        return (
            r_cell, l_cell, d_cell, u_cell, rd_cell, ld_cell, ru_cell, lu_cell,
        );
    } else {
        let r_cell = grid[y][x + 1];
        let l_cell = grid[y][x - 1];
        let d_cell = grid[y + 1][x];
        let u_cell = grid[y - 1][x];
        let rd_cell = grid[y + 1][x + 1];
        let ru_cell = grid[y - 1][x + 1];
        let ld_cell = grid[y + 1][x - 1];
        let lu_cell = grid[y - 1][x - 1];

        return (
            r_cell, l_cell, d_cell, u_cell, rd_cell, ld_cell, ru_cell, lu_cell,
        );
    }
}

fn check_edge(x: usize, y: usize, len: usize) -> usize {
    if x == 0 && y == 0 {
        println!("return = 0");
        return 0;
    } else if x == 0 && y > 0 && y < len - 1 {
        println!("y is > 0");
        return 1;
    } else if x > 0 && x < len - 1 && y == 0 {
        return 2;
    } else if x == len - 1 && y == len - 1 {
        return 3;
    } else if x == len - 1 && y < len - 1 && y > 0 {
        return 4;
    } else if x < len - 1 && x > 0 && y == len - 1 {
        return 5;
    } else {
        // Condition in this step =>
        // x < len-1 && x > 0 && y < len-1 && y > 0 {
        println!("mid val");
        return 6;
    }
}

// *** TODAY ***
// check_life()
// check_fate()
    // Takes (&focus_cell, find_neighbour(return: char), check_life())
        // * Rules
        //  + Any live cell with fewer than two live neighbours dies, as if by underpopulation
        //  + Any live cell with two or three live neighbours lives on to the next generation
        //  + Any live cell with more than three live neighbours dies, as if by overpopulation
        //  + Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction
    // iterate through check_life((..)) { return i_sum value }
    // if i_sum < 2 ~~> SET grid[y][x] = 