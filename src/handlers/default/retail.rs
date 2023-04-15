use super::prelude::*;

lazy_static! {

static ref RETAIL : RegexHandler = RegexHandler {
    regex: Regex::new(r"\bRetail\b").unwrap(),
    remove_match: true
};

}

pub fn handle_retail(input: &str) -> (String, Vec<String>) {
    let mut result = input.to_string();
    RETAIL.handle(&mut result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retail() {
        let name = "The.Movie.2019.Retail.1080p.BluRay.x264-FOO";
        let (new_name, matches) = handle_retail(name);
        assert_eq!(new_name, "The.Movie.2019.Retail.1080p.BluRay.x264-FOO");
        assert_eq!(matches, Vec::<String>::new());
    }

    #[test]
    fn test_neither() {
        let name = "The.Movie.2019.1080p.BluRay.x264-FOO";
        let (new_name, matches) = handle_retail(name);
        assert_eq!(new_name, "The.Movie.2019.1080p.BluRay.x264-FOO");
        assert_eq!(matches, Vec::<String>::new());
    }
}