fn main() {
    //Trying out simple commands in rust
    println!("Welcome to my first rust code!!!!");
    print!("This is a simple Rust program.");
    print!("Hello, Rustaceans!");

    // Variables
    let x: &str ="Bread";
    let y: &str="Butter";
    println!("Can you pass me the {} for my {}", y, x);

    // Changing the Variable
    let mut x: i32 = 5;
    println!("The value of x is: {}", x);
    x = x + 1;
    println!("The new value of x is: {}", x);

    // Varibales with different data types
    let first: &str="Hi";
    let second: i32=42;
    let third: f64=3.14;
    let fourth: char='Z';
    let fifth: bool=true;

    println!("{}, {}, {}, {}, {}", first, second, third, fourth, fifth);

    // Constants and Boolean Logic
    const BIRTH_YEAR: i32 = 1990;
    println!("I was born in the year: {}", BIRTH_YEAR);

     let logged_in = true;
     let is_admin = false;

    println!("Is regular user: {}", logged_in && !is_admin);

    // Comparison Operators
    let age =21;
    let can_vote = age >= 18;
    println!("Can vote: {}", can_vote);

    // if-else Statements
    let mut temperature = 30;
    temperature -= 10;
    if temperature > 25 {
        println!("Its a hot day");
    }else {
        println!("Its a cold day");
    }

    //idk
    let time = 20;
    let greeting = if time < 18 {
    "Good day."
    } else {
    "Good evening."
    };
    println!("{}", greeting);
    
    // User Input
    use std::io;
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    println!("{}", input);

}
