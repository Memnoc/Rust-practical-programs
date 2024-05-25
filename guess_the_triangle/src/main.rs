use rand::Rng;
use std::io::{self, Write};

// INFO:
// retto = 90
// acuto < 90
// ottuso > 90
// piatto = 180
// giro = 360

fn main() {
    println!("== Hello, world! ==");

    let random_angle = rand::thread_rng().gen_range(1..=190);

    println!("What kind of angle is: {}", random_angle);
    io::stdout().flush().unwrap();

    let mut angle_type = String::new();

    io::stdin()
        .read_line(&mut angle_type)
        .expect("failed to read line");

    read_angle(angle_type);
}

fn read_angle(angle: String) {
    println!("{}", angle);
}
