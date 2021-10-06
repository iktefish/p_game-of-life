use std::ffi::CString;
use std::io;
use std::collections::HashMap;
use core::iter;

fn main() {
    let coordinate_val: &str = "x11y13";
    // initialize hashmap
    let mut data_map: HashMap<&str, char> = HashMap::new();

    // NOTE: This is the ~push()~ function
    // To append data into HashMap
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
    // let coordinate_tup = update_coordinate(grid_size.parse().unwrap());
    let coordinate_tup = update_coordinate(grid_size.parse().unwrap(), coordinate_val.to_string()); // sending ~coordinate_val~ as param for testing
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

// fn update_coordinate(grid_size: i32) -> (String, i32, i32) {
fn update_coordinate(grid_size: i32, coordinate_val: String) -> (String, i32, i32) {
    let both_idx = extract_coordinate(&coordinate_val);
    let mut hypo_x_idx: i32 = both_idx.0;
    let mut hypo_y_idx: i32 = both_idx.1;
    // let mut hypo_x_idx: i32 = 11;
    // let mut hypo_y_idx: i32 = 13;
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

fn extract_coordinate(temp_coordinate: &str ) -> (i32, i32) {
    let x_index:usize = temp_coordinate.find('x').unwrap_or(0)  ;
    let y_index:usize = temp_coordinate.find('y').unwrap_or(0) ;
    let mut x_extracted_value = String::new();
    let x_extracted_value_pointer = &mut x_extracted_value;
    let mut y_extracted_value = String::new();
    let y_extracted_value_pointer = &mut y_extracted_value;
    let mut indexdifference_xy:usize = (y_index-1) - x_index;
    let endindex:usize = temp_coordinate.chars().count();
    let mut indexdifference_y_end:usize = endindex - (y_index+1);
    let mut loopcounter_x:usize = 0;
    let loopcounter_x_pointer = &mut loopcounter_x;
    let mut loopcounter_y:usize = 0;
    let loopcounter_y_pointer = &mut loopcounter_y;

    //LOOP FOR X
    let output_x = loop {
        println!("Loop counter is {:?}", & mut *loopcounter_x_pointer);
        // let coordinate_convert = temp_coordinate.chars().nth(x_index+1+ *loopcounter_x_pointer).unwrap();
        let coordinate_add_x = temp_coordinate.chars().nth(x_index+1+ *loopcounter_x_pointer).unwrap() ;
        println!("string to be added {:?}", coordinate_add_x );
        let x_finalstring = &mut *x_extracted_value_pointer;
        x_finalstring.push(coordinate_add_x);                 // FINAL STRING THAT NEEDS TO BE RETURNED

        println!("PRINTED FINAL STRING {:?}", x_finalstring );
        println!("PRINTED FINAL STRING AFTER CONVERSION{:?}", convert_string_toi32(x_finalstring.to_string()) );
        *loopcounter_x_pointer =  *loopcounter_x_pointer + 1;

        if &mut *loopcounter_x_pointer >= & mut indexdifference_xy{
            break convert_string_toi32(x_finalstring.to_string());
        }
    };

    //LOOP FOR Y
    let output_y = loop {
        println!("Loop counter is {:?}", & mut *loopcounter_y_pointer);
        let coordinate_add_y = temp_coordinate.chars().nth(y_index+1+ *loopcounter_y_pointer).unwrap() ;
        println!("{:?}", coordinate_add_y );
        let y_finalstring = &mut *y_extracted_value_pointer;
        y_finalstring.push(coordinate_add_y);
        println!("PINTED FINAL STRING {:?}", y_finalstring);
        println!("PRINTED FINAL STRING AFTER CONVERSION{:?}", convert_string_toi32(y_finalstring.to_string()) );
        *loopcounter_y_pointer =  *loopcounter_y_pointer + 1;

        if &mut *loopcounter_y_pointer >= & mut indexdifference_y_end{
            break convert_string_toi32(y_finalstring.to_string());
        }
    };

    println!("OUTPUTTED X VALUE FROM LOOP IS{:?}", output_x );
    println!("OUTPUTTED Y VALUE FROM LOOP IS{:?}", output_y);

    return (output_x,output_y);
}

fn convert_string_toi32 (to_be_converted:String ) -> i32 {
    let converted_value = to_be_converted.parse::<i32>().unwrap();
    return converted_value;

}

// -----------------------------------
// This function is used to find types
// Use when needed
// -----------------------------------
fn find_type<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
