// Vectors are resizable arrays

use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3 ,4 ,5];
    
    // Reassign value
    numbers[2] = 20;
    println!("{:?}", numbers);

    // Push to vector
    numbers.push(5);
    numbers.push(6);

    // Pop
    numbers.pop();

    // Get length
    println!("Vector Length: {}", numbers.len());

    // Get single value
    println!("Single Value: {}", numbers[0]);

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

    // Loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop & mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Numbers Vec: {:?}", numbers)
}