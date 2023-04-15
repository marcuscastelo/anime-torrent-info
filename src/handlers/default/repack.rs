use super::prelude::*;

lazy_static! {

static ref REPACK : RegexHandler = RegexHandler {
    regex: Regex::new(r"REPACK|RERIP").unwrap(),
    remove_match: false
};

}

pub fn handle_repack(input: &str) -> (String, Vec<String>) {
    let mut result = input.to_string();
    REPACK.handle(&mut result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repack() {
        let name = "The.Movie.2019.REPACK.1080p.BluRay.x264-FOO";
        let (new_name, matches) = handle_repack(name);
        assert_eq!(new_name, "The.Movie.2019.REPACK.1080p.BluRay.x264-FOO");
        assert_eq!(matches, Vec::<String>::new());
    }

    #[test]
    fn test_rerip() {
        let name = "The.Movie.2019.RERIP.1080p.BluRay.x264-FOO";
        let (new_name, matches) = handle_repack(name);
        assert_eq!(new_name, "The.Movie.2019.RERIP.1080p.BluRay.x264-FOO");
        assert_eq!(matches, Vec::<String>::new());
    }

    #[test]
    fn test_neither() {
        let name = "The.Movie.2019.1080p.BluRay.x264-FOO";
        let (new_name, matches) = handle_repack(name);
        assert_eq!(new_name, "The.Movie.2019.1080p.BluRay.x264-FOO");
        assert_eq!(matches, Vec::<String>::new());
    }
}