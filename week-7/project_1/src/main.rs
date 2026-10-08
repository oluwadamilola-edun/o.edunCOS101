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

fn trapezium ()->f64 {
  println!("Area of trapezium. Please input the values for height, base 1, and base 2");

  println!("Height in cm:");
  let mut t_height = String::new();
  io::stdin().read_line(&mut t_height).expect("Failed to read trapezium height input");
  let t_height:f64 = t_height.trim().parse().expect("Height Input is not a proper number");

  println!("Base1 in cm:");
  let mut base1 = String::new();
  io::stdin().read_line(&mut base1).expect("Failed to read trapezium base1 input");
  let base1:f64 = base1.trim().parse().expect("Base1 Input is not a proper number");

  println!("Base2 in cm:");
  let mut base2 = String::new();
  io::stdin().read_line(&mut base2).expect("Failed to read trapezium base2 input");
  let base2:f64 = base2.trim().parse().expect("Base2 input is not a proper number");

  let t_area:f64 = t_height / 2.0 * (base1 + base2);
  println!("Area of trapezium with height {}, base1 {}, and base2 {} is: {}", t_height,base1,base2,t_area);

  return t_area;
}

fn rhombus ()->f64 {
//Rhombus,       area     1/2 * diagonal1 * diagonal2
  println!("Area of a Rhombus. Please input the values for diagonal 1 and diagonal 2.");

  println!("Diagonal1 in cm:");
  let mut d1 = String::new();
  io::stdin().read_line(&mut d1).expect("Failed to read rhombus Diagonal1 input");
  let d1:f64 = d1.trim().parse().expect("Diagonal1 input is invalid");

  println!("Diagonal2 in cm:");
  let mut d2 = String::new();
  io::stdin().read_line(&mut d2).expect("Failed to read rhombus Diagonal2 input");
  let d2:f64 = d2.trim().parse().expect("Diagonal2 input is invalid");

  let r_area:f64 = 0.5 * (d1 * d2);
  println!("Area of rhombus with diagonal 1 {} and Diagonal2 {} is: {}", d1,d2,r_area);
  return r_area;
}

fn parallelogram ()->f64 {
//Parallelogram, area     base * altitude
  println!("Area of a Parallelogram. Please input the values for base and altitude.");

  println!("Base in cm:");
  let mut p_base = String::new();
  io::stdin().read_line(&mut p_base).expect("Failed to read parallelogram Base input");
  let p_base:f64 = p_base.trim().parse().expect("Base input is invalid");

  println!("Altitude in cm:");
  let mut altitude = String::new();
  io::stdin().read_line(&mut altitude).expect("Failed to read parallelogram Altitude input");
  let altitude:f64 = altitude.trim().parse().expect("Altitude input is invalid");

  let p_area:f64 = p_base * altitude;
  println!("Area of a Parallelogram with base {} and altitude {} is: {}",p_base,altitude,p_area);
  return p_area;
}

fn cube ()->f64 {
//Cube, surface  area     6 * side * side
  println!("Surface Area of a cube. Please input the values for the side.");

  println!("Side in cm:");
  let mut side = String::new();
  io::stdin().read_line(&mut side).expect("Failed to read cube Side input");
  let side:f64 = side.trim().parse().expect("Cube Side input is invalid");

  let c_sarea:f64 = 6.0 * side * side;
  println!("Surface area of a cube with side {} is: {}",side,c_sarea);
  return c_sarea;
}

fn cylinder ()->f64 {
//Cylinder,      volume   pi * radius * radius * height
  println!("Volume of a cylider. Please input the values for the radius and height.");

  let pi:f64 = 3.14;

  println!("Radius in cm:");
  let mut cy_radius = String::new();
  io::stdin().read_line(&mut cy_radius).expect("Failed to read cube Radius input");
  let cy_radius:f64 = cy_radius.trim().parse().expect("Cylinder Radius input is invalid");

  println!("Height in cm:");
  let mut cy_height = String::new();
  io::stdin().read_line(&mut cy_height).expect("Failed to read cube Height input");
  let cy_height:f64 = cy_height.trim().parse().expect("Cylinder Height input is invalid");

  let cy_volume:f64 = pi * cy_radius * cy_radius * cy_height;
  println!("Volume of a cylinder with raduis {} and height {} is: {}",cy_radius, cy_height,cy_volume);
  return cy_volume;

}

fn main() {
    let mut decision = 'y';

    while decision != 'n' {
        println!("Hello! Choose a shape from menu below and input values to work on the formula.!");
        println!("Trapezium: Area\n Rhombus: Area\n Parallelogram: Area\n Cube: Surface Area\n Cylinder: Volume\n");

        let mut shape = String::new();
        io::stdin().read_line(&mut shape).expect("Failed to read input for shape!");
        let shape = shape.trim().to_lowercase(); //.expect("Failed to get shape inputed");

        if shape == "trapezium" {
            trapezium();
        } else if shape == "rhombus" {
            rhombus();
        } else if shape == "parallelogram" {
            parallelogram();
        } else if shape == "cube" {
            cube();
        } else if shape == "cylinder" {
            cylinder();
        } else {
            println!("Shape inputed is not a part of the shapes in the shape calculator, Please input on of these: Trapezium, Rhombus, Parallelogram, Cube, Cylinder");
        }

        println!("Shape calculator is done. Would you like to go again? Y/N");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        decision = input.trim().to_lowercase().chars().next().unwrap();
    }
    println!("Thank you for using the shape calculator.");
}
