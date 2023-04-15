use super::prelude::*;

lazy_static! {

static ref CONVERT : RegexHandler = RegexHandler {
    regex: Regex::new(r"CONVERT").unwrap(),
    remove_match: false
};
}

pub fn handle_extended(input: &str) -> (String, Vec<String>) {
    let mut result = input.to_string();
    CONVERT.handle(&mut result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert() {
        let name = "The.Movie.2019.CONVERT.1080p.BluRay.x264-FOO";
        let (new_name, matches) = handle_extended(name);
        assert_eq!(new_name, "The.Movie.2019.CONVERT.1080p.BluRay.x264-FOO");
        assert_eq!(matches, Vec::<String>::new());
    }
}