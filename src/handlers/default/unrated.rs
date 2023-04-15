use super::prelude::*;

lazy_static! {

static ref UNRATED : RegexHandler = RegexHandler {
    regex: Regex::new(r"\bunrated|uncensored\b").unwrap(),
    remove_match: false
};

}

pub fn handle_unrated(input: &str) -> (String, Vec<String>) {
    let mut result = input.to_string();
    UNRATED.handle(&mut result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unrated() {
        let name = "The.Movie.2019.unrated.1080p.BluRay.x264-FOO";
        let (new_name, matches) = handle_unrated(name);
        assert_eq!(new_name, "The.Movie.2019.unrated.1080p.BluRay.x264-FOO");
        assert_eq!(matches, Vec::<String>::new());
    }

    #[test]
    fn test_uncensored() {
        let name = "The.Movie.2019.uncensored.1080p.BluRay.x264-FOO";
        let (new_name, matches) = handle_unrated(name);
        assert_eq!(new_name, "The.Movie.2019.uncensored.1080p.BluRay.x264-FOO");
        assert_eq!(matches, Vec::<String>::new());
    }

    #[test]
    fn test_neither() {
        let name = "The.Movie.2019.1080p.BluRay.x264-FOO";
        let (new_name, matches) = handle_unrated(name);
        assert_eq!(new_name, "The.Movie.2019.1080p.BluRay.x264-FOO");
        assert_eq!(matches, Vec::<String>::new());
    }
}