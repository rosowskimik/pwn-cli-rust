use sha1::{Digest, Sha1};

pub fn hash_string(password: String) -> String {
    format!("{:02x}", Sha1::digest(password.as_bytes()))
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn hashes_string() {
        let password = String::from("test1234");
        let hashed_password = hash_string(password);
        assert_eq!(
            hashed_password,
            String::from("9bc34549d565d9505b287de0cd20ac77be1d3f2c")
        );
    }
}
