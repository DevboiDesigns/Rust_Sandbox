// Primitve Str = Immutable fixed-length string somewhere in memory
// String = Growable, heap allocated data structure - Use when you need to modify or own string data 

pub fn run() {
    let hello = "Hello";  // primitve not growable
    let mut newHello = String::from("hello again "); // Growable - mutable 

    // Get length
    println!("Length: {}", newHello.len());

    // Push - add to string  - only for 'char' one character  
    newHello.push('W');

    // Push whole words
    newHello.push_str("orld!");
    println!("{}", newHello);

    // Capacity 
    println!("Capacity: {}", newHello.capacity()); // Capacity: 22

    // Check if empty 
    println!("Is empty: {}", newHello.is_empty()); // Is empty: false

    // Contains
    println!("Contains 'World' {}", newHello.contains("World")); // Contains 'World' true

    // Replace 
    println!("Replace: {}", newHello.replace("World", "There")); // Replace: hello againThere!

    // Loop through string by whitespace 
    for word in newHello.split_whitespace() {
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