// Arrays - fixed list where elements are the same data types 

use std::mem; // import in - Standard Library 

pub fn run() {
    //    "i32" type - "5" length
    let mut numbers: [i32; 5] = [1,2,3,4,5];

    // Re-assign value
    numbers[2] = 20;

    println!("{:?}", numbers); // [1, 2, 3, 4, 5]

    // get single value
    println!("Single value {}", numbers[0]);

    // array length
    println!("Array length: {}", numbers.len()); // Array length: 5

    // amount of memory taking up - stack allocated  // std:: standard library
    println!("Array occupies {} bytes", mem::size_of_val(&numbers)); // Array occupies 20 bytes

    // get slice
    let slice: &[i32] = &numbers[0..2]; // Slice: [1, 2]

    println!("Slice: {:?}", slice); // must use debug pring on array {:?}
}