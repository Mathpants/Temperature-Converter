use std::io;

fn main() {
    loop {
        let mut input: String = String::new();

        println!("Hello!");
        println!("Please Choose between Celsius and Fahrenheit");
        println!("1: Celsius -> Fahrenheit");
        println!("2: Fahrenheit -> Celsius");
        println!("3: Exit");

        match io::stdin().read_line(&mut input) {
            Ok(num) => num,
            Err(_) => continue,
        };

        let input_choice: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if input_choice == 1 {
            println!("Celsius -> Fahrenheit!")
        } else if input_choice == 2 {
            println!("Fahrenheit -> Celsius!")
        } else if input_choice == 3 {
            break;
        } else {
            continue;
        }

        let mut input: String = String::new();

        println!("Please enter temperature to convert:");

        match io::stdin().read_line(&mut input) {
            Ok(num) => num,
            Err(_) => continue,
        };

        let input_temperature: f64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match input_choice {
            1 => celsius_to_fahrenheit(input_temperature),
            2 => fahrenheit_to_celsius(input_temperature),
            _ => continue,
        };
    }
}

fn celsius_to_fahrenheit(celsius: f64) {
    let fahrenheit: f64 = celsius * 1.8 + 32.0;
    println!("{celsius} Celsius is {fahrenheit} Fahrenheit!")
}

fn fahrenheit_to_celsius(fahrenheit: f64) {
    let celsius: f64 = (fahrenheit - 32.0) / 1.8;
    println!("{fahrenheit} Fahrenheit is {celsius} Celsius!")
}
