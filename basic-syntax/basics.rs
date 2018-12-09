// This is a comment and will be ignored by the compiler
// This is the main function
fn main() { 
    // println! is a macro that prints text to the console
    println!("Hello World!");

    // This is a variable declaration
    let n = 10;

    // if-else is similar to other languages
    // However, the boolean condition doesn't need to be surrounded by parentheses
    if n < 0 {
        println!("{} is a negative number.", n);
    }
    else if n > 0 {
        println!("{} is a positive number.", n);
    }
    else {
        println!("{} is zero.", n);
    }

    // There are several ways of looping in Rust
    // here's an example of using for loop
    for a in 0..10 {
        println!("Current value : {}", a);
    }
}
