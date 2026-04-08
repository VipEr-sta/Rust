//use std::io::{self};

// Each value has a single owner
/* move semantics: when you assign a value to another variable, 
the ownership is moved, and the original variable can no longer be used*/

/*Borrowing: 
1. Imutable vs mutable references 
2. ! mutable reference or multiple immutable references at the same time
3. References must always pointing to valid memory
// References are pointers that allow you to access data without taking ownership of it.

*/




use std::vec; 


fn main() {

    // mut makes it mutable
    // name      type          value
    let hello: Vec<i32> = (0..=10).collect();
    // name of vairable is the owner
    println!("{:?}", hello);

    // Variables:
    let x: i32 = 5; // x is the owner of the value 5

    // Tuple
    let (some_char, some_number) = ('a', 42); 
    println!("Character: {}, Number: {}", some_char, some_number);

    // Functions
    fn greet() -> String {
        println!("Hello, world!");
        "Hello, world!".into()
    }

    fn condition(x: i32) -> i32 {
        if x > 10 {
            1
        } else {
            0
        }
    }


let s1: String = String::from("hello");
let s2: String = s1; // s1 is MOVED to s2
println!("{}", s2);
condition(x);
greet();

// Structs
struct Point {
    x: f64,
    y: f64, 
}

let p1: Point = Point { x: 1.0, y: 2.0 };
println!("Point: ({}, {})", p1.x, p1.y);

// matches

let number: i32 = 5;
match number {
    1 => println!("One"),
    2 => println!("Two"),
    3 => println!("Three"),
    _ => println!("Other"), // _ is a catch-all pattern}
}



let result: Option<i32> = Some(42);
// similar to switch statement in other languages but forces explicit handling of all cases
// some is enum for that says it can be either some value or none
// none is the absence of a value
match result {
    Some(value) => println!("Got a value: {}", value),
    None => println!("Got nothing"),
}





// Calculator example

/* 
let mut num1: String = String::new(); // creates mutable string
let mut operator: String = String::new(); // creates mutable string
let mut num2: String = String::new(); // creates mutable string


io::stdin()
    .read_line(&mut num1)
    .expect("Invalid");
println!("you typed: {}", num1.trim()); // trim removes the trailing newline





println!("{}", num1);

io::stdin()
    .read_line(&mut operator)
    .expect("Invalid");
println!("you typed: {}", operator.trim()); // trim removes the trailing newline

io::stdin()
    .read_line(&mut num2)
    .expect("Invalid");
println!("you typed: {}", num2.trim()); // trim removes the trailing newline


match operator.trim() {
    "+" =>
        println!("Answer: {}", add(num1.trim().parse().unwrap(), num2.trim().parse().unwrap())),

    
    "-" => 
        println!("Answer: {}", sub(num1.trim().parse().unwrap(), num2.trim().parse().unwrap())),
    "*" => 
        println!("Answer: {}", mult(num1.trim().parse().unwrap(), num2.trim().parse().unwrap())),
    "/" => 
        println!("Answer: {}", div(num1.trim().parse().unwrap(), num2.trim().parse().unwrap())),
    _ => println!("Invalid operator")
    
}



*/

// loops

// infinite loop

loop {
    println!("This will loop forever");
    break; // to exit the loop



}
let mut i = 0;
while  
    i < 5 {
        println!("i: {}", i);
        i += 1; // increment i


}

let words = vec!["hello", "world", "rust"];

for word in &words {
    println!("Word: {}", word);
}
print!("{:?}", words)
}


/* 

fn add(a: i32, b: i32) -> i32 {
    a+b
}

fn sub(a: i32, b: i32) -> i32{
    a-b
}

fn mult(a: i32, b: i32) -> i32{
    a*b
}

fn div(a: i32, b: i32) -> i32 {
    a/b
}
    */









