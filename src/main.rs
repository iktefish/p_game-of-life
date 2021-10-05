use std::io;
use std::collections::HashMap;
use std::str;

fn main() {
     let coordinate_val: &str = "x232y12";

     extract_coordinate(coordinate_val);


    // // initialize hashmap
    // let mut data_map: HashMap<&str, char> = HashMap::new();
    //
    // // data_map.insert(
    // //     "hello",
    // //     '1'
    // // );
    // // println!("HashMap value {}", data_map["hello"]);
    //
    // loop {
    //     let ret_check: (bool, String) = check_input();
    //
    //     if ret_check.0 == true {
    //         // DEBUG
    //         // println!("From main ---> {:?}, {:?}", ret_check.0, ret_check.1);
    //
    //         println!("Grid dimensions set to {} x {}", ret_check.1, ret_check.1);
    //         break;
    //     }
    // }
 }
//
// fn get_input() -> String {
//     println!("Please enter the grid size" );
//
//     let mut input_string: String = String::new();
//     std::io::stdin().read_line(&mut input_string).expect("Failed");
//     return input_string;
// }
//
// fn check_input() -> (bool, String) {
//     let user_input: String = get_input(); // i get a String
//
//     let check_bool: bool = user_input.trim().chars().all(char::is_numeric); // check if my str is a numeric
//
//     // // DEBUG
//     // println!("Is user_input only numerals ---> {}", check_bool); // print bool
//
//     match check_bool {
//         true => return (true, user_input.trim().to_string()),
//         false => return (false, "Please enter a proper number!".trim().to_string()),
//     }
// }



fn extract_coordinate(temp_coordinate: &str ) -> (i32, i32){
 //x232y12
let x_index = temp_coordinate.find('x').unwrap_or(0) ; //unwrapped some(x) convert to i32
let y_index = temp_coordinate.find('y').unwrap_or(0) ; //unwrapped some(x) convert to i32 as i32
let mut x_extracted_value = String::new();
let x_extracted_value_pointer = &mut x_extracted_value;
let mut y_extracted_value = String::new();
let y_extracted_value_pointer = &mut y_extracted_value;

let mut indexdifference_xy = (y_index-1) - x_index;

let mut loopcounter_x = 0;
let mut loopcounter_x_pointer = &mut loopcounter_x;

loop{
    if &mut *loopcounter_x_pointer >= & mut indexdifference_xy{
        break;
    }

    println!("Loop counter is {:?}", & mut *loopcounter_x_pointer);
    let coordinate_add_x = temp_coordinate.chars().nth(x_index+1+ *loopcounter_x_pointer).unwrap();
    println!("string to be added {:?}", coordinate_add_x );
    let x_finalstring = &mut *x_extracted_value_pointer;
    x_finalstring.push(coordinate_add_x);

    println!("{:?}", x_finalstring );
    *loopcounter_x_pointer =  *loopcounter_x_pointer + 1;

}






//println!("sliced string is {:?}", temp_coordinate.chars().nth(x_index+1).unwrap() );
println!("{:?}",indexdifference_xy );

return (1,2);











}













// -----------------------------------
// This function is used to find types
// Use when needed
// -----------------------------------
 fn find_type<T>(_: &T) {
     println!("{}", std::any::type_name::<T>())
 }
