//3rd way, the itereation method:  a 'for loop', exactly like arrays
/*
The primary difference between print! and println! 
in Rust is that println! automatically appends a newline character (\n) 
to the end of the output, while print! does not. 
*/

fn main() {

    //Name vector
    let name = vec!["Mary","Sam","Sally","Greg","Ade","Mark","June","Ife"];

    let age = vec![16,17,19,22,20,21,18,23];

    println!("\nAge Allocation: \n");

    //loop to iterate elements in vector
    for i in 0..age.len(){
        //iterating through i on the vector
        print!("{} is {} years old\n", name[i],age[i]);
    }
}
