use std::io::{self, Write};

fn main() {
    println!("Exercise 1: Convert temperatures between Fahrenheit and Celsius");

    println!("Fahrenheit to Celsius:");
    print!("Enter a temperature in Fahrenheit: ");

    // the buffer does not auto-flush: you must manually call flush, 
    // otherwise the prompt will only appear after `read_line`.
    io::stdout().flush().expect("Failed to flush stdout");

    let mut fahrenheit_input = String::new();
    io::stdin()
        .read_line(&mut fahrenheit_input)
        .expect("Failed to read line");

    let fahrenheit: f64 = fahrenheit_input
        .trim()
        .parse()
        .expect("Please enter a valid number");

    let celsius = from_fahrenheit_to_celsius(fahrenheit);
    println!("{}°F is {}°C", fahrenheit, celsius);

    // --- SEPARATOR ---

    println!("Celsius to Fahrenheit:");
    print!("Enter a temperature in Celsius: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut celsius_input = String::new();
    io::stdin()
        .read_line(&mut celsius_input)
        .expect("Failed to read line");

    let celsius: f64 = celsius_input
        .trim()
        .parse()
        .expect("Please enter a valid number");

    let fahrenheit = from_celsius_to_fahrenheit(celsius);
    println!("{}°C is {}°F", celsius, fahrenheit);
}

fn from_fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn from_celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}
