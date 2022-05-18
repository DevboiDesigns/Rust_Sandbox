// Vectors - resizable arrays 

use std::mem; // import in - Standard Library 

pub fn run() {
    //    "i32" type - "5" length
    let mut numbers: Vec<i32> = vec![1,2,3,4]; // define as vector 

    // Re-assign value
    numbers[2] = 20;

    // Add onto Vector
    numbers.push(5);
    numbers.push(6);

    // remove last value 
    numbers.pop();

    println!("{:?}", numbers); // [1, 2, 3, 4, 5]

    // get single value
    println!("Single value {}", numbers[0]);

    // vector length
    println!("Vector length: {}", numbers.len()); // Array length: 5

    // amount of memory taking up - stack allocated  // std:: standard library
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers)); // Array occupies 20 bytes

    // get slice
    let slice: &[i32] = &numbers[0..2]; // Slice: [1, 2]

    println!("Slice: {:?}", slice); // must use debug pring on array {:?}


    // Loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }
}