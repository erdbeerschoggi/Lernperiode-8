---
title: Simple Calculator Tutorial in Rust
---

# Goal

In this tutorial, you will learn how to create a Calculator that is able to do simple Math.

# Previous Knowledge

We'll assume you have a slim knowledge of coding.

# What you'll learn

You will learn how to build a simple Calculator in Rust. While doing this Tutorial you will also learn basic concepts because as I said before we are assuming that u have none to very little coding knowledge.

# Tutorial
In order for out calculator to work we need to be able to use User Input and User Output, otherwise our Calculator wouldnt really make sense. In order for us to be able to use User Input and User Output we need to write the following statement at the very beginning:
```rust
use std::io;
```
This code statement basically tells Rust that we are using input and output tools from Rusts standard library. Without this user Input and Output wouldnt be possible.

Now we have to add the main Function so we can define the programs starting point. 
```rust
fn main ()
```

Now we are going to start writing the actual Code so our Calculator will work.
First thing we do is write the Variables. Because if we dont declare them now we wont be able to work with anything. Varibales are essential for this Calculator to work, because they are going to hold our user input.
With this being a simple Calculator, it should only be able to Calculate with 2 numbers. Thats why we declare 2 Variables that will stand for this number: x and y. 
```rust
let mut x = String::new();
let mut y = String::new();
```
Lets break this down: 
1. ***let*** declares our Variable
2. ***mut*** makes the Variable changeable, since UserInput will modify this Variable, we need it to be changeable.
3. ***x & y*** is the name of our Variable
4. ***String::new();*** Creates an empty String, so that means the Variable is empty until the user inputs something.

Now we created both Variables for the 2 Numbers that the user will input. But thats not all the Variables that we will need. We also need a Variable that will hold the operator choice. So we basically let the User choose which one of the 4 operations they would like to use: (1) Addition, (2) Subtract, (3) Multiply or (4) Divide. Now to hold the Users operator choice we also need a Variable so we just follow the principles from the other 2 Variables.
```rust
let mut op = String::new();
```

Now we just write the Menu for choosing the operators. For the Console to be able to Output Text we just use the statement println!();
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
Obviously just the Output on the Console wont make anything work yet but it just helps to know what we have to do.

Now the most important thing. Our Calculator is supposed to be able to read User Input and for it to do this we need to code the following statement:
```rust
io::stdin().read_line(&mut op).expect("Invalid Input");
```
lets break this line of code down: 
1. ***io::stdin()*** This statement grants us access to the UserInput and Ouput function
2. ***.read_line*** This reads what the User writes. So if the user types in 3 then it reads the number 3.
3. ***(&mut op)*** The & References to what the User writes, mut lets the text that the user writes modiy the variable op.
4. ***.expect("Invalid Input");*** If the Input fails the program will just give us a message "Invalid Input" to let us know that its nto working.

Now we ask the User to type in his 2 numbers that he would like to use with the specific operation they chose before. For those 2 Numbers we also repeat the process of reading the User Input just like before. This Snippet of code should look like this:
```rust
println!("Enter your first number:");
io::stdin().read_line(&mut x).expect("Invalid Input");

println!("Now enter your second number:");
io::stdin().read_line(&mut y).expect("Invalid Input");
```

If you look at the Variables we declared before, youll see that we made them an empty string. Now if you know what a string is then you know that the string datatype only holds text but we are using Numbers so we have to convert the string datatype into a number datatype. In order to do this we write the following:
```rust
let x: f64 = x.trim().parse().expect("Error in Converting");
let y: f64 = y.trim().parse().expect("Error in Converting");
```
Lets break it down:
1. ***x: f64*** Converts the 64-bit floating number, usually if you code in rust you dont specifically have to define it because rust just recognizes it and adds it in itself.
3. ***x.trim()*** when you press enter a new line is created so the converting will definetly work.
4. ***.parse()*** Converts the Datatype
5. ***.expect("Error in Converting");*** If the onverting doesnt work the programm would crash and give us the message ""Error in Converting"

Here we just remove the new line so its easier to compare, because with the operators we dont have to change it into a different Datatype.
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
lets break this down: 
1. ***let result: f64 = if op == "1" {
        x + y
    }*** This snippet lets us return the value (Result). So we basically create the variable result and now we start the if statement. Here we see that if op == 1 then it should add up the numbers. Because our first operator was addition. Now we continue that logic for the rest of the calculations. So if the user inputs 2 (for the operator they chose) the numbers should subtract and this logic works for all of the 4 Operators.
2. ***else {
        println!("Invalid Operator Selected");
        0.0 
    };*** Now the last else statement should give us back the result. If our code has a mistake and we cant get the result we should get a console output of "Invalid Operator Selected" and if our result works we should get a number. The 0.0 gets used as a placeholder for the number of our result

Now our Calculator has all of the logic it needs, we just have to print the result on our Console:
```rust
println!("Your result is: {}", result);
```
There where the wavy brackets are, is where our result will be shown. And so rust knows that we want the result in those bracktes we just type a comma and then the variable result. 

Now our Code is done and if you followed this Tutorial you should end up with the Code looking somewhat like this:
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
1. If you type cargo run in the terminal to make your code run please make sure you have to write code open, or else it might not run at all or run the wrong code.
2. Make sure you definetly have this ***use std::io;*** statement at the very start of the code, if you dont add this you wont be able to use user input or user output.
3. A mistake I kept doing at the start was writing printIn(); and not println!(); If you look closely youll see that one letter is an I and the other is a l. Just please make sure you use write letter because I kept writing it wrong and had no idea why my code didnt work.
