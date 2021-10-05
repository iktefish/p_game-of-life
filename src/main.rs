use std::io;
use std::collections::HashMap;

fn main() {
    let coordinate_val: &str = "x0y0";
    // initialize hashmap
    let mut data_map: HashMap<&str, char> = HashMap::new();

    // data_map.insert(
    //     "hello",
    //     '1'
    // );
    // println!("HashMap value {}", data_map["hello"]);

    loop {
        let ret_check: (bool, String) = check_input();

        if ret_check.0 == true {
            // DEBUG
            // println!("From main ---> {:?}, {:?}", ret_check.0, ret_check.1);

            println!("Grid dimensions set to {} x {}", ret_check.1, ret_check.1);
            break;
        }
    }
 }

fn get_input() -> String {
    println!("Please enter the grid size" );

    let mut input_string: String = String::new();
    std::io::stdin().read_line(&mut input_string).expect("Failed");
    return input_string;
}

fn check_input() -> (bool, String) {
    let user_input: String = get_input(); // i get a String

    let check_bool: bool = user_input.trim().chars().all(char::is_numeric); // check if my str is a numeric

    // // DEBUG
    // println!("Is user_input only numerals ---> {}", check_bool); // print bool

    match check_bool {
        true => return (true, user_input.trim().to_string()),
        false => return (false, "Please enter a proper number!".trim().to_string()),
    }
}

// -----------------------------------
// This function is used to find types
// Use when needed
// -----------------------------------
// fn find_type<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }
