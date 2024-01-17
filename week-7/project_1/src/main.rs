use std::io;

    fn main(){
    println!("Area of Trapezium {}",get_trapeziumarea());
    let get_trapeziumarea:f64 = (a / 2 * (b + c));
}


fn main() {
    let mut input1 = String::new();
    println!("Enter the height of the trapezium:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let a:i32 = input1.trim().parse().expect("Invalid input");

    let mut input2 = String::new();
    println!("Enter base1 of the trapezium:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b:i32 = input2.trim().parse().expect("Invalid input");

    let mut input3 = String::new();
    println!("Enter base2 of the trapezium:");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let c:i32 = input3.trim().parse().expect("Invalid input");

    let mut input4 = String::new();
    println!("Enter diagonal 1 of the rhombus:");
    io::stdin().read_line(&mut input4).expect("Failed to read input");
    let d:i32 = input4.trim().parse().expect("Invalid input");

    let mut input5 = String::new();
    println!("Enter diagonal 2 of rhombus:");
    io::stdin().read_line(&mut input5).expect("Failed to read input");
    let e:i32 = input5.trim().parse().expect("Invalid input");

    let mut input6 = String::new();
    println!("Enter base of parallelogram:");
    io::stdin().read_line(&mut input6).expect("Failed to read input");
    let f:i32 = input6.trim().parse().expect("Invalid input");

    let mut input7 = String::new();
    println!("Enter altitude of the parallelogram");
    io::stdin().read_line(&mut input7).expect("Failed to read input");
    let g:i32 = input7.trim().parse().expect("Invalid input");

    let mut input8 = String::new();
    println!("Enter length of the side of the cube:");
    io::stdin().read_line(&mut input8).expect("Failed to read input");
    let h:i32 = input8.trim().parse().expect("Invalid input");

    let mut input9 = String::new();
    println!("Enter radius of cylinder:");
    io::stdin().read_line(&mut input9).expect("Failed to read input");
    let i:i32 = input9.trim().parse().expect("Invalid input");

    let mut input10 = String::new();
    println!("Enter height of cylinder:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let j:i32 = input1.trim().parse().expect("Invalid input");

}





