use std::io::Write;

fn main() {

    let name_of_comm = vec!["Aigbogun Alamba Daudu", "Murtala Afeez Bendu", "Okorocha Calistus Ogbona", "Adewale Jimoh Akanbi", "Osazuwa Faith Etieye"];
    let ministry = vec!["Internal Affairs", "Justice", "Defense", "Power & Steel", "Petroleum"];
    let geo_zone = vec!["South West", "North East", "South South", "North Central", "South East"];
    
    let mut file = std::fs::File::create("ministers.txt").expect("Create failed");

    file.write_all(
        format!("{:^19} {:^19} {:^19} {:20}\n", "S/N", "NAME OF COMMISSIONER", "MINISTRY", "GEOPOLITICAL ZONE")
            .as_bytes(),
    )
    .expect("Write failed");

    for i in 0..name_of_comm.len() {
        let first_item = name_of_comm[i];
        let second_item = ministry[i];
        let third_item = geo_zone[i];

        file.write_all(format!("{:^20}",i+1).as_bytes()).expect("Write failed");
        file.write_all(format!("{:^20}",first_item.trim()).as_bytes()).expect("Write failed");
        file.write_all(format!("{:^20}",second_item.trim()).as_bytes()).expect("Write failed");
        file.write_all(format!("{:^20}",third_item.trim()).as_bytes()).expect("Write failed");
        file.write_all("\n".as_bytes()).expect("Write failed");
    }
}