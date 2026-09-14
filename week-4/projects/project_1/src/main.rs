/*
Given the values of a,b, and c, find the roots of a quadratic equation with a rust program. 
Read all 3 values from the keyboard.

Discriminant (d)= b*b - 4.0*a*c
d > 0: two distinct roots
d = 0: exactly one real root
d < 0: no real roots
*/

use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter first value for discriminant: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter second value for discriminant: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter third value for discriminant: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f32 = input3.trim().parse().expect("Not a valid number");

    let d:f32 = b*b - 4.0*a*c;
    println!("Discriminant of the quadratic equation with values a: {} b: {} and c: {} is: {}",input1, input2, input3, d);

    if d > 0.0 {
        println!("The quadratic equation has two distinct roots!");
    } else if d == 0.0 {
        println!("The quadratic equation has exactly one distinct root!");
    } else {
        println!("The quadratic equation has no real roots!");
    }
}