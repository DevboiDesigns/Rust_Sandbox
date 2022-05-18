// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped lang

pub fn run() {
    let name = "Brad";
    let mut age = 37; // make variable mutable 

    age = 38;

    println!("My name is {} and I am {}", name, age);

    // Define Constant
    const ID: i32 = 001; // must declare type on const 
    println!("ID: {}", ID);

    // Assign mutable variables at once 
    let ( my_name, my_age ) = ("Brad", 37);
    println!("{} is {}", my_name, my_age);
}