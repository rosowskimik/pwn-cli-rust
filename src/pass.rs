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

pub fn search_pass<'a>(tail: &str, list: &'a str) -> Option<&'a str> {
    for line in list.lines() {
        let (to_cmp, count) = line.split_at(35);
        if tail == to_cmp {
            return Some(&count[1..]);
        }
    }
    return None;
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn finds_match() {
        let tail = "549D565D9505B287DE0CD20AC77BE1D3F2C";
        let list = "549D565D9505B287DE0CD20AC77BE1D3F2C:13\r\n02F52CAFE6556978D0ED9866118C7ECE4EF:3\r\n0857068D3CCC83ACB1659C6B816141C8BBB:19\r\n";

        assert_eq!(search_pass(tail, list), Some("13"));
    }

    #[test]
    fn returns_none() {
        let tail = "0CA7B56BBAA81771F33B071A13E736B9BF0";
        let list = "549D565D9505B287DE0CD20AC77BE1D3F2C:13\r\n02F52CAFE6556978D0ED9866118C7ECE4EF:3\r\n0857068D3CCC83ACB1659C6B816141C8BBB:19\r\n";

        assert_eq!(search_pass(tail, list), None);
    }
}
