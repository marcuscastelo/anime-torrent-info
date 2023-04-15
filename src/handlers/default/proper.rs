use super::prelude::*;

lazy_static! {

static ref PROPER : RegexHandler = RegexHandler {
    regex: Regex::new(r"PROPER").unwrap(),
    remove_match: false
};

}

pub fn handle_proper(input: &str) -> (String, Vec<String>) {
    let mut result = input.to_string();
    PROPER.handle(&mut result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proper() {
        let name = "The.Movie.2019.PROPER.1080p.BluRay.x264-FOO";
        let (new_name, matches) = handle_proper(name);
        assert_eq!(new_name, "The.Movie.2019.PROPER.1080p.BluRay.x264-FOO");
        assert_eq!(matches, Vec::<String>::new());
    }

    #[test]
    fn test_neither() {
        let name = "The.Movie.2019.1080p.BluRay.x264-FOO";
        let (new_name, matches) = handle_proper(name);
        assert_eq!(new_name, "The.Movie.2019.1080p.BluRay.x264-FOO");
        assert_eq!(matches, Vec::<String>::new());
    }
}