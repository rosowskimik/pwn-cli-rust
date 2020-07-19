use async_std::task;
use pwned::{hash, pass};

fn main() -> Result<(), surf::Error> {
    let password = pass::read_password();
    let hashed_password = hash::hash_string(password);
    let (head, tail) = hashed_password.split_at(5);
    task::block_on(async {
        let res = pass::fetch_password_list(head).await?;
        match pass::search_list(tail, &res) {
            Some(count) => println!("Your password was found {} times.", count),
            None => println!("Your password is safe."),
        };
        Ok(())
    })
}
