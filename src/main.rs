use std::io;

use car_maintenance::users::users::{self, User, UserRole};

enum MenuState {
    MainMenu,
    DefaultMenu,
    VehicleOptions,
    AccountOptions,
}

fn main() {
    // Initial state of the app - it will change depending on user choices
    let mut state = MenuState::MainMenu;

    println!("Let's begin to monitor your car's health!");
    println!("--------------------------------------");

    loop {
        match state {
            // Main menu options
            MenuState::MainMenu => {
                println!("Main Menu: (1-0)");
                println!("1. Log In");
                println!("2. Sign In");
                println!("0. Exit");

                match get_user_input_str() {
                    Ok(choice) => match choice.as_str() {
                        "1" => {
                            let login_attempt = get_user_input("login");
                            let password_attempt = get_user_input("password");

                            //match users::log_in(&login_attempt, password_attempt) // we won't need password after this so we can consume it
                            // println!("Logging in...");                            
                            // println!("Hello {}, nice to have you back!", logged_user.login);
                            state = MenuState::DefaultMenu;
                        }
                        "2" => {
                            println!("We're thrilled to have you!");
                            let logged_user: users::User = create_user_from_input();
                            // TODO: sign_in();
                            state = MenuState::DefaultMenu;
                        }
                        "0" => {
                            println!("Goodbye!");
                            break;
                        }
                        _ => println!("Mismatched choice number! Try again"),
                    },
                    Err(err) => eprintln!("Error: {}", err),
                }
            },
            // Default Menu after logging/signing in
            MenuState::DefaultMenu => {
                println!("Choose option (1-9):");
                println!("1. Show next maintenance");
                println!("2. Show history");
                println!("3. Show odometer");
                println!("4. Add maintenance");
                println!("5. Edit existing maintenance");
                println!("6. Remove existing maintenance");
                println!("8. Vehicle options");
                println!("9. Account options");
                println!("0. Exit");

                match get_user_input_str() {
                    Ok(choice) => match choice.as_str() {
                        "1" => todo!(),
                        "2" => todo!(),
                        "3" => todo!(),
                        "4" => todo!(),
                        "5" => todo!(),
                        "6" => todo!(),
                        "8" => state = MenuState::VehicleOptions,
                        "9" => state = MenuState::AccountOptions,
                        "0" => {
                            println!("Goodbye!");
                            // TODO: Remember the state and log in the previous user
                            break;
                        }
                        _ => println!("Mismatched choice number! Try again"),
                    },
                    Err(err) => eprintln!("Error: {}", err),
                }
            },
            // Vehicle panel
            MenuState::VehicleOptions => {
                println!("Choose option (1-4):");
                println!("1. Change default vehicle");
                println!("2. Add new vehicle");
                println!("3. Edit existing vehicle");
                println!("4. Remove vehicle");
                println!("0. Back");

                match get_user_input_str() {
                    Ok(choice) => match choice.as_str() {
                        "1" => todo!(),
                        "2" => todo!(),
                        "3" => todo!(),
                        "4" => todo!(),
                        "0" => state = MenuState::DefaultMenu,
                        _ => println!("Mismatched choice number! Try again"),
                    },
                    Err(err) => eprintln!("Error: {}", err),
                }
            },
            // Account panel
            MenuState::AccountOptions => {
                println!("Choose option (1-9):");
                // TODO: more account options
                println!("1. Log out");
                println!("9. Remove account");
                println!("0. Back");

                match get_user_input_str() {
                    Ok(choice) => match choice.as_str() {
                        "1" => {
                            println!("Logging out!");
                            state = MenuState::MainMenu;
                            // TODO: Remember the state and don't log the user
                        }
                        "2" => todo!(),
                        "3" => todo!(),
                        "4" => todo!(),
                        "9" => todo!(),
                        "0" => state = MenuState::DefaultMenu,
                        _ => println!("Mismatched choice number! Try again"),
                    },
                    Err(err) => eprintln!("Error: {}", err),
                }
            },
        }
    }
}

//Getting user information
//get usr info as string
fn get_user_input_str() -> Result<String, io::Error> {
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).unwrap();
    
    let trimmed_input = input.trim().to_string();

    if trimmed_input.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Your input is empty!"));
    }
    Ok(trimmed_input)
}
//wraper around get_user_input_str
fn get_user_input(prompt: &str) -> String {
    loop {
        println!("Enter {}: ", prompt);
        let input = get_user_input_str();
        match input {
            Ok(value) if !value.is_empty() => return value,
            _ => println!("Invalid input! Please try again."),
        }
    }
}

//Create new user
pub fn create_user_from_input() -> User {
    let name = get_user_input("Name");
    let login = get_user_input("Login");
    let password = get_user_input("Password");
    let email = get_user_input("Email");
    let phone = get_user_input("Phone");

    let mut user = User {
        user_id: 0, // Auto-increment w bazie danych
        name,
        login,
        password_hash: String::new(),
        email,
        phone,
        role: UserRole::User,
        salt: String::new(),
    };
    
        user.hash_password(&password);
    
        user
}

