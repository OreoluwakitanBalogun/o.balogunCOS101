

fn main() {
   
    let levels = vec![
        "APS 1-2",
        "APS 3-5",
        "APS 5-8",
        "EL1 8-10",
        "EL2 10-13",
        "SES",
    ];

    
    let office_admin = vec![
        "Intern",
        "Administrator",
        "Senior Administrator",
        "Office Manager",
        "Director",
        "CEO",
    ];

    let academic = vec![
        "-",                       
        "Research Assistant",
        "PhD Candidate",
        "Post-Doc Researcher",
        "Senior Lecturer",
        "Dean",
    ];

    let lawyer = vec![
        "Paralegal",
        "Junior Associate",
        "Associate",               
        "Senior Associate 1-2",
        "Senior Associate 3-4",
        "Partner",
    ];

    let teacher = vec![
        "Placement",
        "Classroom Teacher",
        "Senior Teacher",
        "Leading Teacher",
        "Deputy Principal",
        "Principal",
    ];

    
    println!("Choose a profession:");
    println!("1. Office Admin");
    println!("2. Academic");
    println!("3. Lawyer");
    println!("4. Teacher");

    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).expect("Failed to read");
    let choice: usize = choice.trim().parse().expect("Please type a number");

    
    let titles = match choice {
        1 => &office_admin,
        2 => &academic,
        3 => &lawyer,
        4 => &teacher,
        _ => {
            println!("Invalid choice");
            return;
        }
    };

    
    println!("\nAPS levels for this profession:");
    for i in 0..titles.len() {
        
        if i < levels.len() {
            println!("{}  →  {}", titles[i], levels[i]);
        }
    }

    
    println!("\nType a job title to look up (or press Enter to finish):");
    let mut title = String::new();
    std::io::stdin().read_line(&mut title).expect("Failed to read");
    let title = title.trim();

    if !title.is_empty() {
        
        let mut found = false;
        for i in 0..titles.len() {
            if titles[i].eq_ignore_ascii_case(title) {
                
                if let Some(level) = levels.get(i) {
                    println!("\n{} holds position {}", title, level);
                    found = true;
                    break;
                }
            }
        }
        if !found {
            println!("Title not found in this profession.");
        }
    }
}