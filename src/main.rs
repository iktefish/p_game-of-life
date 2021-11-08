fn main() {
    gen_grid();
}

fn gen_grid() {
    const n: usize = 4;

    let parr: [i32; n] = [1, 6, 11, 21];
    let marr: [i32; n] = [2, 7, 12, 32];
    let garr: [i32; n] = [3, 8, 13, 33];
    let jarr: [i32; n] = [4, 9, 14, 34];

    let arr: [[i32; n]; n] = [parr, marr, garr, jarr];

    for i in arr.iter() {
        println!("{:?}", i);
    }
}
