//Rust Program to develop a system that performs the Student Council Voting system.

use std::io;

fn main() {
    for x in 0..150{
    println!("Student Council Voter System");
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
     
    // input whether candidate is a class rep or not a class rep     
    println!("\n Please enter if your the current 'class rep' or not the 'class rep' :");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let candidate_status:String = input1.trim().parse().expect("Invalid status");

    //input whether candidate is in 100 level or not in 100 level
    println!("\n Please enter if you are in '100 level' or not in '100 level' :");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let candidate_level:i32 = input2.trim().parse().expect("Invalid level");

    //input whether candidate CGPA is above 4.0 or candidate CGPA is below 4.0
    println!("\n Please enter if your CGPA :");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let candidate_CGPA:f64 = input3.trim().parse().expect("Invalid input");

    //input name of the candidate
    println!("\nPlease enter your name :");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");

    //input email of address of the candidate
    println!("\nPlease enter your email address");
    let mut email_address = String::new();
    io::stdin().read_line(&mut email_address).expect("Failed to read input");

    //input candidate department
    println!("\nPlease enter your department") ;
    let mut department = String::new();
    io::stdin().read_line(&mut department).expect("Failed to read input");

    //input candidate state fo origin
    println!("\nPlease enter your state of origin");
    let mut state_of_origin = String::new();
    io::stdin().read_line(&mut state_of_origin).expect("Failed to read input");

    if candidate_status == "class rep" && candidate_level > 100 && candidate_CGPA >= 4.0
{
    println!("Candidate Information");
    println!("{}", name);
    println!("{}", email_address);
    println!("{}", department);
    println!("{}", state_of_origin);
    println!("You can vote");
}
    else
{
    println!("Sorry, but you are not eligible to vote");
}

    println!("count {}", x)
    }
}
