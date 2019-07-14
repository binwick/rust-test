pub fn run() {
    println!("hello from the print.rs file");
    println!("Number: {}", 1);
    println!("{} is from {}", "Brad", "Mass");
    println!("{} is from {} and {0} likes to {2}", "Brad", "Mass", "code");
    println!("{name} likes to play {activity}", name = "John", activity = "Baseball");
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);
    println!("{:?}", (12, true, "hello"));
    println!("10 + 10 = {}", 10 + 10);
    println!();
}