use std::io;

fn celcius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

fn fahrenheit_to_celcius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn main() {
    println!("Smart Weather Temperature Converter");
    println!("Which conversion do you want to do:");
    println!("1. Celsius to Fahrenheit");
    println!("2. Fahrenheit to Celsius");

    
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");

    let choice = choice.trim();

    println!("Enter temperature value:");
    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read");

    let temp: f64 = temp.trim().parse().expect("Please enter a valid number");


    if choice == "1" {
        let result = celcius_to_fahrenheit(temp);
        println!("Temperature: {:.1}°C", temp);
        println!("Converted : {:.1}°F",result);
    } else if choice == "2" {
        let result = fahrenheit_to_celcius(temp);
        println!("Temperature: {:.1}°F", temp);
        println!("Converted : {:.1}°C",result);
    } else {
        println!("Invalid.");
    }
}
