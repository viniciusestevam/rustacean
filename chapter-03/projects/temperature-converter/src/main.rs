use std::io;
use std::io::prelude::*;
use std::num::ParseFloatError;

fn main() {
    println!("=== Temperature Converter ===");
    loop {
        println!("\nOptions:");
        println!("1 -> Celsius to Fahrenheit");
        println!("2 -> Fahrenheit to Celsius\n\n");
        println!("Select one option:");

        let option: u32 = match read_input() {
            Ok(num) => num.round() as u32,
            Err(_) => {
                println!("Invalid option!\n");
                continue;
            }
        };

        let valid_options = [1, 2];
        if !valid_options.contains(&option) {
            println!("Invalid option!\n");
            continue;
        };

        println!("Please input the value to be converted:");
        let value = match read_input() {
            Ok(num) => num,
            Err(_) => {
                println!("Please provide a valid number!\n");
                continue;
            }
        };

        if option == 1 {
            let converted = celsius_to_fahrenheit(value);
            println!(
                "\n--> {:.2}°C converted to Fahrenheit is {:.2}°F\n\n\n",
                value, converted
            );
        }

        if option == 2 {
            let converted = fahrenheit_to_celsius(value);
            println!(
                "\n--> {:.2}°F converted to Celsius is {:.2}°C",
                value, converted
            );
        }

        pause();
    }
}

fn celsius_to_fahrenheit(temperature: f64) -> f64 {
    (temperature * 9.0 / 5.0) + 32.0
}

fn fahrenheit_to_celsius(temperature: f64) -> f64 {
    (temperature - 32.0) * 5.0 / 9.0
}

fn read_input() -> Result<f64, ParseFloatError> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let parsed = match input.trim().parse::<f64>() {
        Ok(num) => num,
        Err(e) => return Err(e),
    };

    Ok(parsed)
}

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    let _ = stdin.read(&mut [0u8]).unwrap();
}
