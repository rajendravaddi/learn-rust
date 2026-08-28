/*
Variables
- Variables are containers for storing data values (characters, numbers etc)
- In Rust, variables are immutable by default. This means that once a variable is declared, it cannot be changed.

*/
fn main() {
    // - To create a variable, use the let keyword:
    let x = 10;
    println!("x = {}",x);

    /* 
    variables in rust are immutable by default
    let y = 10;
    y = 20; -> causes an error

    to declare mutable variables we need to use `mut` keyword
    */
    let mut y = 10;
    println!("y before = {}",y);
    y = 20;
    println!("y after = {}",y);

    /* 
    shadowing
    shadowing is a feature in Rust that allows you to redeclare a variable with the same name, and the new variable will shadow the previous one. 
    */
    let x = 10;
    let x = x + 1;
    let x = x * 2;
    println!("x = {}",x);

    /*
    Placeholder - {
    while printing variables using print macros we need to use placeholder {}
    println!("Hello {}", x);
    */
}
