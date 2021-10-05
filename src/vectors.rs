//Vectors are resizable arrays

use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1,2,3,4];

    numbers.push(5);
    numbers.push(6);

    //Vecs are stack allocated
    println!("Vec occupies {} bytes", mem::size_of_val(&numbers) );

    println!("{:?}", numbers);
    println!("{}", numbers[0]);

    //get slice

    let slice : &[i32] = &numbers[0..2];
    println!("Slice {:?}", slice);


    //loop thru vector values

    for x in numbers.iter_mut() {
        *x *= 2
    }
    println!("Numbers Vec : {:?}", numbers);
}