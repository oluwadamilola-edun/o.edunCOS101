// WEEK 6 PRACTICE 1
fn main() {
    let name = "Aishat Lawal";
    let uni:&str = "Pan-Atlantic University";
    let addr:&str = "Km 52 Lekki-Epe Expressway, Ibeju-Lekki, Lagos";
    println!("Name: {name}");// you can put the variable name inside the {} insead of putting outside
    println!("University: {uni}, \nAddress: {addr}");

    let department:&'static str = "Computer Science";
    let school: &'static str = "School of Science and Technology";
    println!("Department: {}, \nSchool: {}", department, school);
}
