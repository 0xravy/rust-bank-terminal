use std::io;
use std::io::Write;

pub fn my_input(placeholder: String) -> String {
    print!("{}", placeholder);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}
