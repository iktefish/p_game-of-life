fn main() {
    sh_gen_grid();
}

fn sh_gen_grid() {
    // const N: usize = 10;
    //
    // let xy_arr_0: [char; N] = ['*', '*', '*', '*', '*', '*', '*', '-', '-', '-'];
    // let xy_arr_1: [char; N] = ['*', '*', '*', '*', '-', '*', '*', '-', '-', '-'];
    // let xy_arr_2: [char; N] = ['*', '*', '*', '*', '-', '*', '-', '-', '-', '-'];
    // let xy_arr_3: [char; N] = ['-', '*', '-', '*', '-', '*', '*', '-', '-', '-'];
    // let xy_arr_4: [char; N] = ['-', '*', '-', '*', '-', '*', '*', '-', '*', '-'];
    // let xy_arr_5: [char; N] = ['-', '*', '-', '*', '*', '*', '*', '-', '*', '-'];
    // let xy_arr_6: [char; N] = ['-', '*', '-', '*', '*', '-', '-', '-', '*', '-'];
    // let xy_arr_7: [char; N] = ['-', '*', '*', '*', '*', '-', '*', '-', '-', '*'];
    // let xy_arr_8: [char; N] = ['-', '*', '*', '*', '*', '-', '*', '-', '-', '*'];
    // let xy_arr_9: [char; N] = ['-', '*', '*', '*', '*', '-', '*', '-', '-', '*'];
    //
    // let arr: [[char; N]; N] = [
    //     xy_arr_0, xy_arr_1, xy_arr_2, xy_arr_3, xy_arr_4, xy_arr_5, xy_arr_6, xy_arr_7, xy_arr_8,
    //     xy_arr_9,
    // ];
    //
    // // for i in arr.iter() {
    // //     println!("{:?}", i);
    // // }
    //
    // for i in arr.iter() {
    //     for ii in i.iter() {
    //         print!("{} ", ii);
    //     }
    //     println!();
    // }

    const N: usize = 10;

    let xy_arr_0: Vec<char> = vec!['*', '*', '*', '*', '*', '*', '*', '-', '-', '-'];
    let xy_arr_1: Vec<char> = vec!['*', '*', '*', '*', '-', '*', '*', '-', '-', '-'];
    let xy_arr_2: Vec<char> = vec!['*', '*', '*', '*', '-', '*', '-', '-', '-', '-'];
    let xy_arr_3: Vec<char> = vec!['-', '*', '-', '*', '-', '*', '*', '-', '-', '-'];
    let xy_arr_4: Vec<char> = vec!['-', '*', '-', '*', '-', '*', '*', '-', '*', '-'];
    let xy_arr_5: Vec<char> = vec!['-', '*', '-', '*', '*', '*', '*', '-', '*', '-'];
    let xy_arr_6: Vec<char> = vec!['-', '*', '-', '*', '*', '-', '-', '-', '*', '-'];
    let xy_arr_7: Vec<char> = vec!['-', '*', '*', '*', '*', '-', '*', '-', '-', '*'];
    let xy_arr_8: Vec<char> = vec!['-', '*', '*', '*', '*', '-', '*', '-', '-', '*'];
    let xy_arr_9: Vec<char> = vec!['-', '*', '*', '*', '*', '-', '*', '-', '-', '*'];

    let vi_vec: Vec<Vec<char>> = vec![
        xy_arr_0, xy_arr_1, xy_arr_2, xy_arr_3, xy_arr_4, xy_arr_5, xy_arr_6, xy_arr_7, xy_arr_8,
        xy_arr_9,
    ];

    // for i in arr.iter() {
    //     println!("{:?}", i);
    // }

    for i in vi_vec.iter() {
        for ii in i.iter() {
            print!("{} ", ii);
        }
        println!();
    }
}
