/* use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let mut data = std::fs::File::create("brewery_data.csv").expect("create failed");
    let mut file = OpenOptions::new().append(true).open("brewery_data.csv").expect("cannot open file");
    file.write_all("\n'33 Export', 'Desperados', 'Goldberg', 'Gulder', 'Heineken', 'Star'".as_bytes()).expect("write failed");
    file.write_all("\n'Legend', 'Turbo KIng', 'Williams', '', '', ''".as_bytes()).expect("write failed");
    file.write_all("\n'Maltina', 'Amstel Malta', 'Malta Gold', 'Fayrouz', '', ''".as_bytes()).expect("write failed");

    println!("Brewery drinks append success");
}
*/

use std::io::Write;

fn main() {
    let header = "Lager, Stout, Non-Alcoholic\n";
    let row1 = "33 Export, Legend, Maltina\n";
    let row2 = "Desperados, Turbo King, Amstel Malta\n";
    let row3 = "Goldberg, Williams, Maltina\n";
    let row4 = "Gulder, , Fayrouz\n";
    let row5 = "Heineken, , \n";
    let row6 = "Star, , ";


    let mut file = std::fs::File::create("brewery_drinks_data.csv").expect("create failed");
    file.write_all(header.as_bytes()).expect("write failed");
    file.write_all(row1.as_bytes()).expect("write failed");
    file.write_all(row2.as_bytes()).expect("write failed");
    file.write_all(row3.as_bytes()).expect("write failed");
    file.write_all(row4.as_bytes()).expect("write failed");
    file.write_all(row5.as_bytes()).expect("write failed");
    file.write_all(row6.as_bytes()).expect("write failed");
    println!("\nDrinks Data written to file.");
}
