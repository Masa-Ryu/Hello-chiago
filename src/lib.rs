//! # Hello Chiago
//! Welcome to the hello_chiago library! You can see the usage of this library in the examples below.
//! # Examples
//! ```
//! use hello_chiago::greeting;
//!fn main() {
//!    greeting();
//!}
//! ```

pub fn greeting() {
    println!("Hello, Chiago!");

    let color_address: [[i32; 32]; 32] = [
        [3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3],
        [3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3],
        [3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3],
        [3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3],
        [3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3],
        [3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3],
        [3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 1, 1, 3, 3, 3, 3, 3, 3, 3, 3, 3],
        [3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 1, 1, 3, 3, 3, 3, 3, 3, 3, 3, 1, 6, 6, 1, 3, 3, 3, 3, 3, 3, 3, 3],
        [3, 3, 3, 3, 3, 3, 3, 3, 3, 1, 6, 6, 1, 3, 3, 3, 3, 3, 3, 1, 1, 6, 6, 1, 3, 3, 3, 3, 3, 3, 3, 3],
        [3, 3, 3, 3, 3, 3, 3, 3, 1, 6, 6, 6, 1, 3, 3, 3, 3, 3, 3, 1, 6, 6, 6, 6, 1, 3, 3, 3, 3, 3, 3, 3],
        [3, 3, 3, 3, 3, 3, 3, 3, 1, 6, 6, 6, 6, 1, 3, 3, 3, 1, 1, 6, 6, 5, 5, 6, 1, 3, 3, 3, 3, 3, 3, 3],
        [3, 3, 3, 3, 3, 3, 3, 1, 6, 5, 5, 5, 6, 6, 1, 1, 1, 1, 5, 5, 5, 5, 5, 5, 5, 1, 3, 3, 3, 3, 3, 3],
        [3, 3, 3, 3, 3, 3, 3, 1, 5, 5, 7, 7, 7, 5, 5, 5, 5, 5, 5, 5, 7, 7, 7, 5, 5, 1, 3, 3, 3, 3, 3, 3],
        [3, 3, 3, 3, 3, 3, 1, 5, 5, 5, 4, 4, 8, 5, 5, 5, 5, 5, 5, 5, 4, 4, 8, 5, 5, 1, 3, 3, 3, 3, 3, 3],
        [3, 3, 3, 3, 3, 3, 1, 5, 5, 5, 2, 4, 8, 5, 5, 5, 5, 5, 5, 5, 2, 4, 8, 5, 5, 5, 1, 3, 3, 3, 3, 3],
        [3, 3, 3, 3, 3, 1, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 2, 2, 5, 5, 5, 5, 5, 5, 5, 5, 1, 3, 3, 3, 3, 3],
        [3, 3, 3, 3, 1, 5, 5, 5, 5, 5, 5, 5, 5, 6, 6, 6, 2, 2, 6, 6, 6, 6, 5, 5, 5, 5, 1, 3, 3, 3, 3, 3],
        [3, 3, 3, 3, 1, 5, 5, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 5, 5, 5, 1, 3, 3, 3, 3, 3],
        [3, 3, 3, 3, 1, 5, 5, 5, 5, 5, 5, 5, 6, 6, 6, 6, 2, 2, 2, 2, 6, 6, 6, 5, 5, 1, 3, 3, 3, 3, 3, 3],
        [3, 3, 3, 1, 1, 5, 5, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 9, 9, 6, 6, 6, 6, 5, 5, 1, 3, 3, 3, 3, 3, 3],
        [3, 3, 1, 10, 10, 1, 1, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 9, 9, 6, 6, 6, 6, 5, 5, 1, 3, 3, 3, 3, 3, 3],
        [3, 1, 10, 10, 10, 10, 1, 1, 1, 1, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 1, 3, 3, 3, 3, 3, 3],
        [1, 1, 1, 10, 10, 10, 10, 10, 10, 1, 1, 1, 1, 1, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 1, 3, 3, 3, 3, 3, 3, 3],
        [5, 5, 1, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 11, 11, 11, 5, 5, 5, 5, 5, 5, 1, 3, 3, 3, 3, 3, 3, 3, 3],
        [5, 5, 5, 1, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 11, 11, 11, 11, 11, 1, 3, 3, 3, 3, 3, 3, 3, 3, 3],
        [5, 5, 5, 5, 1, 1, 1, 1, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 1, 3, 3, 3, 3, 3, 3, 3, 3, 3],
        [5, 5, 5, 5, 5, 5, 5, 5, 1, 1, 1, 1, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 1, 3, 3, 3, 3, 3, 3, 3, 3, 3],
        [5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 1, 1, 1, 1, 1, 1, 10, 10, 10, 1, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3],
        [5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 1, 1, 1, 1, 1, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3],
        [5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 1, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3],
        [5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 1, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3],
        [5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 1, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3],
    ];

    for i in color_address {
        for j in i {
            print!("\x1b[48;2;{}m  \x1b[0m", color_output(j));
        }
        println!("")
    }

}

fn color_output(number: i32) -> String{
    match number {
        1 => "1;0;3".to_string(),
        2 => "64;65;63".to_string(),
        3 => "14;129;58".to_string(),
        4 => "129;129;128".to_string(),
        5 => "254;192;1".to_string(),
        6 => "255;216;102".to_string(),
        7 => "254;240;208".to_string(),
        8 => "242;243;242".to_string(),
        9 => "255;125;128".to_string(),
        10 => "254;103;0".to_string(),
        11 => "204;83;0".to_string(),
        _ => "255;255;255".to_string()
    }

}
