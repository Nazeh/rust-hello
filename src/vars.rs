// Variables are immutable by default
// Block-scoped language

pub fn run (){
    let name = "Nazeh";
    let mut age = 19; // I wish

    println!("My name is {} and I am {}", name, age);
    age = 29;
    println!("My name is {} and I am {}", name, age);

    // Constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("Nazeh", 29);
    println!("My name is {} and I am {}", my_name, my_age);
}