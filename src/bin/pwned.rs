use pwned::{hash, read};

fn main() {
    let password = read::read_password();
    let hashed_password = hash::hash_string(password);
    dbg!(hashed_password);
}
