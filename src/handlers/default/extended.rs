use super::prelude::*;

lazy_static! {

static ref EXTENDED : RegexHandler = RegexHandler {
    regex: Regex::new(r"EXTENDED").unwrap(),
    remove_match: false
};

}

pub fn handle_extended(input: &str) -> (String, Vec<String>) {
    let mut result = input.to_string();
    EXTENDED.handle(&mut result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extended() {
        let name = "The.Movie.2019.EXTENDED.1080p.BluRay.x264-FOO";
        let (new_name, matches) = handle_extended(name);
        assert_eq!(new_name, "The.Movie.2019.EXTENDED.1080p.BluRay.x264-FOO");
        assert_eq!(matches, Vec::<String>::new());
    }
}