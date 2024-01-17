use std::io::{self, Write};

struct Company {
    name: String,
    date_founded: i32,
    company_shares: i32,
    company_liability: i32,
    leverage: f64,
}

fn main() {
    let company_names = vec![
        "Cadbury Nigeria Plc", "Champion Breweries Plc", "Dangote Sugar Refinery Plc", "Flour Mills Nigeria Plc", "Nestle Nigeria Plc", "Unilever Nigeria Plc", "Honeywell Nigeria Plc", "Nigeria Breweries Plc",
    ];
    let dates_founded = vec![1965, 1974, 1970, 1960, 1961, 1923, 1906, 1946];
    let company_shares = vec![15000000, 25000000, 18000000, 32000000, 8000000, 37000000, 34000000, 30000000];
    let company_liability = vec![5500000, 8000000, 10000000, 4000000, 1500000, 11000000, 9000000, 12000000];

    // Calculate Company Leverage
    let mut company_leverage: Vec<f64> = Vec::new();
    for i in 0..company_names.len() {
        let diff: f64 = (company_shares[i] - company_liability[i]) as f64 / company_shares[i] as f64;
        company_leverage.push(diff);
    }

    let mut companies: Vec<Company> = Vec::new();
    for i in 0..company_names.len() {
        let company = Company {
            name: company_names[i].to_string(),
            date_founded: dates_founded[i],
            company_shares: company_shares[i],
            company_liability: company_liability[i],
            leverage: company_leverage[i],
        };
        companies.push(company);
    }

    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("bonus_data.txt")
        .expect("Failed to open file for append");

    for company in &companies {
        if company.company_shares > 20_000_000 {
            let bonus_value = company.leverage * 2.0; // Example calculation, you can modify this as needed
            file.write_all(
                format!("{}\t{:.2}\n", company.name, bonus_value).as_bytes(),
            )
            .expect("Failed to write bonus data");
        }
    }

    for company in &companies {
        if company.company_liability < 10_000 {
            let bonus_value = 0.05 * company.leverage;
            file.write_all(
                format!("{}\t{:.2}\n", company.name, bonus_value).as_bytes(),
            )
            .expect("Failed to write bonus data");
        }
    }

    let mut input_username = String::new();
    loop {
        print!("Enter your username:");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input_username).expect("ERROR");

        input_username = input_username.trim().to_string();

        if input_username.len() == 4
            && company_names
                .iter()
                .any(|&name| name.to_lowercase().starts_with(&input_username.to_lowercase()))
        {
            break;
        } else {
            println!("Invalid username. Please enter a valid username.");
            input_username.clear();
        }
    }

    let mut input_password = String::new();
    loop {
        print!("Enter your password:");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input_password).expect("Input error");

        input_password = input_password.trim().to_string();

        if input_password.len() >= 3
            && input_password.len() <= 8
            && !input_password.chars().any(|c| c.is_ascii_uppercase())
            && !input_password.contains(&['@', '#', '$'].iter().cloned().collect::<String>())
        {
            break;
        } else {
            println!("INVALID PASSWORD. Please follow the specified criteria.");
            input_password.clear();
        }
    }

    let mut file = std::fs::File::create("company_data.txt").expect("ERROR");
    file.write_all("COMPANY NAME\tDATE FOUNDED\tSHARES\tLIABILITY\tLEVERAGE\n".as_bytes())
        .expect("ERROR");

    for company in &companies {
        file.write_all(
            format!(
                "{}\t{}\t{}\t{}\t{}\n",
                company.name, company.date_founded, company.company_shares, company.company_liability, company.leverage
            )
            .as_bytes(),
        )
        .expect("ERROR");
    }
}
