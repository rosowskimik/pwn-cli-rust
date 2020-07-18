use sha1::{Digest, Sha1};

pub fn hash_string(input: String) -> String {
    let hash = format!("{:02x}", Sha1::digest(input.as_bytes()));
    hash.to_uppercase()
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
            String::from("9BC34549D565D9505B287DE0CD20AC77BE1D3F2C")
        );
    }
}
