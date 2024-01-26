use std::io;

fn main() 
{

    for _ in 0..150 {

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();
    let mut input5 = String::new();
    let mut input6 = String::new();
    let mut input7 = String::new();

    println!("Enter Name: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");

    println!("Enter Email: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");

    println!("Enter Department: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");

    println!("Enter State of Origin: ");
    io::stdin().read_line(&mut input4).expect("Not a valid string");

    println!("You are a Class Rep: ");
    io::stdin().read_line(&mut input5).expect("Not a valid string");
    let class_rep:bool = input5.trim().parse().expect("Not a valid response");

    println!("Enter current level: ");
    io::stdin().read_line(&mut input6).expect("Not a valid string");
    let level:i32 = input6.trim().parse().expect("Not a valid response");

    println!("Enter CGPA: ");
    io::stdin().read_line(&mut input7).expect("Not a valid string");
    let cgpa:f32 = input7.trim().parse().expect("Not a valid response");

    if class_rep == true && level > 100 && cgpa > 4.0 {
        println!("Name: {}Email: {}Department: {}State of Origin: {}",input1,input2,input3,input4);
        println!("You can vote");
    } else {
        println!("Sorry, you are not eligible to vote");
    }
}
}


