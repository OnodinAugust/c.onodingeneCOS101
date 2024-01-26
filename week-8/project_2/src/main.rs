use std::io;

fn details(name: &mut Vec<String>, yrs_of_exp: &mut Vec<usize>) {

    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Name:");
    io::stdin().read_line(&mut input1).expect("Not a valid input");
    let dev_name = input1.trim();
    name.push(dev_name.to_string());

    println!("Years of experience:");
    io::stdin().read_line(&mut input2).expect("Not a valid input");
    let experience:usize = input2.trim().parse().expect("Not valid");
    yrs_of_exp.push(experience);
}

fn main() {

    let mut name: Vec<String> = Vec::new();
    let mut yrs_of_exp: Vec<usize> = Vec::new();

    let mut input3 =String::new();
    println!("Number of interview candidates:");
    io::stdin().read_line(&mut input3).expect("Not a valid input");
    let no_of_candidates:usize = input3.trim().parse().expect("Not valid");

    for i in 0..no_of_candidates {
        details(&mut name, &mut yrs_of_exp);
    }

    let most_yrs_of_exp = checker(no_of_candidates, &yrs_of_exp);
    println!("\nThe most experienced interview candidate is {} with {} years of experience.",name[most_yrs_of_exp], yrs_of_exp[most_yrs_of_exp]);
}

fn checker(input4:usize, experience: &Vec<usize>)->usize {
    let mut yrs_of_exp = 0;
    let mut max_yrs_of_exp = 0;
    for i in 0..input4 {
        if experience[i] > yrs_of_exp {
            yrs_of_exp = experience[i];
            max_yrs_of_exp = i;
        }
    }
    return max_yrs_of_exp;
}