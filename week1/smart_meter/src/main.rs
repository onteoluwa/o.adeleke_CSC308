use std::io;

fn main() {
    println!("Welcome to Smart Meter");

    
    println!("Enter your electricity usage (in kWh):");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read :()");

    
    let units: f64 = input.trim().parse().expect("Please enter a valid number");


    let rate = if units > 200.0 {
        30.0
    } else if units > 100.0 {
        25.0
    } else {
        20.0
    };


    let total_bill = units * rate;


    println!("-------------------------------------");
    println!("Electricity Usage: {:.1} kWh", units);
    println!("Rate per Unit: ₦{:.1}", rate);
    println!("Total Bill: ₦{:.1}", total_bill);

}
