pub fn run() {
//    let hello = "hello";
    let mut hello = String::from("hello ");
    println!("Length: {}", hello.len());// size

    hello.push('W');
    hello.push_str("orld");

    println!("Capacity: {}", hello.capacity());// bytes
    println!("Is Empty: {}", hello.is_empty());
    println!("Contains: 'World' {}", hello.contains("World"));
    println!("Replace: {}", hello.replace("World", "There"));
    for word in hello.split_whitespace() {
        println!("{}", word);
    }
    println!("{}", hello);

    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    println!("{}", s);

    assert_eq!(2, s.len());
}