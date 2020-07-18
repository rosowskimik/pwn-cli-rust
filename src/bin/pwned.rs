use pwned::{hash, pass, request};
use std::process;

fn main() {
    let password = pass::read_password();
    let hashed_password = hash::hash_string(password);
    let (head, tail) = hashed_password.split_at(5);

    let response_list = match request::search_head(head) {
        Ok(response) => response.text().unwrap(),
        Err(_) => {
            eprintln!("Check your internet connection");
            process::exit(1);
        }
    };

    match pass::search_pass(tail, &response_list) {
        Some(count) => {
            println!("Your password was found {} times.", count);
        }
        None => {
            println!("Your password is safe.");
        }
    }
}
