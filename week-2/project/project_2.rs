/*
P.M. Okeke and Sons Ltd is downsizing and readjusting their product sales due to an ongoing recession. You have been
consulted to write a Rust program that calculates the sum and the average of the following sales record.
S/N Item    Qty Amount
1   Toshiba 2   450,000.00
2   Mac     1   1,500,000.00
3   HP      3   750,000.00
4   Dell    3   2,850,000.00
5   Acer    1   250,000.00
*/

    fn main() {
    let item1 = "Toshiba"; let qty1 = 2; let amount1:f64 = 450_000.00;
    let item2 = "Mac";     let qty2 = 1; let amount2:f64 = 1_500_000.00;
    let item3 = "HP";      let qty3 = 3; let amount3:f64 = 750_000.00;
    let item4 = "Dell";    let qty4 = 3; let amount4:f64 = 2_850_000.00;
    let item5 = "Acer";    let qty5 = 1; let amount5:f64 = 250_000.00;

    println!("{} {} {}", item1, qty1, amount1);
    println!("{} {} {}", item2, qty2, amount2);
    println!("{} {} {}", item3, qty3, amount3);
    println!("{} {} {}", item4, qty4, amount4);
    println!("{} {} {}", item5, qty5, amount5);

    // qty * amount = line total, for each row
    let total1 = qty1 as f64 * amount1;
    let total2 = qty2 as f64 * amount2;
    let total3 = qty3 as f64 * amount3;
    let total4 = qty4 as f64 * amount4;
    let total5 = qty5 as f64 * amount5;

    let sum = total1 + total2 + total3 + total4 + total5;
    let total_qty:f64 = qty1 as f64 + qty2 as f64 + qty3 as f64 + qty4 as f64 + qty5 as f64;
    let avg = sum / total_qty;

    println!("Sum of the sales record is {:.2} and the average is {:.2}", sum, avg);
}
