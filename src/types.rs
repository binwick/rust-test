/*
u8,i8...
f32, f64
bool
char


*/
pub fn run() {
    // i32
    let x = 1;
    // f64
    let y = 2.5;

    let z: i64 = 23234234234;

    // find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    let is_active = true;
    let is_greater = 10 > 5;
    let a1 = 'a';
    let face = '\u{1f600}';

    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}