use reqwest::blocking::Response;
use reqwest::Result;

pub fn search_head(head: &str) -> Result<Response> {
    let mut url = String::from("https://api.pwnedpasswords.com/range/");
    url.push_str(head);

    reqwest::blocking::get(&url)?.error_for_status()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sends_request() {
        let head = "9BC34";
        let response = search_head(head);
        assert!(response.is_ok());
    }

    #[test]
    fn fails_format() {
        let head = "9BC34asd";
        assert!(search_head(head).is_err());
    }
}
