use std::io;

fn main() {
    // Display menu
    println!("P - Poundo Yam / Edinkaiko Soup  ₦3200");
    println!("F - Fried Rice & Chicken         ₦3000");
    println!("A - Amala & Ewedu Soup           ₦2500");
    println!("E - Eba & Egusi Soup             ₦2000");
    println!("W - White Rice & Stew            ₦2500");

    // Read food type
    let mut food = String::new();
    println!("\nEnter food type (P/F/A/E/W):");
    io::stdin().read_line(&mut food).unwrap();
    let food = food.trim().to_uppercase();

    // Read quantity
    let mut qty = String::new();
    println!("Enter quantity:");
    io::stdin().read_line(&mut qty).unwrap();
    let qty: u32 = qty.trim().parse().unwrap();

    // Get price
    let price = match food.as_str() {
        "P" => 3200,
        "F" => 3000,
        "A" => 2500,
        "E" => 2000,
        "W" => 2500,
        _ => {
            println!("Invalid choice");
            return;
        }
    };

    // Calculate total
    let mut total = price * qty;

    // Apply 5% discount if total > 10000
    if total > 10000 {
        total = (total as f64 * 0.95) as u32;
        println!("5% discount applied");
    }

    println!("Total charge: ₦{}", total);
}