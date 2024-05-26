use clap::{Arg, Command};
use colored::*;
use rand::Rng;
use std::io::{self, Write};

fn main() {
    let matches = Command::new("== Programma per imparare gli angoli ==")
        .version("1.0")
        .author("Matteo Stara <smartdroidesign@gmail.com>")
        .about("Aiutiamo i giovani studenti ad imparare i diversi tipi di angoli")
        .arg(
            Arg::new("start")
                .long("start")
                .short('s')
                .help("Comincia la sessione per imparare gli angoli"),
        )
        .get_matches();

    if matches.contains_id("start") {
        angle_identification_session();
    } else {
        println!("Usa --start per iniziare la sessione");
    }

    fn angle_identification_session() {
        loop {
            let random_angle = rand::thread_rng().gen_range(1..=360);
            println!(
                "{} {} {}?",
                "Che tipo di angolo ha".bold(),
                random_angle.to_string().yellow().bold(), // Convert the angle to a string and make it bold
                "gradi".green().bold()
            );

            let mut angle_type = String::new();
            println!(
                "{}",
                "Srivi il tipo di anglo o scrivi 'esci' per uscire: ".cyan()
            );
            io::stdout().flush().unwrap();
            io::stdin()
                .read_line(&mut angle_type)
                .expect("failed to read line");

            if angle_type.trim().eq_ignore_ascii_case("esci") {
                println!("Prigramma terminato.");
                break;
            }

            validate_angle(&angle_type.trim().to_lowercase(), random_angle);
        }
    }

    fn validate_angle(user_input: &str, angle: i32) {
        let correct_type = match angle {
            90 => "retto",
            0 | 360 => "giro",
            1..=89 => "acuto",
            91..=179 => "ottuso",
            180 => "piatto",
            181..=359 => "piatto",
            _ => "non valido",
        };

        validate_user_input(user_input, correct_type, angle);
    }

    fn validate_user_input(user_input: &str, correct_type: &str, angle: i32) {
        if user_input == correct_type {
            println!(
                "{} L'angolo di {} gradi è un angolo {}.",
                "GIUSTO!".red().bold(),
                angle.to_string().yellow().bold(),
                correct_type.green(),
            );
        } else {
            println!(
                "{} L'angolo di {} gradi è un angolo {}.",
                "SBAGLIATO!".red().bold(),
                angle.to_string().yellow().bold(),
                correct_type.green(),
            );
        }
    }
}
