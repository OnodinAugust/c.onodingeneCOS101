use std::io;

fn main(){

    let mut input = String::new(); 
    println!("Which formula is required?");
    println!("Area of Trapezium");
    println!("Area of Rhombus");
    println!("Area of Parallelogram");
    println!("Total Surface Area of Cube");
    println!("Volume of Cylinder");

    io::stdin().read_line(&mut input).expect("Invalid input");
    let formula = input.trim();

    if formula == "Area of Trapezium" {
        println!("Area of the trapezium is {}",get_area_of_trapezium());
    } else if formula == "Area of Rhombus" {
        println!("Area of the rhombus is {}",get_area_of_rhombus());
    } else if formula == "Area of Parallelogram" {
        println!("Area of the parallelogram is {}",get_area_of_parallelogram());
    } else if formula == "Total Surface Area of Cube" {
        println!("Total surface area of the cube is {}",get_area_of_cube());
    } else if formula == "Volume of Cylinder" {
        println!("Volume of the cylinder is {}",get_volume_of_cylinder());
    }
}

fn get_area_of_trapezium()->f64 {

let mut input1 = String::new();
println!("Enter height:");
io::stdin().read_line(&mut input1).expect("Not a valid string");
let height_tr:f64 = input1.trim().parse().expect("Invalid input");

let mut input2 = String::new();
println!("Enter base 1:");
io::stdin().read_line(&mut input2).expect("Not a valid string");
let base_1:f64 = input2.trim().parse().expect("Invalid input");

let mut input3 = String::new();
println!("Enter base 2:");
io::stdin().read_line(&mut input3).expect("Not a valid string");
let base_2:f64 = input3.trim().parse().expect("Invalid input");

let mut area:f64 = (height_tr / 2.0) * (base_1 + base_2);
return area;
}

fn get_area_of_rhombus()->f64 {

let mut input4 = String::new();
println!("Enter diagonal 1:");
io::stdin().read_line(&mut input4).expect("Not a valid string");
let diagonal_1:f64 = input4.trim().parse().expect("Invalid input");

let mut input5 = String::new();
println!("Enter diagonal 2:");
io::stdin().read_line(&mut input5).expect("Not a valid string");
let diagonal_2:f64 = input5.trim().parse().expect("Invalid input");

let mut area:f64 = (1.0 / 2.0) * diagonal_1 * diagonal_2;
return area;
}

fn get_area_of_parallelogram()->f64 {

    let mut input6 = String::new();
    println!("Enter base:");
    io::stdin().read_line(&mut input6).expect("Not a valid string");
    let base:f64 = input6.trim().parse().expect("Invalid input");

    let mut input7 = String::new();
    println!("Enter altitude:");
    io::stdin().read_line(&mut input7).expect("Not a valid string");
    let altitude:f64 = input7.trim().parse().expect("Invalid input");

    let mut area:f64 = base * altitude;
    return area;
}

fn get_area_of_cube()->f64 {

    let mut input8 = String::new();
    println!("Enter length:");
    io::stdin().read_line(&mut input8).expect("Not a valid string");
    let length:f64 = input8.trim().parse().expect("Invalid input");

    let mut area = 6.0 * length.powf(2.0);
    return area;
}

fn get_volume_of_cylinder()->f64 {

    let mut input9 = String::new();
    println!("Enter radius:");
    io::stdin().read_line(&mut input9).expect("Not a valid string");
    let radius:f64 = input9.trim().parse().expect("Invalid input");

    let mut input10 = String::new();
    println!("Enter height:");
    io::stdin().read_line(&mut input10).expect("Not a valid string");
    let height_cy:f64 = input10.trim().parse().expect("Invalid input");

    let mut area = (22.0 / 7.0) * radius.powf(2.0) * height_cy;
    return area;
}