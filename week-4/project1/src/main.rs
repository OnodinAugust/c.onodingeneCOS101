// Rust program to calculate the speed of a car in km

use std::io;

fn main() {
    let mut distance1 = String::new();
    let mut time1 = String::new();
    let mut distance2 = String::new();
    let mut time2 = String::new();

    println!("Enter  the first distance:");
    io::stdin().read_line(&mut distance1).expect("Not a valid string");
    let d1:f32 = distance1.trim().parse().expect("Not a valid number");

    println!("Enter  the first time:");
    io::stdin().read_line(&mut time1).expect("Not a valid string");
    let t1:f32 = time1.trim().parse().expect("Not a valid number");

    let d1:f32 = d1 * 1.60934;
    let speed1:f32 = d1 / t1;
    println!("The first speed of the car given distance and time: {}",speed1);
   
    println!("Enter second distance:");
    io::stdin().read_line(&mut distance2).expect("Not a valid string");
    let d2:f32 = distance2.trim().parse().expect("Not a valid number");

    println!("Enter second time: ");
    io::stdin().read_line(&mut time2).expect("Not a valid string");
    let t2:f32 = time2.trim().parse().expect("Not a valid number");

    let d2:f32 = d2 * 1.60934;
    let speed2:f32 = d2 / t2;
    println!("The second speed of the car given distance and time: {}",speed2);

}