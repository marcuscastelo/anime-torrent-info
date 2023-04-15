use super::prelude::*;

lazy_static! {

static ref REGION : RegexHandler = RegexHandler {
    regex: Regex::new(r"R\d").unwrap(),
    remove_match: false
};

}

pub fn handle_region(input: &str) -> (String, Vec<String>) {
    let mut result = input.to_string();
    REGION.handle(&mut result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_region() {
        let name = "The.Movie.2019.R2.1080p.BluRay.x264-FOO";
        let (new_name, matches) = handle_region(name);
        assert_eq!(new_name, "The.Movie.2019.R2.1080p.BluRay.x264-FOO");
        assert_eq!(matches, Vec::<String>::new());
    }

    #[test]
    fn test_neither() {
        let name = "The.Movie.2019.1080p.BluRay.x264-FOO";
        let (new_name, matches) = handle_region(name);
        assert_eq!(new_name, "The.Movie.2019.1080p.BluRay.x264-FOO");
        assert_eq!(matches, Vec::<String>::new());
    }
}