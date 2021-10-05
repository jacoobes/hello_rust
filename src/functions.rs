pub fn run() {
   let greet = String::from("gello");
    println!("{}", pooba(greet.as_str(), " World"));

    println!("a");
}

fn greeting(greet: String, name: &str) {
 println!("{} {}, nice to meet you", greet, name);
}

fn pooba(greet: & str, name: & str) -> String {
    let mut greeting =  String::from(greet);

    greeting.push_str(name);

    return greeting
}