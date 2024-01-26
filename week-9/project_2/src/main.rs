use std::io::Write;

fn main() {

    let name = vec!["Oluchi Mordi", "Adams Aliyu", "Shania Bolade", "Adekunle Gold", "Bianca Edemoh"];
    let matric_number = vec!["ACC10211111", "ECO10110101", "CSC10328828", "EEE11020202", "MEE10202001"];
    let department = vec!["Accounting", "Economics", "Computer Science", "Electrical Engineering", "Mechanical Engineering"];
    let level = vec!["300", "100", "200", "200", "100"];

    let mut file = std::fs::File::create("details.txt").expect("Create failed");

    file.write_all(
        format!("{:^80}\n", "PAU SMIS")
            .as_bytes(),
    )
    .expect("Write failed");
    file.write_all(
        format!("{:^19} {:^19} {:^19} {:20}\n", "Student Name", "Matric. Number", "Department", "Level")
            .as_bytes(),
    )
    .expect("Write failed");

    for i in 0..name.len() {
        let first_item = name[i];
        let second_item = matric_number[i];
        let third_item = department[i];
        let fourth_item = level[i];

        file.write_all(format!("{:^20}",first_item.trim()).as_bytes()).expect("Write failed");
        file.write_all(format!("{:^20}",second_item.trim()).as_bytes()).expect("Write failed");
        file.write_all(format!("{:^20}",third_item.trim()).as_bytes()).expect("Write failed");
        file.write_all(format!("{:^20}",fourth_item.trim()).as_bytes()).expect("Write failed");
        file.write_all("\n".as_bytes()).expect("Write failed");
    }
}