/*
Project: Alaba Electronics
Mr Ogbeifuna runs a series of electronics shops at Alaba International Market in Lagos. He has just received a consignment of 30 laptops. Using
your knowledge of Rust structs and methods, calculate the total cost if a customer purchases 3 laptops from each brand.
Brand   Quantity    Price each
HP       10         N650,000
IBM      6          N755,000
Toshiba  10         N550,000
Dell     4          N850,000
The shape: a Laptop struct with brand and price, a method that costs a purchase, and one total across the four brands.
*/
struct Laptop{
    brand:String,
    price:f64,
}

impl Laptop {
    fn cost(&self)->f64 {
        self.price * 3.0
    }
}

fn main() {
    let hp = Laptop {
        brand:String::from("HP"),
        price:650_000.00
    };
    let ibm = Laptop {
        brand:String::from("IBM"),
        price:755_000.00
    };
    let toshiba = Laptop {
        brand:String::from("Toshiba"),
        price:550_000.00
    };
    let dell = Laptop {
        brand:String::from("Dell"),
        price:850_000.00
    };

    println!("The brand is {} and the price for 3 is {}", hp.brand, hp.cost());
    println!("The brand is {} and the price for 3 is {}", ibm.brand, ibm.cost());
    println!("The brand is {} and the price for 3 is {}", toshiba.brand, toshiba.cost());
    println!("The brand is {} and the price for 3 is {}", dell.brand, dell.cost());

    let total_cost:f64 = hp.cost() + ibm.cost() + toshiba.cost() + dell.cost();
    println!("The total cost across all brands is {}", total_cost);
}
