// Arrays - fixed list where elements are the same data types

use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3 ,4 ,5];
    
    // Reassign value
    numbers[2] = 20;

    println!("{:?}", numbers);

    // Get single value
    println!("Single Value: {}", numbers[0]);

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice)
}