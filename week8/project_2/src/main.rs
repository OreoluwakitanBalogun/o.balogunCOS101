fn main() {
    
    let candidates = vec![
        ("Adebayo", 4),
        ("Chioma", 7),
        ("Ibrahim", 5),
        ("Funke", 9),
        ("Emeka", 6),
    ];

    
    let mut highest_name = candidates[0].0;
    let mut highest_years = candidates[0].1;

    
    for i in 1..candidates.len() {
        let (name, years) = candidates[i];   
        if years > highest_years {
            highest_years = years;
            highest_name = name;
        }
    }

    
    println!(
        "The developer with the highest experience is {} with {} years.",
        highest_name, highest_years
    );
}