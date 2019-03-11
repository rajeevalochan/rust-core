pub fn run() {
    println!("Print Statemet");
    println!("Number {}", 1);
    //Basic Formatting
    println!("{} is from {}", "Rajeevalochan", "Veppampattu");
    //Positional Arguments
    println!(
        "{0} is from {1} and {0} Loves to {2}",
        "Rajeevalochan", "Veppampatu", "Code"
    );
    //Named Arguments
    println!(
        "{hisName} loves {herName}",
        hisName = "Rajeevalochan",
        herName = "Buji"
    );
    //Placeholder Traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);
    println!("{:?}", (12, true, "Hola"));
}
