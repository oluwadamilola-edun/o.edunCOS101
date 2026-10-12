use std::io;

fn main() {
    let categories = vec!["Office Admin", "Academic", "Lawyer", "Teacher"];
    let positions = vec!["APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];

    let office_admin = vec!["Intern", "Administrator", "Senior Administrator", "Office Manager", "Director", "CEO"];
    let academic = vec!["NIL", "Research Assistant", "PhD Candidate", "Post-Doc Researcher", "Senior Lecturer", "Dean"];
    let lawyer = vec!["Paralegal", "Junior Associate", "Associate", "Senior Associate 1-2", "Senior Associate 3-4", "Partner"];
    let teacher = vec!["Placement", "Classroom Teacher", "Senior Teacher", "Leading Teacher", "Deputy Principal", "Principal"];

    println!("Welcome to The APS Level Checker!");
    println!("Please select a professional category from the menu below:");

    for i in 0..categories.len() {
        println!("{}. {}", i + 1, categories[i]);
    }

    // Read user category choice
    let mut input1 = String::new();
    println!("\nEnter your choice (1-4):");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let choice: usize = input1.trim().parse().expect("Invalid input");

    // Select the appropriate vector based on input
    let selected_roles = match choice {
        1 => office_admin,
        2 => academic,
        3 => lawyer,
        4 => teacher,
        _ => {
            println!("Invalid choice, defaulting to Office Admin.");
            office_admin
        }
    };

    // Ask for years of experience
    let mut input_years = String::new();
    println!("\nEnter your years of experience (e.g., 5):");
    io::stdin().read_line(&mut input_years).expect("Failed to read input");
    let years: usize = input_years.trim().parse().expect("Invalid input");

    // Match years of experience to the corresponding vector index
    let index = match years {
        0..=2 => 0,  // APS 1-2
        3..=4 => 1,  // APS 3-5
        5..=8 => 2,  // APS 5-8 (e.g., Associate Lawyer with 5-8 years)
        9..=10 => 3, // EL1 8-10
        11..=13 => 4,// EL2 10-13
        _ => 5,      // SES (Executive / Top Tier)
    };

    // Retrieve and print the result using subscript access
    if index < positions.len() {
        let role = selected_roles[index];
        let level = positions[index];
        println!("Result Found!");
        println!("With {} years of experience, your matching role is '{}' and your position tier is **{}**", years, role, level);
    } else {
        println!("Error: Invalid experience range!");
    }
}