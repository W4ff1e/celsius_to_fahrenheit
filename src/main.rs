use std::io;

const C_CONVERSION: f64 = 1.8;
const F_CONVERSION: f64 = 0.55555555555;
fn main() {
    loop {
        println!("Please enter the temperature: ");

        let mut input: String = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");

        let input: f64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Please enter your unit (C or F): ");
        let mut unit: String = String::new();

        io::stdin()
            .read_line(&mut unit)
            .expect("Failed to read line!");

        let unit: String = unit.trim().to_lowercase();

        if unit == "c" {
            let result: f64 = input * C_CONVERSION;

            let unit: String = unit.trim().to_uppercase();

            println!("You converted {input}°{unit} to {}°F", result + 32.00);
        } else if unit == "f" {
            let result: f64 = input - 32.00;

            let unit: String = unit.trim().to_uppercase();

            println!(
                "You converted {input}°{unit} to {}°C",
                result * F_CONVERSION
            );
        } else {
            println!("Invalid unit!");
            continue;
        }
    }
}
