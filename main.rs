use rand::Rng;
use std::collections::HashSet;
use std::io;

fn main() {
    // Get user input for password settings
    let length = input_usize("Enter the length of the password: ");
    let use_uppercase = input_bool("Include uppercase letters? (y/n): ");
    let use_symbols = input_bool("Include symbols? (y/n): ");
    let password_number = input_usize("How much passwords do you want to create? ");

    // Store previously generated passwords
    let mut previous_passwords = HashSet::new();

    // Generate and print multiple passwords
    for _ in 0..password_number {
        let mut generated = generate_password(length, use_uppercase, use_symbols);
        
        // Ensure the password is unique
        while previous_passwords.contains(&generated) {
            generated = generate_password(length, use_uppercase, use_symbols);
        }

        previous_passwords.insert(generated.clone());
        println!("{} ({} chars)", generated, generated.len());
    }
}

// Function to generate a password based on user preferences
fn generate_password(length: usize, use_uppercase: bool, use_symbols: bool) -> String {
    let mut base_characters = String::from("abcdefghijklmnopqrstuvwxyz0123456789");

    if use_uppercase {
        base_characters.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }
    if use_symbols {
        base_characters.push_str("!@#$%^&*()-_=+[]{}|;:',.<>?/~`");
    }

    let mut rng = rand::thread_rng();
    let password: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..base_characters.len());
            base_characters.chars().nth(idx).unwrap()
        })
        .collect();

    password
}

// Function to get a usize input from the user
fn input_usize(prompt: &str) -> usize {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid number."),
        }
    }
}

// Function to get a boolean input (y/n) from the user
fn input_bool(prompt: &str) -> bool {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim().to_lowercase().as_str() {
            "y" => return true,
            "n" => return false,
            _ => println!("Please enter 'y' or 'n'."),
        }
    }
}
