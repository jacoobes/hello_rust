//vars hold prim data or references to data
// variables are immutable by default
//rust is block scoped
pub fn run() {
    let name = "Brad";
    let age = 37;
    println!("My name is {}, and I am {}", name, age);

    //Define constants
    // type has to be explicitly stated.
    const ID : i32 = 001;
    println!("ID {}", ID);

    //assign multiple at once
    let ( my_name, my_age ) = ("Brad", 37);
    println!("{} is {}", my_name, my_age);
}