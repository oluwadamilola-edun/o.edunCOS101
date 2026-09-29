use std::io;

fn main() {
    let p = "Poundo Yam/Edinkaiko Soup(P)"; let p_price:u32 = 3200;
    let f = "Fried Rice & Chicken(F)"; let f_price:u32 = 3000;
    let a = "Amala & Ewedu Soup(A)"; let a_price:u32 = 2500;
    let e = "Eba & Egusi Soup(E)"; let e_price:u32 = 2000;
    let w = "White Rice & Stew(W)"; let w_price:u32 = 2500;

    println!("Menu: 
        \nFood (P): {p} Price(N): {p_price}
        \nFood (F): {f} Price(N): {f_price}
        \nFood (A): {a} Price(N): {a_price}
        \nFood (E): {e} Price(N): {e_price}
        \nFood (W): {w} Price(N): {w_price}");

        let total:u32;


        println!("Enter food code (P, F, A, E, W) to order from menu:");
        let mut order = String::new();
        io::stdin().read_line(&mut order).expect("Not a valid input");
        let order = order.trim().to_uppercase();

        println!("Enter the quantity(Input a digit): ");
        let mut quantity = String::new();
        io::stdin().read_line(&mut quantity).expect("Not a valid string input");
        let quantity:u32 = quantity.trim().parse().expect("Not a valid number input");

        let price: u32;

        if order == "P" {
            price = p_price;
        } else if order == "F" {
            price = f_price;
        } else if order == "A" {
            price = a_price;
        } else if order == "E" {
            price = e_price;
        } else if order == "W" {
            price = w_price;
        } else {
            println!("Invalid food selection!");
            return; // stops calculating a value here so if user enters a letter not part of the 5 it doesn't go on with code below.
        }

        total = price * quantity;

    if total > 10000 {
        let discount = total * 5/100;
        let final_total = total - discount;

        println!("Total: N{total}");
        println!("Discount: N{discount}");
        println!("Final Charge: N{final_total}");
    } else {
        println!("Final Charge: N{total}");
    }
}