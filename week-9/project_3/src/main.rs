use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    // 1. Define the datasets as vectors of strings
    let commissioners = vec![
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieye",
    ];

    let ministries = vec![
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    let zones = vec![
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    // 2. Create and open a single output file called "efcc_merged.csv"
    let mut file = File::create("efcc_merged.csv")?;

    // Write the CSV header row
    writeln!(file, "Commissioner,Ministry,Zone")?;

    // 3. Loop through the datasets using index to merge them together line by line
    for i in 0..commissioners.len() {
        writeln!(
            file,
            "{},{},{}",
            commissioners[i], ministries[i], zones[i]
        )?;
    }

    println!("Merge complete! Data successfully saved to 'efcc_merged.csv'.");

    Ok(())
}