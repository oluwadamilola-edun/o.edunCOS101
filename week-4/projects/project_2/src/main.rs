/*
Take an employee's experience and age as input, and determine the annual incentive from these criteria:
Expecienced && age >=40  : N1_560_000
Expecienced && age 30=<x<40  : N1_480_000
Expecienced && age <30  : N1_300_000
Not Expecienced : N100_000
*/

use std::io;

fn main() {
    println!("Annual Incentive based on Age and experience!");

    println!("Are you an experienced person (true or false): ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Not a valid String");
    let experience:bool = input1.trim().parse().expect("Failed to read input");

    println!("Enter Age: ");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Not a valid String");
    let age:u8 = input2.trim().parse().expect("Not a valid number");

    if experience == true && age >= 40 {
        println!("Your annual incentive is 1,560,000 Naira!");
    } else if experience == true && age < 40 && age >= 30 {
        println!("Your annual incentive is 1,480,000 Naira!");
    }  else if experience == true && age < 30 {
        println!("Your annual incentive is 1,300,000 Naira!");
    } else {
        println!("Your annual incentive is 100,000 Naira!");
    }
}