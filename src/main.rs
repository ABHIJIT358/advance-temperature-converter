use std::io;
fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0/5.0) + 32.0
}

fn fahrenheit_to_celsius(f: f64) -> f64 {

    (f - 32.0) * 5.0 / 9.0

}
 fn celsius_to_kelvin(c: f64) -> f64 {
    c + 273.15
 }

 fn kelvin_to_cesius(k: f64) -> f64 {
             k - 273.15   
 }

 fn read_number() -> f64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input ).expect("Failed to read");
    input.trim().parse::<f64>().expect("Invalid number")
 }

 fn main() {
    loop {
        println!("\n Temperature Converter");
        println!("1. Celsius to Fahrenheit");
        println!("2. Fahrenheit to Celsius");
        println!("3.Celsius to Kelvin");
        println!("4. Kelvin to Celsius");
        println!("5.Exit");
        println!("Enter your choice");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: u32 = choice.trim().parse().unwrap_or(0);

        match choice {
            1 => {
                println!("Enter Celsius:");
                let c = read_number();
                println!("Result: {:2} °F", celsius_to_fahrenheit(c));

            }
               2 => {
                println!("Enter Fahrenheit:");
                let f = read_number();
                println!("Result: {:2} °C", fahrenheit_to_celsius(f));
                
            }
               3 => {
                println!("Enter Celsius:");
                let c = read_number();
                println!("Result: {:2} K", celsius_to_kelvin(c));
                
            }
               4 => {
                println!("Enter Kelvin:");
                let k = read_number();
                println!("Result: {:2} °C", kelvin_to_cesius(k));
                
            }
               5 => {
                println!("Exiting program:");
               break;
                
            }


            _=>println!("Invalid choice, try again"),
        }

    
       


    }
 }