use std::ffi::CString;
use std::io;
use std::collections::HashMap;
use core::iter;

fn main() {
    let coordinate_val: &str = "x0y0";
    // initialize hashmap
    let mut data_map: HashMap<&str, char> = HashMap::new();

    // data_map.insert(
    //     "hello",
    //     '1'
    // );
    // println!("HashMap value {}", data_map["hello"]);

    let grid_size: String = loop {
        let ret_check: (bool, String) = check_input();

        if ret_check.0 == true {
            // DEBUG
            // println!("From main ---> {:?}, {:?}", ret_check.0, ret_check.1);

            println!("Grid dimensions set to {} x {}", ret_check.1, ret_check.1);

            // These 2 lines wont compile as ret_check.1 dies when loop ends
            // *grid_size_ptn = ret_check.1.as_str();
            // *grid_size_ptn = &test_var;
            break ret_check.1;
        };
    };
    // println!("RUNRUNRUN {}", iterate_through_check());

    println!("grid_size ---> {}", grid_size);
    let coordinate_tup = update_coordinate(grid_size.parse().unwrap());
    data_map.insert(&coordinate_tup.0 as &str, ' ');
    println!("HashMap value {}", data_map[&coordinate_tup.0 as &str]);
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

fn update_coordinate(grid_size: i32) -> (String, i32, i32) {
    // extract_coordinates()
    let mut hypo_x_idx: i32 = 11;
    let mut hypo_y_idx: i32 = 13;
    if hypo_x_idx >= grid_size - 1 {
        hypo_x_idx = 0;
        hypo_y_idx = hypo_y_idx + 1;
    } else {
        hypo_x_idx = hypo_x_idx + 1;
    }
    println!("hypo_x_idx ---> {}", hypo_x_idx);
    println!("hypo_y_idx ---> {}", hypo_y_idx);
    // new_coordinate(hypo_x_idx, hypo_y_idx);
    println!("{}", grid_size);

    let new_coordinate: String = gen_new_coordinate(hypo_x_idx.to_string(), hypo_y_idx.to_string());
    return (new_coordinate, hypo_x_idx, hypo_y_idx);
    // return (hypo_x_idx.to_string(), hypo_y_idx.to_string());
}

fn gen_new_coordinate(x_arg: String, y_arg: String) -> String {
    let mut new_coordinate: String = "x".to_string();
    new_coordinate.push_str(&x_arg);
    new_coordinate.push_str("y");
    new_coordinate.push_str(&y_arg);
    println!("{}", new_coordinate);
    return new_coordinate;
}


// -----------------------------------
// This function is used to find types
// Use when needed
// -----------------------------------
fn find_type<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
