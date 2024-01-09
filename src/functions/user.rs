use super::input::my_input;

#[derive(Debug)]
pub struct User {
    name: String,
    pass: String,
    bank: i32,
    cash: i32,
}

pub fn help() {
    println!("+------------------------------------------+");
    println!("| what you want do now ?                   |");
    println!("| Enter 'h' if you need help.              |");
    println!("| Enter 's' to see all users in bank.      |");
    println!("| Enter 'a' to add new user in bank.       |");
    println!("| Enter 'r' to remove user from bank.      |");
    println!("| Enter 'f' to search about uses in bank.  |");
    println!("| Enter 'l' to take loan in you're banke.  |");
    println!("| Enter 'c' to get cash from bank.         |");
    println!("+------------------------------------------+");
}

pub fn add_user(users: &mut Vec<User>) {
    let mut username: String;
    let mut password: String;

    loop {
        username = my_input("Enter your name: ".to_string());

        if username != "".to_string().trim() {
            break;
        }
        println!("you must enter you're name");
    }

    if username.parse::<i32>().is_ok() {
        println!("you can't use numbers in you're name.");
        return;
    } else if find_user(users, &username).is_some() {
        println!("this username is used try diffrent name!");
        return;
    }

    loop {
        password = my_input("Enter your pass: ".to_string());

        if password != "".to_string().trim() {
            break;
        }
        println!("you must enter you're pass");
    }

    users.push(User {
        name: username,
        pass: password,
        bank: 0,
        cash: 0,
    });
}

pub fn remove_user(users: &mut Vec<User>, index: usize) {
    users.remove(index);
}

pub fn find_user<'a>(users: &'a mut Vec<User>, name: &'a str) -> Option<(usize, &'a mut User)> {
    for (i, user) in users.iter_mut().enumerate() {
        if user.name == name {
            return Some((i, user));
        }
    }
    None
}

pub fn get_loan(user: &mut User, loan: String) {
    user.bank += loan.parse::<i32>().unwrap();
}

pub fn get_cash(user: &mut User, cash: String) {
    let c = cash.parse::<i32>().unwrap();
    let bc = user.bank - c;

    if bc < 0 {
        println!("you dont have enough mony in bank");
        return;
    } else if c == 0 {
        println!("You must enter an amount");
        return;
    }

    user.cash += c;
    user.bank = bc;
}
