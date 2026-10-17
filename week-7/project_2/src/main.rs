/* You have been consulted to design a flowchart and develop a Rust program that can ask the 
    i. number of siblings from a client, and then input the 
    ii. first name and age of each sibling. 
If the age of any sibling is greater than 18, ask if the sibling is married or single.
If single then ask if a student or a worker. 
If a student, ask university and course of study.
If the sibling is married, find out if any offspring and what city the family lives in. 
If the age of the sibling is less than 18, then find out WAEC status. 
If yes, then input secondary school attended, else input current class level.

Finally displays the data of all siblings.

Hints:
Use a For loop to iterate the number of siblings.
Use an vector to store details.

Name: Jon
Age: 15
Marital Status: NIL
occupation: NIL
University: NIL
Course: NIL
Has children: NIL
Children: NIL
Family City: NIL
Secondary School: West High School
Class level: NIL
*/

use std::io;
use capitalize::Capitalize;
use titlecase::titlecase;


struct Sibling {
    name: String,
    age: u8,
    marital_status: String,
    life_status: String,
    occupation: String,
    university: String,
    course: String,
    has_children: String,
    children: u8,
    family_city: String,
    waec_status: String,
    secondary_school: String,
    class_level: String,
}


fn sib_info(name: &str) {
    // do you have siblings? how many?
    let mut sib_num = String::new();
    println!("Number of sibling(s) you have:");
    io::stdin().read_line(&mut sib_num).expect("Unable to read input of siblings number");
    let sib_num:u16 = sib_num.trim().parse().expect("Input for sibling number is unable to be read");
    println!("The number of siblings '{}' has is: {}", name, sib_num);

    //let mut siblings: Vec<(String, u16)> = Vec::new(); if i was only storing name and age here and without struct
    let mut siblings: Vec<Sibling> = Vec::new(); //using the struct now instead

//sibling name and age
    println!("Please input the first name and age of each sibling!");
    for i in 1..=sib_num {
        println!("First Name of Sibling {}", i);
        let mut sib_name = String::new();
        io::stdin().read_line(&mut sib_name).expect("Failed to read sibling name input");
        //let sib_name = sib_name.trim().to_string().capitalize();
        let sib_name = sib_name.trim().split_whitespace().next().unwrap().to_string().capitalize(); // Capitalizes first letter of the word and everything else lowercase

        println!("Age of Sibling {}", i);
        let mut sib_age = String::new();
        io::stdin().read_line(&mut sib_age).expect("Failed to read sibling age input");
        let sib_age: u8 = sib_age.trim().parse().expect("Sibling age input is invalid");

        //

        let mut sibling = Sibling {
            name: sib_name,
            age: sib_age,
            marital_status: String::new(),
            life_status: String::new(),
            occupation: String::new(),
            university: String::new(),
            course: String::new(),
            has_children: String::new(),
            children: 0,
            family_city: String::new(),
            waec_status: String::new(),
            secondary_school: String::new(),
            class_level: String::new(),
        };    

   /* println!("The first names and ages of {}'s siblings are:", name);
    for (sib_name, sib_age) in &siblings {
        println!("{}, {}", sib_name, sib_age);
    }
*/
// above 18: marital status yes:no of children and city address; 
    if sib_age >= 18 {
        let mut status = String::new();
        println!("Is sibling married or single?");
        io::stdin().read_line(&mut status).expect("Failed to read status input");
        sibling.marital_status = status.trim().to_string().to_lowercase();

        if sibling.marital_status == "married" {
            let mut children = String::new();
            println!("Does sibling have children [true/false]?");
            io::stdin().read_line(&mut children).expect("Failed to read sibling children status");
            sibling.has_children = children.trim().to_string().to_lowercase();

            if sibling.has_children == "true" {
                let mut num_children = String::new();
                println!("How many children does the sibling have? [1 and above valid numbers]");
                io::stdin().read_line(&mut num_children).expect("Failed to read input for number of sibling children");
                sibling.children = num_children.trim().parse().expect("Invalid number inputed");
            }

            let mut city = String::new();
            println!("What is the name of the city the family reside?");
            io::stdin().read_line(&mut city).expect("Failed to read family city input");
            sibling.family_city = city.trim().to_string().capitalize();
        } 
//single: worker or student? if student: uni and course; worker: profession.
        else if sibling.marital_status == "single" {
            let mut life_status = String::new();
            println!("Is sibling a student or a worker [student/worker]?");
            io::stdin().read_line(&mut life_status).expect("Failed to read sibling life status");
            sibling.life_status = life_status.trim().to_string().to_lowercase();

            if sibling.life_status == "student" {
                let mut uni = String::new();
                println!("Which university does sibling study at?");
                io::stdin().read_line(&mut uni).expect("Failed to read sibling university");
                sibling.university = titlecase(uni.trim().to_string().capitalize());

                let mut course = String::new();
                println!("What course does sibling study at '{}'", uni);
                io::stdin().read_line(&mut course).expect("Failed to read sibling course");
                sibling.course = course.trim().to_string().capitalize();

            } else if sibling.life_status == "worker" {
                let mut occupation = String::new();
                println!("What is the sibling's occupation?");
                io::stdin().read_line(&mut occupation).expect("Failed to read sibling occupation");
                sibling.occupation = occupation.trim().to_string().capitalize();
            } else {
                println!("Please input either worker/student for sibling life status");
            }

        } else {
            println!("PLease input either 'married' or 'single' for sibling marital status");
        }
    } 
//sibling age decision below 18 WAEC status, yes:sec school, no:current class;
    else if sib_age < 18 {
        let mut waec = String::new();
        println!("Has sibling written thier waec? True/False");
        io::stdin().read_line(&mut waec).expect("Failed to read sibling waec status");
        sibling.waec_status = waec.trim().to_string().to_lowercase();

        if sibling.waec_status == "true" {
            let mut sec_school = String::new();
            println!("Since the waec status is true, Input the name of the secondary school sibling attended.");
            io::stdin().read_line(&mut sec_school).expect("Failed to read secondary school input");
            sibling.secondary_school = sec_school.trim().to_string().capitalize();
        } else if sibling.waec_status == "false" {
            let mut class = String::new();
            println!("Since the waec status is false, Input the class level sibling is currently in.");
            io::stdin().read_line(&mut class).expect("Failed to read input for class_level");
            sibling.class_level = class.trim().to_string();
        } else {
            println!("Please input either 'true' or 'false' on your waec status");
        }
    }

    siblings.push(sibling);
 }
 println!("\n\n=== All Siblings' Data ===");
    for (i, sib) in siblings.iter().enumerate() {
        println!("\n--- Sibling {} ---", i + 1);
        println!("Name: {}", sib.name);
        println!("Age: {}", sib.age);
        if !sib.marital_status.is_empty() { println!("Marital Status: {}", sib.marital_status); }
        if !sib.life_status.is_empty() { println!("Life Status: {}", sib.life_status); }
        if !sib.occupation.is_empty() { println!("Occupation: {}", sib.occupation); }
        if !sib.university.is_empty() { println!("University: {}", sib.university); }
        if !sib.course.is_empty() { println!("Course: {}", sib.course); }
        if !sib.has_children.is_empty() { println!("Has children: {}", sib.has_children); }
        if sib.children > 0 { println!("Number of children: {}", sib.children); }
        if !sib.family_city.is_empty() { println!("City: {}", sib.family_city); }
        if !sib.waec_status.is_empty() { println!("WAEC Status: {}", sib.waec_status); }
        if !sib.secondary_school.is_empty() { println!("Secondary School: {}", sib.secondary_school); }
        if !sib.class_level.is_empty() { println!("Class Level: {}", sib.class_level); }
    }
 }


fn main() {
    println!("Hello! What is your name?");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read name input");
    let name = name.trim().capitalize();  // Capitalizes first letter of the word and everything else lowercase
    println!("Siblings Information for {}", name);

    sib_info(&name);
}
