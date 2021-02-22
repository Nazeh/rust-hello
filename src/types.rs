pub fn run() {
    // Default of an integer is "i32"
    let x = 1;

    // Default of float is "f64"
    let y = 2.5;

    // Add explicit type
    let z: i64 = 42424242;

    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    let is_active: bool = true;

    // Get boolean form expression
    let is_greater = 10 > 4;

    // Char uses a single quote
    let a1 = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, is_greater, a1, face))
}