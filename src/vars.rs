pub fn run() {
    let name = "Rajeevalochan";
    let mut age = 24;
    println!("My name is {} and I am {}", name, age);
    age = 25;
    println!("My name is {} and I am {}", name, age);

    //Define Constant
    const ID: i32 = 001;
    println!("ID {}", ID);

    //Assign multiple vars
    let (my_name, my_age) = ("Rajeevalochan", 25);
    println!("Name {}, my_age {}", my_age, my_name);
}
