// use std::collections::HashMap;  // to use hashmap
// // use std::io;                    // to take user input
use std::io;                    // to take user input

fn main() {
//let mut grid_size = get_input().trim().parse::<i64>().unwrap();
//println!("{}", grid_size + 2 );
println!("From main ---> {:?}",checkinput());

 }

fn get_input() -> String{
// fn get_input() -> &'static str{
    println!("Please enter the grid size" );

    let mut input_string = String::new();
    std::io::stdin().read_line(&mut input_string).expect("Failed");
    return input_string;

    // let my_own_str: String = input_string.to_owned();
    // let sliced_str: &str = &my_own_str[..];

    // println!("sliced_str ---> {}\nmy_own_string ---> {}", sliced_str, my_own_str);

    // return sliced_str;

    // let my_test_str: &str = "2";
    // return my_test_str;
}

fn checkinput() -> bool{
    // match get_input().bytes().all(|c| c.is_ascii_digit()) {

    // let  test = get_input().bytes().all(|c| c.is_ascii_digit());
    // let test = get_input().chars().all(char::is_numeric);

    let test_var = get_input(); // i get a String

    let my_own_str: String = test_var.to_owned(); // i own the Strin
    let sliced_str: &str = &my_own_str[..];       // i cut Strin into str
    let sliced_str_new: &str = "123312";       // i cut Strin into str
    // let sliced_str: &str = test_var.as_str();       // i cut Strin into str
    // let sliced_str: &str = "123";       // if i put a str "123" then true

    println!("sliced_str ---> {}", sliced_str); // print to check input val
    println!("my_own_string ---> {}", my_own_str); // print to check input val
    // let test = sliced_str.trim().chars().all(char::is_numeric); // check if my str is a numeric


    // NOTE: FIXED - when user puts input Rust takes the input and a linebreak ("123\n")
    // We just need to remove the linebreak and everything will work
    // This way the slice is also not needed, and STAY AWAY FROM SLICING STRINGS
    // Strings in Rust are UTF-8 encoded and slicing may mess it up
    let test = test_var.trim().chars().all(char::is_numeric); // check if my str is a numeric

    println!("---------------------------------------");
    println!("Type of my_own_str");
    find_type(&my_own_str);
    println!("---------------------------------------");
    println!("Type of sliced_str");
    find_type(&sliced_str);
    println!("---------------------------------------");
    println!("Type of sliced_str_new");
    find_type(&sliced_str_new);
    println!("---------------------------------------");

    println!("TEST ---> {}", test); // print bool

    match test {
        true => return true,
        false => return false,
    }
}

fn find_type<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
