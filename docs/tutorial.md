---
title: Simple Calculator Tutorial in Rust
---

# Goal

In this tutorial, you will learn how to create a Calculator that is able to do simple Math.

# Previous Knowledge

We'll assume you dont know how to code or have a slim knowledge of coding.

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
1. *let* declares our Variable
2. *mut* makes the Variable changeable, since UserInput will modify this Variable, we need it to be changeable.
3. *x & y* is the name of our Variable
4. *String::new();* Creates an empty String, so that means the Variable is empty until the user inputs something.

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
1. *io::stdin()*

# Result

# What could go wrong?
