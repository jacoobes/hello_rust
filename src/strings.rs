    //primitve str = immutable fixed-length somewhere in memory
    // String = growable, heap-allocated data struct use when u modify or own string data
    pub fn run() {
       let hello= "Hello";
       let mut hello_mutable = String::from(hello);

        //Get length
        println!("Length: {}", hello.len());

        //Push char
        hello_mutable.push('W');
        //Push string
        hello_mutable.push_str("orld");

        //capacity in bytes
        println!("Capacity : {}", hello_mutable.capacity());
        //many methods in string impl

        //Loop through string by whitespace

        for word in hello_mutable.split_whitespace() {
            println!("{}", word)
        }



        //Create a string with capacity
        let mut s = String::with_capacity(10);
        s.push('a');
        s.push('a');
        assert_eq!(2, s.len());
        assert_eq!(11, s.capacity());
        println!("{}", s);
        println!("{}", hello);
    }