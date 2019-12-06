use std::io;

fn main() -> io::Result<()> {
    loop {
        greeting();
        let selection: i32 = read_input();
        let temperature = read_temperature();
        let result = match selection {
            1 => fahrenheit(temperature),
            2 => celcius(temperature),
            3 => break,
            _ => continue
        };
        display_result(result);
    }

    Ok(())
}

fn greeting() -> () {
    println!("Welcome to the temperature conversion program");
    println!("1.) Convert celcius to fahrenheit");
    println!("2.) Convert fahrenheit to celcius");
    println!("3.) Quit");
}

fn read_input() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse().expect("Excpected to receive an integer")
}

fn read_temperature() -> f64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse().expect("Excpected to receive an integer")
}

fn fahrenheit(temp :f64) -> f64 {
    temp * 9.0/5.0 + 32.0
}

fn celcius(temp : f64) -> f64 {
    (temp - 32.0) * 5.0/9.0
}

fn display_result(result : f64) -> () {
    println!("The temperature is {}", result);
}
