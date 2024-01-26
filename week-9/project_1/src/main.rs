use std::io::Write;

fn main() {
    
    let category = vec!["Lager:","\n\nStout:","\n\nNon-Alcoholic:"];
    let row1 = vec!["\n33 Export","\nLegend","\nMaltina"];
    let row2 = vec!["\nDesperados","\nTurbo King","\nAmstel Malta"];
    let row3 = vec!["\nGoldberg","\nWilliams","\nMalta Gold"];
    let row4 = vec!["\nGulder","","\nFayrouz"];
    let row5 = vec!["\nHeienken","",""];
    let row6 = vec!["\nStar","",""];

    let mut file = std::fs::File::create("brew.txt").expect("create failed");
    for i in 0..category.len() {
file.write_all(category[i].as_bytes()).expect("write failed");
file.write_all(row1[i].as_bytes()).expect("write failed");
file.write_all(row2[i].as_bytes()).expect("write failed");
file.write_all(row3[i].as_bytes()).expect("write failed");
file.write_all(row4[i].as_bytes()).expect("write failed");
file.write_all(row5[i].as_bytes()).expect("write failed");
file.write_all(row6[i].as_bytes()).expect("write failed");
    }
println!("\nData written to file." );

}