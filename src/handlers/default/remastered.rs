use super::prelude::*;

lazy_static! {

static ref REMASTERED : RegexHandler = RegexHandler {
    regex: Regex::new(r"\bRemaster(?:ed)?\b").unwrap(),
    remove_match: false
};

}

pub fn handle_remastered(input: &str) -> (String, Vec<String>) {
    let mut result = input.to_string();
    REMASTERED.handle(&mut result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remastered() {
        let name = "The.Movie.2019.Remastered.1080p.BluRay.x264-FOO";
        let (new_name, matches) = handle_remastered(name);
        assert_eq!(new_name, "The.Movie.2019.Remastered.1080p.BluRay.x264-FOO");
        assert_eq!(matches, Vec::<String>::new());
    }

    #[test]
    fn test_remaster() {
        let name = "The.Movie.2019.Remaster.1080p.BluRay.x264-FOO";
        let (new_name, matches) = handle_remastered(name);
        assert_eq!(new_name, "The.Movie.2019.Remaster.1080p.BluRay.x264-FOO");
        assert_eq!(matches, Vec::<String>::new());
    }

    #[test]
    fn test_neither() {
        let name = "The.Movie.2019.1080p.BluRay.x264-FOO";
        let (new_name, matches) = handle_remastered(name);
        assert_eq!(new_name, "The.Movie.2019.1080p.BluRay.x264-FOO");
        assert_eq!(matches, Vec::<String>::new());
    }
}