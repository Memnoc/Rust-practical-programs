use rand::Rng;
use std::io::{self, Write};

fn main() {
    println!("== Benvenuto nel programma di apprendimento degli angoli ==");

    let mut rng = rand::thread_rng();
    let random_angle = rng.gen_range(1..=360);

    println!("Che tipo di angolo è: {} gradi?", random_angle);
    io::stdout().flush().unwrap();

    let mut angle_type = String::new();

    io::stdin()
        .read_line(&mut angle_type)
        .expect("Errore nella lettura del tipo di angolo");

    let angle_type = angle_type.trim().to_lowercase();
    validate_angle(&angle_type, random_angle);
}

fn validate_angle(user_input: &str, angle: i32) {
    let correct_type = match angle {
        90 => "retto",
        0 | 360 => "giro",
        1..=89 => "acuto",
        91..=179 => "ottuso",
        180 => "piatto",
        181..=359 => "piatto", // This range seems incorrect; it should be 'ottuso' until 359
        _ => "non valido",
    };

    validate_user_input(user_input, correct_type, angle);
}

fn validate_user_input(user_input: &str, correct_type: &str, angle: i32) {
    if user_input == correct_type {
        println!(
            "Corretto!! L'angolo di {} gradi è un angolo {}.",
            angle, correct_type
        );
    } else {
        println!(
            "Sbagliato!! L'angolo di {} gradi è un angolo {}, non un angolo {}.",
            angle, correct_type, user_input
        );
    }
}
