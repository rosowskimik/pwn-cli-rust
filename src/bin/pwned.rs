use pwned::read;

fn main() {
    let password = read::read_password();
    dbg!(password);
}
