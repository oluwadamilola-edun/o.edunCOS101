/*
Project II: The EY Interview
Ernst & Young (EY) Global Limited is a multinational professional services network headquartered in London, with branch offices all around the
world. EY Nigeria recently launched a project and is scouting for developers with the highest years of experience. The CEO of EY Nigeria has
contacted you: develop a Rust program that finds the person with the highest years of programming experience during the job interview, using
any Rust compound data type.
Hints
A vector of tuples fits this well: (name, years). Walk it with a loop, keep the highest seen so far.
Compound data type means vector, tuple or slice. Your pick, your defence
*/

use std::io;
//use capitalize::Capitalize;
use titlecase::titlecase;

fn candidate(){
    println!("All candidates will be inputed and only selected candidates will be considered!");
    let mut candidates: Vec<(String, u16)> = Vec::new(); //name, years of experience

    let mut c_amount = String::new();
    println!("How many candidates are you inputing?");
    io::stdin().read_line(&mut c_amount).expect("Failed to read candidate amount input");
    let c_amount: u16 = c_amount.trim().parse().expect("Number inputed for candidate amount is invalid");

    for i in 1..=c_amount {
        println!("First and Last Name of candidate {}", i);
        let mut c_name = String::new();
        io::stdin().read_line(&mut c_name).expect("Failed to read candidate name input");
        //let c_name = c_name.trim().capitalize();// only capitalizes the first letter of the first word
        let c_name = titlecase(c_name.trim()); // titlecase capitalizes first letter of each word and everything else lowercase

        println!("Amount of experience years candidate {} is:", i);
        let mut c_experience_years = String::new();
        io::stdin().read_line(&mut c_experience_years).expect("Failed to read candidate age input");
        let c_experience_years: u16 = c_experience_years.trim().parse().expect("candidate age input is invalid");

        candidates.push((c_name,c_experience_years));
    }
    //finds the person with the highest years of programming experience during the job interview
    /* prints all candidates
    let mut count = 1;
    for i in candidates {
        println!("\n--- Candidate {}: {:?} ---",count, i);   
        count+=1;
    }
    */

    // Find the highest years of experience

    if candidates.len() > 0 {
        let mut highest = candidates[0].1;
        let mut i = 0;

        while i < candidates.len() {
            if candidates[i].1 > highest {
                highest = candidates[i].1;
            }

            i += 1;
        }

        // Display all candidates with the highest experience
        println!("\nCandidates with the highest experience:");

        let mut i = 0;

        while i < candidates.len() {
            if candidates[i].1 == highest {
                println!("Name: {}", candidates[i].0);
                println!("Years of experience: {}", candidates[i].1);
            }

            i += 1;
        }
    } else {
        println!("No candidates were entered.");
    }
}

fn main() {
    println!("Hello, This is the EY intervew candidate selection");
    candidate()
}

