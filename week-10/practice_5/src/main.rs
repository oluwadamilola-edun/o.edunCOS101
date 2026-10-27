fn main() {
    //a list of nos
    let x = vec![15, 25, 35, 45, 55];
    borrow_vector(&x);
    println!("Printing the alue from main() x[0]={}", x[0]);
}

fn borrow_vector(z:&Vec<i32>){
    println!("Inside borrow_vector function{:?} \n", z);
}