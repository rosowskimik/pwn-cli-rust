pub fn read_password() -> String {
    loop {
        eprintln!("Please insert your password:");
        match rpassword::read_password() {
            Ok(password) => {
                if password.is_empty() {
                    eprintln!("Password can't be empty");
                    continue;
                }
                break password;
            }
            Err(_) => continue,
        }
    }
}
