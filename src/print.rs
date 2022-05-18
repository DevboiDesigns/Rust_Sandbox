// pub - public
pub fn run() {
    // Print to console 
    println!("Hello from print file");

    println!("Number: {}", 1); // Number: 1 

    // Basic formatting
    println!("{} is from {}", "Brad", "Mass"); // Brad is from Mass

    // Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "code"); // Brad is from Mass and Brad likes to code

    //Named Arguments
    println!("{name} likes to play {activity}", name = "John", activity = "Baseball"); // John likes to play Baseball

    // Placeholder 
    println!("Binary: {:b} Hex: {:x} Ocatal: {:o}", 10, 10, 10); // Binary: 1010 Hex: a Ocatal: 12

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello")); // (12, true, "hello") - tuple 

    // Basic math
    println!("10 + 10 = {}", 10 + 10); // 10 + 10 = 20

}
