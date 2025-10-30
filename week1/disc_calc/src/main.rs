use std::io;

fn main() {
    println!("Discount Calculator");

    println!("Enter your total bill amount (₦):");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read");
    let bill: f64 = input.trim().parse().expect("Please enter a valid number");

    
    let discount_rate = if bill > 10000.0 {
        0.15
    } else if bill > 5000.0{
        0.10
    } else {
        0.0
    };

    
    let discount_amount = bill * discount_rate;
    let final_amount = bill - discount_amount;

    
    println!("-----------------------------------");
    println!("Original Bill: ₦{:.0}", bill);
    println!("Discount Applied: {:.0}%", discount_rate * 100.0);
    println!("Final Bill: ₦{:.0}", final_amount);
}

