mod functions;
use functions::input::my_input;
use functions::user::*;

use std::{env, process};

fn main() {
    let mut users: Vec<User> = vec![];
    let mut input: String;
    let system = env::consts::OS;

    if let "windows" = system {
        process::Command::new("cls").status().unwrap();
    } else {
        process::Command::new("clear").status().unwrap();
    }

    help();
    loop {
        println!();
        input = my_input("-> ".to_string());

        println!(":++++++++++++++++++++++++++++:");
        match input.to_string().trim() {
            "h" => {
                help();
            }
            "s" => {
                println!("{:?}", users);
            }
            "a" => {
                add_user(&mut users);
                println!("{:?}", users);
            }
            "r" => {
                let name = my_input("Enter username who you want remove: ".to_string());
                let find = find_user(&mut users, &name);

                match find {
                    Some((i, _)) => {
                        remove_user(&mut users, i);
                        println!("{:?}", users);
                    }
                    None => {
                        println!("User '{name}' does not exit.");
                    }
                }
            }
            "f" => {
                let username = my_input("Enter username to find: ".to_string());
                let find = find_user(&mut users, &username);

                match find {
                    Some(_) => println!("{:?}", find),
                    None => println!("we cant find this user"),
                }
            }
            "l" => {
                let name = my_input("Enter username who you want to give him loan: ".to_string());
                let find = find_user(&mut users, &name);

                match find {
                    Some((_, user)) => {
                        let loan =
                            my_input("Enter loan number to give to user in bank: ".to_string());
                        get_loan(user, loan);
                        println!("{:?}", user);
                    }
                    None => {
                        println!("User '{name}' does not exit.");
                    }
                }
            }
            "c" => {
                let name =
                    my_input("Enter username who you want to take cash from bank: ".to_string());
                let find = find_user(&mut users, &name);

                match find {
                    Some((_, user)) => {
                        let cash =
                            my_input("Enter cash number to give to user from bank: ".to_string());
                        get_cash(user, cash);
                        println!("{:?}", user);
                    }
                    None => {
                        println!("User '{name}' does not exit.");
                    }
                }
            }
            _ => println!("you must to enter one of argument in top"),
        }
        println!(":============================:");
    }
}
