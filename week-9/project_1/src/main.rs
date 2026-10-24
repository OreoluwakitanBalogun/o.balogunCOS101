
use std::fs::File;
use std::io::Write;

fn main() {
    let mut file = File::create("nigerian_breweries.txt")
        .expect("could not create file");

    file.write_all(b"Nigerian Breweries Plc - Drink Categories\n\n")
        .expect("write failed");

    file.write_all(b"Lager\n")
        .expect("write failed");
    file.write_all(b"33 Export\nDesperados\nGoldberg\nGulder\nHeineken\nStar\n\n")
        .expect("write failed");

    file.write_all(b"Stout\n")
        .expect("write failed");
    file.write_all(b"Legend\nTurbo King\nWilliams\n\n")
        .expect("write failed");

    file.write_all(b"Non-Alcoholic\n")
        .expect("write failed");
    file.write_all(b"Maltina\nAmstel Malta\nMalta Gold\nFayrouz\n")
        .expect("write failed");

    println!("File nigerian_breweries.txt created successfully.");
}