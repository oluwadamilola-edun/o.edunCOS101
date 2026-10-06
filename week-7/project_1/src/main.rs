/* The shape calculator
Areas and a volume, chosen by a user, with one function per shape.
Your MTH 101 lecturer needs a Rust program that calculates the area or volume of different shapes, depending on the user's choice. It prompts for
a shape, reads the inputs, and performs the right calculation. Use functions: one per shape.

Trapezium,     area     height / 2 * (base1 + base2)
Rhombus,       area     1/2 * diagonal1 * diagonal2
Parallelogram, area     base * altitude
Cube, surface  area     6 * side * side
Cylinder,      volume   pi * radius * radius * height

The skeleton: a menu, one read for the choice, then a function per shape that reads its own inputs and returns the answer.

*/

use std::io;

fn main() {
    println!("Hello, world!");
}
