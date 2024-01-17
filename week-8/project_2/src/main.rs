use std::cmp::Ordering;

struct Candidate {
    name: String,
    years_of_experience: u32,
}

fn find_max_experience(candidates: &Vec<Candidate>) -> Option<&Candidate> {
    candidates.iter().max_by(|a, b| a.years_of_experience.cmp(&b.years_of_experience))
}

fn main() {
    // Create a vector of candidates
    let mut candidates = Vec::new();

    // Add sample candidates
    candidates.push(Candidate {
        name: String::from("John Doe"),
        years_of_experience: 5,
    });
    candidates.push(Candidate {
        name: String::from("Jane Doe"),
        years_of_experience: 8,
    });
    candidates.push(Candidate {
        name: String::from("Bob Smith"),
        years_of_experience: 10,
    });

    // Find the candidate with the highest years of experience
    if let Some(max_candidate) = find_max_experience(&candidates) {
        println!(
            "The candidate with the highest years of experience is {} with {} years.",
            max_candidate.name, max_candidate.years_of_experience
        );
    } else {
        println!("No candidates found.");
    }
}