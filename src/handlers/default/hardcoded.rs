use super::prelude::*;

lazy_static! {

static ref HARDCODED : RegexHandler = RegexHandler {
    regex: Regex::new(r"HC|HARDCODED").unwrap(),
    remove_match: false
};

}

pub fn handle_hardcoded(input: &str) -> (String, Vec<String>) {
    let mut result = input.to_string();
    HARDCODED.handle(&mut result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hardcoded() {
        let name = "The.Movie.2019.HARDCODED.1080p.BluRay.x264-FOO";
        let (new_name, matches) = handle_hardcoded(name);
        assert_eq!(new_name, "The.Movie.2019.HARDCODED.1080p.BluRay.x264-FOO");
        assert_eq!(matches, Vec::<String>::new());
    }

    #[test]
    fn test_hc() {
        let name = "The.Movie.2019.HC.1080p.BluRay.x264-FOO";
        let (new_name, matches) = handle_hardcoded(name);
        assert_eq!(new_name, "The.Movie.2019.HC.1080p.BluRay.x264-FOO");
        assert_eq!(matches, Vec::<String>::new());
    }

    #[test]
    fn test_neither() {
        let name = "The.Movie.2019.1080p.BluRay.x264-FOO";
        let (new_name, matches) = handle_hardcoded(name);
        assert_eq!(new_name, "The.Movie.2019.1080p.BluRay.x264-FOO");
        assert_eq!(matches, Vec::<String>::new());
    }
}