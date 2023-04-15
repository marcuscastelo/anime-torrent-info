use super::handler::Handler;
use fancy_regex::Regex;

pub struct RegexHandler {
    pub regex: Regex,
    pub remove_match: bool,
}

impl Handler for RegexHandler {
    fn handle(&self, filename: &str) -> (String, Vec<String>) {
        // Returns tuple of (filename, match)
        let caps = match self.regex.captures(filename).unwrap() {
            Some(caps) => caps,
            None => return (filename.to_string(), vec![]),
        };

        // Get every match group as a vec of strings
        let groups: Vec<String> = caps
            .iter()
            .skip(1)
            .map(|m| m.map(|m| m.as_str().to_string()).unwrap_or_default())
            .collect();

        //if remove_match is true, remove match from filename. Example: ("[HorribleSubs]  [720p].mkv", "One Piece - 123")
        if self.remove_match {
            let match_start = caps.get(0).unwrap().start();
            let match_end = caps.get(0).unwrap().end();

            let mut filename = filename.to_string();
            filename.replace_range(match_start..match_end, "");

            return (filename, groups);
        } else {
            return (filename.to_string(), groups);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use paste::paste;

    #[test]
    fn test_capture() {
        let (input, regex, expected) = 
        (
            "[HorribleSubs] One Piece - 123 [720p].mkv",
            "One Piece - 123",
            (
                "[HorribleSubs] One Piece - 123 [720p].mkv".to_owned(),
                vec![]
            )
        );

        let regex = Regex::new(regex).unwrap();

        assert_eq!(expected, RegexHandler{regex, remove_match: false}.handle(input));
    }

    macro_rules! test_handle {
        ($($name:ident: $value:expr,)*) => {
            $(
            paste! {
                    #[test]
                    fn [<test_ $name>]()
                    {
                        let (input, regex, expected) = $value;

                        let regex = Regex::new(regex).unwrap();

                        assert_eq!(expected, RegexHandler{regex, remove_match: false}.handle(input));
                    }
                }
            )*
        }
    }

    test_handle! {
        t1: (
            "[HorribleSubs] One Piece - 123 [720p].mkv",
            "(One Piece - 123)",
            (
                "[HorribleSubs] One Piece - 123 [720p].mkv".to_owned(),
                vec!["One Piece - 123".to_owned()]
            )
        ),
    }

    macro_rules! test_handle_remove {
        ($($name:ident: $value:expr,)*) => {
            $(
                paste! {
                        #[test]
                        fn [<test_ $name _remove>]()
                        {
                            let (input, regex, expected) = $value;

                            let regex = Regex::new(regex).unwrap();

                            assert_eq!(expected, RegexHandler{regex, remove_match: true}.handle(input));
                        }
                    }
                )*
        }
    }

    test_handle_remove! {
        t1: (
            "[HorribleSubs] One Piece - 123 [720p].mkv",
            "(One Piece - 123)",
            (
                "[HorribleSubs]  [720p].mkv".to_owned(),
                vec!["One Piece - 123".to_owned()]
            )
        ),
    }
}
