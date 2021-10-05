/*
Primitives
Ints -> u8, i8, u16, i16, u32, i32, u64. i64. u128. i128 (number o bits taken in memory)
Floats : f32, f64
Boolean : bool
Characters : char
Tuples,
Arrays (immutable)

 */

pub fn run() {
    //Default is i32
    let x = 1;
    //Default is f64
    let y = 2.5;

    //Add explicit type
    let z : i64 = 23498094;

    //find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    let is_active = true;

    let is_greater : bool = 10 < 5;
    let face = '\u{1F600}';

    let a1 = 'a';
    println!("{:?}", (  x,y,z,is_active, face) );
}