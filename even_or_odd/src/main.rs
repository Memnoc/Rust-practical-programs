use std::io::{self, Write};

fn main() {
    println!("== Even or Odd ==");
    println!("Is the number even or odd?");

    println!("Enter a number:");
    io::stdout().flush().unwrap();

    let mut number_str = String::new();

    io::stdin()
        .read_line(&mut number_str)
        .expect("failed to read line");

    let number: i32 = number_str
        .trim()
        .parse()
        .expect("You have to write a number!");

    print_even_or_odd(number);
}

fn print_even_or_odd(number: i32) {
    match number % 2 {
        0 => println!("{} is even", number),
        _ => println!("{} is odd", number),
    }
}
