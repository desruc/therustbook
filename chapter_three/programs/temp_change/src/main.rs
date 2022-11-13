use std::io;

// Convert temperatures between Fahrenheit and Celsius.
fn main() {
    println!("Please input temperature in Celsius");

    let mut temp = String::new();

    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");

    let temp: u32 = temp.trim().parse().expect("Please type a number");

    let fahrenheit: u32 = (temp * 9 / 5) + 32;

    println!("The temp in Farhenheit is {fahrenheit}");
}
