mod text;
use std::env;
use text::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let options = &args[1..].to_vec();

    if args.len() <= 1 {
        println!("Somethings is missing...");
    }

    for option in options {
        match option.as_str() {
            "--help" => {
                println!("{}", HELP);
            }
            "--purpose" => {
                println!("{} {}", REMINDER, PURPOSE);
            }
            "--known" => {
                println!("{} {}", REMINDER, KNOWN);
            }
            "--rncp" => {
                println!("{} {}", REMINDER, RNCP);
            }
            "--contact" => {
                println!("{} {}", REMINDER, CONTACT);
            }
            "--skills" => {
                println!("{} {}", REMINDER, SKILLS);
            }
            "--hobbies" => {
                println!("{} {}", REMINDER, HOBBIES);
            }
            "--projects" => {
                println!("{} {}", REMINDER, PROJECTS);
            }
            "--education" => {
                println!("{} {}", REMINDER, EDUCATION);
            }
            "--professional" => {
                println!("{} {}", REMINDER, PROFESSIONAL);
            }
            "--all" => {
                println!(
                    "{} {} {} {} {} {} {} {} {} {}",
                    REMINDER,
                    PURPOSE,
                    KNOWN,
                    RNCP,
                    PROJECTS,
                    PROFESSIONAL,
                    EDUCATION,
                    SKILLS,
                    HOBBIES,
                    CONTACT
                );
            }
            _ => {
                println!("Invalid option consider using borisResume --help");
            }
        }
    }
}
