use std::io;
use std::collections::HashMap;
use std::str;

fn main() {
     let coordinate_val: &str = "x1234y12";
     extract_coordinate(coordinate_val);
 }


fn extract_coordinate(temp_coordinate: &str ) -> (i32, i32){
let x_index:usize = temp_coordinate.find('x').unwrap_or(0)  ;
let y_index:usize = temp_coordinate.find('y').unwrap_or(0) ;
let mut x_extracted_value = String::new();
let x_extracted_value_pointer = &mut x_extracted_value;
let mut y_extracted_value = String::new();
let y_extracted_value_pointer = &mut y_extracted_value;
let mut indexdifference_xy:usize = (y_index-1) - x_index;
let mut loopcounter_x:usize = 0;
let loopcounter_x_pointer = &mut loopcounter_x;


 //LOOP FOR X
 let outputx = loop {
    println!("Loop counter is {:?}", & mut *loopcounter_x_pointer);
     let coordinate_convert = temp_coordinate.chars().nth(x_index+1+ *loopcounter_x_pointer).unwrap();
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


println!("OUTPUTTED VALUE FROM LOOP IS{:?}", outputx );


return (outputx,2);
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
