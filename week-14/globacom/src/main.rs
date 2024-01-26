use std::io;
use std::io::Read;

fn main() {
    let mut input1 = String::new();
    println!("Enter criteria: ");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let criteria = input1.trim();

    if criteria == "administrator" {
        let mut file_admin = std::fs::File::open(r"C:\Users\USER\Desktop\globacom_dbase.sql").unwrap();
        let mut contents_admin = String::new();
        file_admin.read_to_string(&mut contents_admin).unwrap();
        print!("{}", contents_admin);
    } else if criteria == "project manager" {
        let mut file_proj = std::fs::File::open(r"C:\Users\USER\Desktop\Project_tb.sql").unwrap();
        let mut contents_proj = String::new();
        file_proj.read_to_string(&mut contents_proj).unwrap();
        print!("{}", contents_proj);
    } else if criteria == "employee" {
        let mut file_emp = std::fs::File::open(r"C:\Users\USER\Desktop\staff_tb.sql").unwrap();
        let mut contents_emp = String::new();
        file_emp.read_to_string(&mut contents_emp).unwrap();
        print!("{}", contents_emp);
    } else if criteria == "customer" {
        let mut file_cust = std::fs::File::open(r"C:\Users\USER\Desktop\Customer_Table_tb.sql").unwrap();
        let mut contents_cust= String::new();
        file_cust.read_to_string(&mut contents_cust).unwrap();
        print!("{}", contents_cust);
    } else if criteria == "vendor" {
        let mut file_vend = std::fs::File::open("dataplan_tb.sql").unwrap(); // Check the correct path
        let mut contents_vend = String::new();
        file_vend.read_to_string(&mut contents_vend).unwrap();
        print!("{}", contents_vend);
    } else {
        print!("Not a valid criteria");
    }
}
