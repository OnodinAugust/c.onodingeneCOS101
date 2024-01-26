use std::io;

fn main() {

    let mut input1 = String::new();
    println!("Name:");
    io::stdin().read_line(&mut input1).expect("Not a valid input");
    let name = input1.trim();

    let mut input2 = String::new();
    println!("Age:");
    io::stdin().read_line(&mut input2).expect("Not a valid input");
    let age:i32 = input2.trim().parse().expect("Not a valid integer");

    let mut input3 = String::new();
    println!("Occupation:");
    io::stdin().read_line(&mut input3).expect("Not a valid input");
    let occupation = input3.trim();

    let mut input4 = String::new();
    println!("Years of experience:");
    io::stdin().read_line(&mut input4).expect("Not a valid input");
    let years_of_exp:i32 = input4.trim().parse().expect("Not a valid integer");

    let mut staff_level = "";
    let input4 = years_of_exp.to_string();
    let input2 = age.to_string();

    let staff_details = vec![name, &input2, occupation, &input4];

    if occupation == "Office Administrator" {
        if years_of_exp >0 && years_of_exp <=2 {
            let  level = "Intern";
            println!("Your APS level is {}", level);
        } else if years_of_exp >2 && years_of_exp <=5 {
            let level = "Administrator";
            println!("Your APS level is {}", level);
        } else if years_of_exp >4 && years_of_exp <=8 {
            let level = "Senior Administrator";
            println!("Your APS level is {}", level);
        } else if years_of_exp >7  && years_of_exp <=10 {
            let level = "Office Manager";
            println!("Your APS level is {}", level);
        } else if years_of_exp >9 && years_of_exp <=13 {
            let level = "Director";
            println!("Your APS level is {}", level);
        } else if years_of_exp >13 {
            let level = "CEO";
            println!("Your APS level is {}", level);
        }
        println!("{:?}\n", staff_details);

    } else if occupation == "Academic"{
        if years_of_exp >0 && years_of_exp <=2 {
            let level = "Rookie";
            println!("Your APS level is {}", level);
        } else if years_of_exp >2 && years_of_exp <=5 {
            let level = "Research Assistant";
            println!("Your APS level is {}", level);
        } else if years_of_exp >4 && years_of_exp <=8 {
            let level = "PhD Candidate";
            println!("Your APS level is {}", level);
        } else if years_of_exp >7  && years_of_exp <=10 {
            let level = "Post-doc Researcher";
            println!("Your APS level is {}", level);
        } else if years_of_exp >9 && years_of_exp <=13 {
            let level = "Senior Lecturer";
            println!("Your APS level is {}", level);
        } else if years_of_exp >13 {
            let level = "Dean";
            println!("Your APS level is {}", level);
        }
        println!("{:?}\n", staff_details);

    } else if occupation == "Lawyer" {
        if years_of_exp >0 && years_of_exp <=2 {
            let level = "Paralegal";
            println!("Your APS level is {}", level);
        } else if years_of_exp >2 && years_of_exp <=5 {
            let level = "Junior Associate";
            println!("Your APS level is {}", level);
        } else if years_of_exp >4 && years_of_exp <=8 {
            let level = "Associate";
            println!("Your APS level is {}", level);
        } else if years_of_exp >7  && years_of_exp <=10 {
            let level = "Senior Associate 1-2";
            println!("Your APS level is {}", level);
        } else if years_of_exp >9 && years_of_exp <=13 {
            let level = "Senior Associate 3-4";
            println!("Your APS level is {}", level);
        } else if years_of_exp >13 {
            let level = "Partner";
            println!("Your APS level is {}", level);
        }
        println!("{:?}\n", staff_details);

    } else if occupation == "Teacher" {
        if years_of_exp >0 && years_of_exp <=2 {
            let level = "Placement";
            println!("Your APS level is {}", level);
        } else if years_of_exp >2 && years_of_exp <=5 {
            let level = "Classroom Teacher";
            println!("Your APS level is {}", level);
        } else if years_of_exp >4 && years_of_exp <=8 {
            let level = "Senior Teacher";
            println!("Your APS level is {}", level);
        } else if years_of_exp >7  && years_of_exp <=10 {
            let level = "Leading Teacher";
            println!("Your APS level is {}", level);
        } else if years_of_exp >9 && years_of_exp <=13 {
            let level = "Deputy Principal";
            println!("Your APS level is {}", level);
        } else if years_of_exp >13 {
            let level = "Principal";
            println!("Your APS level is {}", level);
        }
        println!("{:?}\n", staff_details);
    }
}