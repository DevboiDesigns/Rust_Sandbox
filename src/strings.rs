// Primitve Str = Immutable fixed-length string somewhere in memory
// String = Growable, heap allocated data structure - Use when you need to modify or own string data 

pub fn run() {
    let hello = "Hello";  // primitve not growable
    let mut new_hello = String::from("hello again "); // Growable - mutable 

    // Get length
    println!("Length: {}", new_hello.len());

    // Push - add to string  - only for 'char' one character  
    new_hello.push('W');

    // Push whole words
    new_hello.push_str("orld!");
    println!("{}", new_hello);

    // Capacity 
    println!("Capacity: {}", new_hello.capacity()); // Capacity: 22

    // Check if empty 
    println!("Is empty: {}", new_hello.is_empty()); // Is empty: false

    // Contains
    println!("Contains 'World' {}", new_hello.contains("World")); // Contains 'World' true

    // Replace 
    println!("Replace: {}", new_hello.replace("World", "There")); // Replace: hello againThere!

    // Loop through string by whitespace 
    for word in new_hello.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity 
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    println!("{}", s);   

    // Assertion Testing
    assert_eq!(2, s.len()); // only shows in console if error
    assert_eq!(10, s.capacity()); 
}