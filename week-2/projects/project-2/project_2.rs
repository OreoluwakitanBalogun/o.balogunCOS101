fn main() {
    // laptop qty
    let toshiba_qty = 2.0;
    
    let mac_qty = 1.0;
    
    let hp_qty = 3.0;
    
    let dell_qty = 3.0;
    
    let acer_qty = 1.0;

    // laptop price
    let toshiba_price:f64 = 450000.00;
    
    let mac_price:f64 = 1500000.00;
    
    let hp_price:f64 = 750000.00;
    
    let dell_price:f64 = 2850000.00;
    
    let acer_price:f64 = 250000.00;

    // total price per laptop brand
    let toshiba_total = toshiba_qty * toshiba_price;
    

    let mac_total = mac_qty * mac_price;
    
    let hp_total = hp_qty * hp_price;
    
    let dell_total = dell_qty * dell_price;
    
    let acer_total = acer_qty * acer_price;

    // total sales
    let total_sales = toshiba_total + mac_total + hp_total + dell_total + acer_total;

    // total qty of laptops
    let total_quantity = toshiba_qty + mac_qty + hp_qty + dell_qty + acer_qty;

    // avg sales
    let average_per_unit = total_sales / total_quantity;
    // print ans
    println!("Total Sales is ₦{}", total_sales);
    println!("Average per Unit is ₦{}", average_per_unit);
}