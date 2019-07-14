pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

    numbers[2] = 23;

    numbers.push(5);

    numbers.push(6);

    numbers.pop();


    let slice: &[i32] = &numbers[0..3];
    println!("Slice:{:?}", slice);

    for x in numbers.iter() {
        println!("number: {}", x)
    }

    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Numbers Vec: {:?}", numbers);
}