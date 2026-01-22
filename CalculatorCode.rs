use std::io;

fn main() {
    // Variables to hold user input
    let mut x = String::new();
    let mut y = String::new();
    let mut op = String::new();

    println!("Welcome to this simple Calculator!");

    //Choosing the operator
    println!("Which operator would you like to use?");
    println!("(1) Addition");
    println!("(2) Subtract");
    println!("(3) Multiply");
    println!("(4) Divide");
    println!("Select the number associated with the desired operation: "); 
    // Reading the user input
    io::stdin().read_line(&mut op).expect("Invalid Input");

    println!("Enter your first number:");
    io::stdin().read_line(&mut x).expect("Invalid Input");

    println!("Now enter your second number:");
    io::stdin().read_line(&mut y).expect("Invalid Input");
    
    // Converting the string inputs into f64 because for this to work we need to do math, and with the string type we cant save numbers
    let x: f64 = x.trim().parse().expect("Error in Converting");
    let y: f64 = y.trim().parse().expect("Error in Converting");
    let op = op.trim();


    // Performing the operation based on user input
    let result: f64 = if op == "1" {
        x + y
    }
    else if op == "2" {
        x - y
    }
    else if op == "3" {
        x * y
    }
    else if op == "4" {
        x / y
    }
    else {
        println!("Invalid Operator Selected");
        0.0 // Das braucht man um einen Wert zurückzugeben, auch wenn der Operator ungültig ist. Das ist nur ein Platzhalter.
    };

    //Resultat wird wiedergegeben
    println!("Your result is: {}", result);

}
