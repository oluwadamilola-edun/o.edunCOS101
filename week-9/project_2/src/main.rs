use std::fs::File;
use std::io::{self, Write};
use titlecase::titlecase;

// Define a struct to blueprint individual student records
#[derive(Debug)]
struct Student {
    name: String,
    matric_no: String,
    dept: String,
    level: u16,
}

fn smis() -> std::io::Result<()> {
    // Create a mutable vector to dynamically store multiple Student instances
    let mut students: Vec<Student> = Vec::new();

    println!("How many students are you inputting? (0 and above)");
    let mut student_count = String::new();
    io::stdin().read_line(&mut student_count).expect("Failed to read input for student count");
    
    let student_count: i64 = student_count.trim().parse().expect("Input is not a valid integer");

    for i in 1..=student_count {
        println!("\n--- Entering details for student {} ---", i);

        println!("What is the first name and last name of student {}?", i);
        let mut student_name = String::new();
        io::stdin().read_line(&mut student_name).expect("Failed to read input for student name");
        let student_name = titlecase(student_name.trim());

        println!("What is the matriculation number of student {}?", i);
        let mut student_matric = String::new();
        io::stdin().read_line(&mut student_matric).expect("Failed to read input for student matric number");
        // Keep matric numbers uppercase (e.g., "acc1021111" -> "ACC1021111")
        let student_matric = student_matric.trim().to_uppercase();

        println!("What is the department of student {}?", i);
        let mut student_dept = String::new();
        io::stdin().read_line(&mut student_dept).expect("Failed to read input for student department");
        let student_dept = titlecase(student_dept.trim());

        println!("What is the level of student {}?", i);
        let mut student_level = String::new();
        io::stdin().read_line(&mut student_level).expect("Failed to read input for student level");
        let student_level: u16 = student_level.trim().parse().expect("Input for student level is invalid");

        // Construct the Student struct instance
        let student = Student {
            name: student_name,
            matric_no: student_matric,
            dept: student_dept,
            level: student_level,
        };

        // Push the student record into the vector
        students.push(student);
    }

    // Display all recorded students neatly on the console
    println!("       RECORDED STUDENTS      ");
    for (i, stud) in students.iter().enumerate() {
        println!("\n--- Student {} ---", i + 1);
        println!("Name: {}", stud.name);
        println!("Matriculation number: {}", stud.matric_no);
        println!("Department: {}", stud.dept);
        println!("Level: {}", stud.level);
    }

    // Create and write student records into a CSV file
    let mut file = File::create("smis.csv")?;
    
    // Write the CSV column headers matching the project requirements
    writeln!(file, "Student Name,Matric. Number,Department,Level")?;

    // Write each student's data formatted with commas
    for student in &students {
        writeln!(
            file,
            "{},{},{},{}",
            student.name, student.matric_no, student.dept, student.level
        )?;
    }

    println!("\nSuccessfully saved student records to 'smis.csv'!");

    Ok(())
}

fn main() {
    println!("Hello User! This is Student Management Information System.");
    
    // Call smis() and handles any potential I/O errors
    if let Err(e) = smis() {
        eprintln!("An error occurred while running SMIS: {}", e);
    }
}