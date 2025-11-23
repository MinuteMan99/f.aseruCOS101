struct StaffLevelEntry {
    aps_level: &'static str,
    min_exp: u8,
    max_exp: u8,
    titles: Vec<&'static str>,
}

struct ValidationResult<'a> {
    level: &'a str,
    status: &'a str,
    message: String,
}

fn initialize_staff_data() -> Vec<StaffLevelEntry> {
    vec![
        StaffLevelEntry {
            aps_level: "APS 1-2",
            min_exp: 1,
            max_exp: 2,
            titles: vec!["Intern", "Paralegal", "Placement"],
        },
        StaffLevelEntry {
            aps_level: "APS 3-5",
            min_exp: 3,
            max_exp: 5,
            titles: vec!["Administrator", "Research Assistant", "Junior Associate", "Classroom Teacher"],
        },
        StaffLevelEntry {
            aps_level: "APS 5-8",
            min_exp: 5,
            max_exp: 8,
            titles: vec!["Senior Administrator", "PhD Candidate", "Associate", "Snr Teacher"],
        },
        StaffLevelEntry {
            aps_level: "EL1 8-10",
            min_exp: 8,
            max_exp: 10,
            titles: vec!["Office Manager", "Post-Doc Researcher", "Senior Associate 1-2", "Leading Teacher"],
        },
        StaffLevelEntry {
            aps_level: "EL2 10-13",
            min_exp: 10,
            max_exp: 13,
            titles: vec!["Director", "Senior Lecturer", "Senior Associate 3-4", "Deputy Principal"],
        },
        StaffLevelEntry {
            aps_level: "SES",
            min_exp: 13,
            max_exp: 99, 
            titles: vec!["CEO", "Dean", "Partner", "Principal"],
        },
    ]
}

fn validate_staff_level<'a>(job_title: &'a str, years_of_experience: u8, staff_data: &'a [StaffLevelEntry]) -> ValidationResult<'a> {
    let normalized_title = job_title.trim().to_lowercase();

    let matched_entry = staff_data.iter().find(|entry| {
        entry.titles.iter().any(|&title| {
            title.to_lowercase() == normalized_title
        })
    });

    match matched_entry {
        Some(entry) => {
            let is_valid_exp = years_of_experience >= entry.min_exp && years_of_experience <= entry.max_exp;

            if is_valid_exp {
                ValidationResult {
                    level: entry.aps_level,
                    status: "VALID",
                    message: format!(
                        "The staff member is correctly placed at {} based on their title and {years_of_experience} years of experience.", 
                        entry.aps_level
                    ),
                }
            } else {
                ValidationResult {
                    level: entry.aps_level,
                    status: "EXPERIENCE MISMATCH",
                    message: format!(
                        "The staff member's title is aligned with {}, which requires {}-{} years of experience, but they have {years_of_experience} years.", 
                        entry.aps_level, entry.min_exp, entry.max_exp
                    ),
                }
            }
        },
        None => {
            ValidationResult {
                level: "UNKNOWN",
                status: "TITLE NOT RECOGNIZED",
                message: format!("The job title '{job_title}' was not found in the Public Service register."),
            }
        }
    }
}

fn main() {
    println!("Nigerian Public Service APS Level Checker\n");

    let staff_data = initialize_staff_data();

    let title_1 = "Associate";
    let exp_1 = 6; 
    let result_1 = validate_staff_level(title_1, exp_1, &staff_data);
    
    println!("Validation for Title: '{}', Experience: {} years", title_1, exp_1);
    println!("   Level: {}", result_1.level);
    println!("   Status: {}", result_1.status);
    println!("   Details: {}\n", result_1.message);

    let title_2 = "Director";
    let exp_2 = 5;
    let result_2 = validate_staff_level(title_2, exp_2, &staff_data);
    
    println!("Validation for Title: '{}', Experience: {} years", title_2, exp_2);
    println!("   Level: {}", result_2.level);
    println!("   Status: {}", result_2.status);
    println!("   Details: {}\n", result_2.message);
    
    let title_3 = "Leading Teacher";
    let exp_3 = 8;
    let result_3 = validate_staff_level(title_3, exp_3, &staff_data);
    
    println!("Validation for Title: '{}', Experience: {} years", title_3, exp_3);
    println!("   Level: {}", result_3.level);
    println!("   Status: {}", result_3.status);
    println!("   Details: {}\n", result_3.message);

    let title_4 = "Head of Strategy";
    let exp_4 = 15;
    let result_4 = validate_staff_level(title_4, exp_4, &staff_data);
    
    println!("Validation for Title: '{}', Experience: {} years", title_4, exp_4);
    println!("   Level: {}", result_4.level);
    println!("   Status: {}", result_4.status);
    println!("   Details: {}\n", result_4.message);
}