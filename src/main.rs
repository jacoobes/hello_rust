use std::ops::Range;

//mod print;
//mod vars;
//mod types;
//mod strings;
//mod tuples;
//mod arrays;
//mod vectors;
// mod conditions;
// mod loops;
mod functions;

fn main() {
    println!("Hello, world!");
 //   print::run();
 //   vars::run();
 //    types::run();
 //     strings::run();
 //     tuples::run();
 //   arrays::run();
 //   vectors::run()
 //conditions::run();

 // loops::run()
 //functions::run()
 // let greet = String::from("Hola");
 //  test(&greet);
 //  let capacity = greet.capacity();

fizz_buzz(1..100)

}

fn fizz_buzz(range: Range<i64>) {

   for x in range {
       if x % 15 == 0 {
           println!("{}", "fizzbuzz")
       } else if x % 5 == 0 {
           println!("{}", "buzz")
       } else if x % 3 ==0  {
           println!("{}", "fizz")
       } else {
           println!("{}", x)
       }

   }


}

fn test(string : &String) {
    println!("{}", string)
}