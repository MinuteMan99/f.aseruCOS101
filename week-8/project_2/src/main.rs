struct MostExperiencedPerson {
    name: String,
    years_of_experience: u32,
}

fn find_most_experienced(candidates: &[(String, u32)]) -> Option<MostExperiencedPerson> {
    if candidates.is_empty() {
        return None;
    }

    let mut best_candidate = MostExperiencedPerson {
        name: candidates[0].0.clone(),
        years_of_experience: candidates[0].1,
    };

    for (name, years) in candidates.iter().skip(1) {
        if *years > best_candidate.years_of_experience {
            best_candidate.name = name.clone();
            best_candidate.years_of_experience = *years;
        }
    }

    Some(best_candidate)
}

fn main() {
    let candidates_data: Vec<(String, u32)> = vec![
        ("Adewale".to_string(), 8),
        ("Bolanle".to_string(), 15),
        ("Chinedu".to_string(), 4),
        ("Damilola".to_string(), 15),
        ("Emeka".to_string(), 12),
    ];

    println!("Nigeria Developer Candidates");
    for (name, years) in &candidates_data {
        println!("Name: {}, Experience: {} years", name, years);
    }


    match find_most_experienced(&candidates_data) {
        Some(person) => {
            println!("The person with the highest years of programming experience is:");
            println!("Name: {}", person.name);
            println!("Years of Experience: {}", person.years_of_experience);
        }
        None => {
            println!("No candidates found in the list.");
        }
    }

    let empty_candidates: Vec<(String, u32)> = vec![];
    match find_most_experienced(&empty_candidates) {
        Some(_) => {},
        None => {
            println!("\nChecked an empty list: No candidates found.");
        }
    }
}