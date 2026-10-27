//declare a struct
struct Employee {
    ceo:String,
    company:String,
    age:u32
}

fn main() {
    //initialize a struct
    let emp1 = Employee {
        company:String::from("Microsoft Corporation"),
        ceo:String::from("Satya Nadella"),
        age:56
    };
    let emp2 = Employee {
        company:String::from("Google Inc."),
        ceo:String::from("Sundai Pichai"),
        age:51
    };
//pass em1 and 2 to display()
display(emp1);
display(emp2);
}

fn display(emp:Employee){
    println!("Name is: {} company is {} age is {}", emp.ceo, emp.company, emp.age);
}