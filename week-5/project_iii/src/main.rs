// Rust program to determine the annual incentive of an employee

use std::io;

fn main() {

    println!("\t\tMenu\t\t\tPrice");
    println!("P = Poundo Yam/Edinkaiko Soup\t\tN3200");
    println!("F = Fried Rice & Chicken\t\tN3000");
    println!("A = Amala & Ewedu Soup\t\t\tN2500");
    println!("E = Eba & Egusi Soup\t\t\tN2000");
    println!("W = White Rice & Stew\t\t\tN2500");
    
    let mut food = String::new();
    let mut total_order:f32 = 0.00;
    let mut input = String::new();

    println!("Enter your order: ");
    io::stdin().read_line(&mut food).expect("Not a valid string");
    let food = food.trim();

    println!("Enter your quantity: ");
    io::stdin().read_line(&mut input).expect("Not a valid string");
    let quantity:f32 = input.trim().parse().expect("Not a valid number");

    if food == "P" {
        total_order += 3200.00 * quantity;
    } else if food == "F" {
        total_order += 3000.00 * quantity;
    } else if food == "A" {
         total_order += 2500.00 * quantity;
    } else if food == "E" {
        total_order += 2000.00 * quantity;
    } else if food == "W" {
        total_order += 2500.00 * quantity;
    }

    let total_order_after_discount = total_order * (95.00 / 100.00);

    if total_order < 10000.00 {
        println!("The total order is {}",total_order);
    } if total_order > 10000.00 {
        println!("The total order is {}",total_order_after_discount);
    }    

}
