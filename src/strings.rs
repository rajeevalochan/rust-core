pub fn run() {
    let mut hello = String::from("Hello ");
    //length of the string
    println!("Length: {}", hello.len());

    //Push a character into the String
    // hello.push("W");

    //push a strng
    hello.push_str("orld!");

    //Capacity o string
    println!("Capacity {}", hello.capacity());

    //Check String isEmpty
    println!("Sting isEmpty{}", hello.is_empty());

    //string contains a given word
    //Case sensitive
    println!("Contains Hello {}", hello.contains("Hello"));

    //Loop string over the white spaces
    for word in hello.split_whitespace() {
        println!("split with whitePaces: {}", word);
    }

    //push a word into the

    println!("{}", hello)
}
