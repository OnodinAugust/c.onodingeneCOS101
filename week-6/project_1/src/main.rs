
Certainly! Below is an example Rust program that uses nested for loops to display the multiplication table vertically from 1 to n:

rust
Copy code
use std::io;

fn main() {
    // Read the value of n from the user
    let mut input = String::new();
    println!("Enter the value of n: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: u32 = input.trim().parse().expect("Please enter a valid number");

    // Display the multiplication table vertically
    println!("Multiplication Table from 1 to {}", n);

    // Outer loop for rows
    for i in 1..=10 {
        // Inner loop for columns
        for j in 1..=n {
            // Calculate and display the product
            let product = i * j;
            print!("{:4}", product); // Adjust the width as needed
        }
        // Move to the next line after each row
        println!();
    }
}