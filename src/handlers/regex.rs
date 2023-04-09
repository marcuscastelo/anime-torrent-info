use super::handler::Handler;
use paste::paste;
use regex::Regex;

struct RegexHandler {
    regex: Regex,
    remove_match: bool,
}

impl Handler for RegexHandler {
    fn handle(&self, filename: &str) -> (String, String) {
        // Returns tuple of (filename, match)
        let caps = self.regex.captures(filename).unwrap();
        let match_ = caps.get(1).unwrap().as_str();

        //if remove_match is true, remove match from filename. Example: ("[HorribleSubs]  [720p].mkv", "One Piece - 123")
        if self.remove_match {
            let match_start = caps.get(0).unwrap().start();
            let match_end = caps.get(0).unwrap().end();

            let mut filename = filename.to_string();
            filename.replace_range(match_start..match_end, "");

            return (filename, match_.to_string());
        } else {
            return (filename.to_string(), match_.to_string());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
                "One Piece - 123".to_owned()
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
                "One Piece - 123".to_owned()
            )
        ),
    }
}
