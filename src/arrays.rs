//Arrays - Fixed list where elms are same data types
use std::mem;

pub fn run() {
    let numbers : [i32; 4] = [1,2,3,4];

    //Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers) );

    println!("{:?}", numbers);
    println!("{}", numbers[0]);

    //get slice

    let slice : &[i32] = &numbers[0..2];
    println!("Slice {:?}", slice)
}