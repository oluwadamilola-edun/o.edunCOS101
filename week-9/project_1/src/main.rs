use std::io::Write;

fn main() {
    let lager: Vec<&str> = vec!["33 Export", "Desperados", "Goldberg", "Guilder", "Heineken", "Star"];
    //let LAGER = "Lager,stout,Non-Alcholic";

    let stout: Vec<&str> = vec!["Legend", "Turbo King", "Williams", "", ""];
    let non_alcholic: Vec<&str> = vec!["Maltina", "Amstel Malta", "Malta Gold", "Fayrouz", ""];

    let mut file = std::fs::File::create("brewery_drinks.csv").expect("create failed");
    file.write_all("Categories: Lager, Stout, Non-Alcholic".as_bytes()).expect("Write failed");
    file.write_all(lager).expect("Write failed");
    file.write_all(stout).expect("write failed");
    file.write_all(non_alcholic).expect("write failed");
    println!("\nDrinks data written to file.");
}
