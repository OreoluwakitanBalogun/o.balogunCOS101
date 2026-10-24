use std::fs::File;
use std::io::Write;

fn main() {

    let students = vec![
        ("Oluchi Mordi", "ACC10211111", "Accounting", "300"),
        ("Adams Aliyu", "ECO10110101", "Economics", "100"),
        ("Shania Bolade", "CSC10328828", "Computer", "200"),
        ("Adekunle Gold", "EEE11020202", "Electrical", "200"),
        ("Blanca Edemoh", "MEE10202001", "Mechanical", "100"),
    ];

    
    println!("Student Name\t\tMatric. Number\t\tDepartment\tLevel");
    println!("---------------------------------------------------------------");
    for (name, matric, dept, level) in &students {
        println!("{:<20}\t{:<15}\t{:<12}\t{}", name, matric, dept, level);
    }

    let mut file = File::create("pau_smis.txt")
        .expect("could not create file");

    file.write_all(b"Student Name\t\tMatric. Number\t\tDepartment\tLevel\n")
        .expect("write failed");
    file.write_all(b"---------------------------------------------------------------\n")
        .expect("write failed");

    for (name, matric, dept, level) in &students {
        let line = format!("{:<20}\t{:<15}\t{:<12}\t{}\n", name, matric, dept, level);
        file.write_all(line.as_bytes())
            .expect("write failed");
    }

    println!("\nStudent records saved to pau_smis.txt");
}