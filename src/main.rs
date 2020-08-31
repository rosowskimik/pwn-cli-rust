use sha1::Sha1;
use thiserror::Error;

type Result<T> = std::result::Result<T, Err>;

#[async_std::main]
async fn main() -> Result<()> {
    let password = get_password()?;
    let password_hash = Sha1::from(password).hexdigest().to_uppercase();

    let hash_list = surf::get(format!(
        "https://api.pwnedpasswords.com/range/{}",
        &password_hash[..5]
    ))
    .recv_string()
    .await?;

    match hash_list
        .lines()
        .find(|line| &password_hash[5..] == &line[..35])
    {
        Some(tail_match) => println!("Your password was found {} time(s)", &tail_match[36..]),
        None => println!("Your password is safe"),
    };

    Ok(())
}

fn get_password() -> Result<String> {
    loop {
        eprint!("Please enter your password: ");
        let p = rpassword::read_password()?;

        if !p.is_empty() {
            break Ok(p);
        }

        eprintln!("Your password can't be empty!");
    }
}

#[derive(Error, Debug)]
enum Err {
    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    NetError(#[from] surf::Exception),
}
