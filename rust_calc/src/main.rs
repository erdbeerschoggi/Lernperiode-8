use std::io;

fn main() {
    let mut x = String::new();
    let mut y = String::new();
    let mut op = String::new();

    println!("Welcome to this simple Calculator!");

    println!("Which operator would you like to use?");
    println!("(1) Addition");
    println!("(2) Subtract");
    println!("(3) Multiply");
    println!("(4) Divide");
    println!("Select the number associated with the desired operation: "); 
    io::stdin().read_line(&mut op).expect("Invalid Input");

    println!("Enter your first number:");
    io::stdin().read_line(&mut x).expect("Invalid Input");

    println!("Now enter your second number:");
    io::stdin().read_line(&mut y).expect("Invalid Input");
    
    println!("Your Result is: {} {} {}", x, y, op);

}
