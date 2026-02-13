---
title: Simple Calculator Tutorial in Rust
---

# Goal

In this tutorial, you will learn how to create a calculator that is able to do simple math.

# Previous Knowledge

We'll assume you have a slim knowledge of coding.

# What you'll learn

You will learn how to build a simple calculator in Rust. While doing this tutorial, you will also learn basic concepts because, as I said before, we are assuming that you have little to no coding knowledge.

# Tutorial
In order for our calculator to work, we need to be able to use user input and user output; otherwise, our calculator wouldn't really make sense. In order for us to be able to use user input and user output, we need to write the following statement at the very beginning:
```rust
use std::io;
```
This code statement basically tells Rust that we are using input and output tools from Rust's standard library. Without this user input and output, it wouldn't be possible.

Now we have to add the main function so we can define the program's starting point. 
```rust
fn main ()
```

Now we are going to start writing the actual code so our calculator will work.
The first thing we do is write the variables. Because if we don't declare them now, we won't be able to work with anything. Variables are essential for this calculator to work because they are going to hold our user input.
With this being a simple calculator, it should only be able to calculate with 2 numbers. That's why we declare 2 variables that will stand for this number: x and y. 
```rust
let mut x = String::new();
let mut y = String::new();
```
Let's break this down: 
1. ***let*** declares our Variable
2. ***mut*** makes the variable changeable. Since UserInput will modify this variable, we need it to be changeable.
3. ***x & y*** is the name of our Variable
4. ***String::new();*** Creates an empty String, so that means the variable is empty until the user inputs something.

Now we created both variables for the 2 numbers that the user will input. But that's not all the variables that we will need. We also need a variable that will hold the operator's choice. So we basically let the user choose which one of the 4 operations they would like to use: (1) addition, (2) subtraction, (3) multiplication, or (4) division. Now to hold the Users operator choice, we also need a variable, so we just follow the principles from the other 2 variables.
```rust
let mut op = String::new();
```

Now we just write the menu for choosing the operators. For the console to be able to output text, we just use the statement println!();
```rust
println!("Welcome to this simple Calculator!");

//Choosing the operator
println!("Which operator would you like to use?");
println!("(1) Addition");
println!("(2) Subtract");
println!("(3) Multiply");
println!("(4) Divide");
println!("Select the number associated with the desired operation: ");

```
Obviously, just the output on the console won't make anything work yet, but it just helps to know what we have to do.

Now the most important thing. Our calculator is supposed to be able to read user input, and for it to do this, we need to code the following statement:
```rust
io::stdin().read_line(&mut op).expect("Invalid Input");
```
Let's break this line of code down: 
1. ***io::stdin()*** This statement grants us access to the UserInput and output function.
2. ***.read_line*** This reads what the user writes. So if the user types in 3, then it reads the number 3.
3. ***(&mut op)*** The & references what the user writes, and mut lets the text that the user writes modify the variable op.
4. ***.expect("Invalid Input");*** if the input fails, the program will just give us a message, "Invalid Input" to let us know that it's not working.

Now we ask the user to type in his 2 numbers that he would like to use with the specific operation they chose before. For those 2 numbers, we also repeat the process of reading the user input just like before. This snippet of code should look like this:
```rust
println!("Enter your first number:");
io::stdin().read_line(&mut x).expect("Invalid Input");

println!("Now enter your second number:");
io::stdin().read_line(&mut y).expect("Invalid Input");
```

If you look at the variables we declared before, you'll see that we made them an empty string. Now if you know what a string is, then you know that the string datatype only holds text, but we are using numbers, so we have to convert the string datatype into a number datatype. In order to do this, we write the following:
```rust
let x: f64 = x.trim().parse().expect("Error in Converting");
let y: f64 = y.trim().parse().expect("Error in Converting");
```
Let's break it down:
1. ***x: f64*** Converts the 64-bit floating number. Usually if you code in Rust, you don't specifically have to define it because Rust just recognizes it and adds it in itself.
3. ***x.trim(): When you press enter, a new line is created, so the converting will definitely work.
4. ***.parse()*** converts the datatype
5. ***.expect("Error in Converting");*** if the converting doesn't work, the program would crash and give us the message ""Error in Converting"

Here we just remove the new line so it's easier to compare, because with the operators we don't have to change it into a different Datatype.
```rust
let op = op.trim();
```

Now for our Calculator to actually work we have to code the logic behind it.
```rust
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
        0.0 //You need this for the result to return. This is just a placeholder.
    };
```
Let's break this down: 
1. ***let result: f64 = if op == "1" {
        x + y
    }*** This snippet lets us return the value (Result). So we basically create the variable result, and now we start the if statement. Here we see that if op == 1, then it should add up the numbers. Because our first operator was addition. Now we continue that logic for the rest of the calculations. So if the user inputs 2 (for the operator they chose), the numbers should subtract, and this logic works for all 4 operators.
2. ***else {
        println!("Invalid Operator Selected");
        0.0 
    };*** Now the last else statement should give us back the result. If our code has a mistake and we can't get the result we should get a console output of "Invalid Operator Selected" and if our result works, we should get a number. The 0.0 gets used as a placeholder for the number of our result.

Now our calculator has all the logic it needs; we just have to print the result on our console:
```rust
println!("Your result is: {}", result);
```
There, where the wavy brackets are, is where our result will be shown. And so Rust knows that we want the result in those brackets; we just type a comma and then the variable result. 

Now our code is done, and if you followed this tutorial, you should end up with the code looking somewhat like this:
```rust
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
```

Now in order for our code to run, we have to open the terminal and type the command cargo run. 

# Result

![RustCalculator (1)](https://github.com/user-attachments/assets/32c53523-cc5b-479c-b036-9b64d902dca2)


# What could go wrong?
1. If you type cargo run in the terminal to make your code run, please make sure you write code to open, or else it might not run at all or run the wrong code.
2. Make sure you definitely have this ***use std::io;*** statement at the very start of the code; if you don't add this, you won't be able to use user input or user output.
3. A mistake I kept doing at the start was writing printIn(); and not println!(); If you look closely, you'll see that one letter is an I and the other is an l. Just please make sure you write letter because I kept writing it wrong and had no idea why my code didn't work.
