/*
Primitive Types - 
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, (number of bits they take in memory) -  i32 most common
                       u - no negative value               i - postive or negative 
Floats: f32, f64
Boolean: (bool)
Characters: (char) - one character 
Tuples
Arrays - fixed length
*/

// Rust is a staticly typed language, which means that is must know the types of all variables at the compile time, however,
// the compiler can usually infer what type we want to use based on the value and how we use it 



pub fn run() {
    // Default is i32
    let x = 1;

    // Default is f64
    let y = 2.5;

    // Add 
    let z: i64 = 45454454545454;

    // Find max size
    println!("Max i32: {}", std::i32::MAX);

    // Boolean
    let is_active = true;

    // Get bool from expression
    let is_greater: bool = 10 > 5;

    println!("{:?}", (x, y, z, is_active));

    println!("{}", is_greater);

    let a1 = 'a';
    println!("{}", a1);

    let face = '\u{1F600}'; // 😀 - unicode for emojis 
    println!("{}", face); 

  

}