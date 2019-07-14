use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    numbers[2] = 23;

    println!("{:?}", numbers);
    println!("Single value: {}", numbers[0]);
    println!("len: {}", numbers.len());
    println!("Array occupies {} bytes", mem::size_of_val(&numbers)); // eq 20

    let slice: &[i32] = &numbers[0..3];
    println!("Slice:{:?}", slice);
}