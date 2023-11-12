// Rust program to determine the annual incentive of an employee

use std::io;

fn main() {
    
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("You are experienced: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let experienced:bool = input1.trim().parse().expect("Not a valid response");

    println!("Enter your age: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let age:i32 = input2.trim().parse().expect("Not a valid number");

    if age >= 40 && experienced == true {
        println!("Your incentive is N1_560_000");
    } if age >= 30 && age < 40 && experienced == true {
        println!("Your incentive is N1_480_000");
    } if age < 30 && experienced == true {
        println!("Your incentive is N1_300_000");
    } if experienced == false {
        println!("Your incentive is N100_000");
    }
}
