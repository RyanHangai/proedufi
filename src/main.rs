use std::io::{self, Write};

fn welcome_message() {
    println!("Welcome to Proedufi!");
    println!("Your financial management app");
}

fn trim_text(text: &str) -> String {
    let trimmed = text.trim();

    let mut result = String::new();
    let mut space_between_words = false;

    for ch in trimmed.chars() {
        if ch.is_whitespace() {
            if !space_between_words {
                result.push(' ');
                space_between_words = true;
            }
        } else {
            result.push(ch);
            space_between_words = false;
        }
    }

    result.trim_end().to_string()
}

fn validate_email(email: &str) -> bool {
    let mut has_at = false;
    let mut has_dot = false;

    for ch in email.chars() {
        if ch == ' ' {
            println!("Email cannot have spaces");
            return false;
        }
        if ch == '@' {
            if has_at {
                println!("Email cannot have two \"@\" (at signs)");
                return false;
            }
            has_at = true;
        }
        if ch == '.' {
            has_dot = true;
        }
    }

    if !has_at || !has_dot {
        return false;
    }

    if email.starts_with('@') || email.starts_with('.') || email.ends_with('@') || email.ends_with('.') {
        return false;
    }

    true
}

fn validate_password(password: &str) -> bool {
    let length = password.len();

    if length < 8 || length > 128 {
        println!("Password must be between 8 and 128 characters");
        return false;
    }

    let mut has_uppercase = false;
    let mut has_lowercase = false;
    let mut has_digit = false;
    let mut has_special = false;

    for ch in password.chars() {
        if ch == ' ' {
            println!("Password cannot have spaces");
            return false;
        }
        if ch.is_uppercase() {
            has_uppercase = true;
        } else if ch.is_lowercase() {
            has_lowercase = true;
        } else if ch.is_ascii_digit() {
            has_digit = true;
        } else if ch.is_ascii_punctuation() {
            has_special = true;
        }
    }

    if has_uppercase && has_lowercase && has_digit && has_special {
        true
    } else {
        println!("Password must have uppercase, lowercase, numbers and special character (!@#%$&*)");
        false
    }
}

fn create_account() {
    println!("You chose to create an account");
    println!("[0] To go back");

    let mut name = String::new();
    let mut email = String::new();
    let mut password = String::new();

    print!("Enter your name: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = trim_text(&input);

    if input == "0" {
        println!("You went back!\n");
        return;
    } else if input.is_empty() {
        println!("Enter your name first!");
    } else if input.len() > 60 {
        println!("Name too long, enter your real name!");
    } else if !validate_name(&input) {
        println!("Name can only have letters");
    } else {
        name = input;
    }

    print!("Enter your email: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = trim_text(&input);

    if input == "0" {
        println!("You went back!\n");
        return;
    } else if input.is_empty() {
        println!("Enter your email first!");
    } else if !validate_email(&input) {
        println!("Enter a valid email");
    } else {
        email = input;
    }

    print!("Enter a password: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = trim_text(&input);

    if input == "0" {
        println!("You went back!\n");
        return;
    } else if input.is_empty() {
        println!("Enter your email first!");
    } else if !validate_password(&input) {
        println!();
    } else {
        password = input;
    }
}

fn main() {
    welcome_message();

    loop {
        println!("Choose an option:");
        println!("[1] Create account");
        println!("[2] Log in");
        println!("[3] Continue without account");
        println!("[0] Exit!");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: i32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Choose a valid option");
                continue;
            }
        };

        match choice {
            1 => create_account(),
            2 => println!("Log into your account"),
            3 => println!("Ok!"),
            0 => {
                println!("See you next time!");
                break;
            }
            _ => println!("Choose a valid option"),
        }
    }
}
