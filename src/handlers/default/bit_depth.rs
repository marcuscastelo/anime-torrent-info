use super::prelude::*;

lazy_static! {
//     parser.addHandler("bitDepth", /(?:8|10|12)[- ]?bit/i, lowercase, { remove: true });
static ref BIT_DEPTH_1 : RegexHandler = RegexHandler {
    regex: Regex::new(r"(?i)(?:8|10|12)[- ]?bit").unwrap(),
    remove_match: true
};

//     parser.addHandler("bitDepth", /\bhevc\s?10\b/i, value("10bit"));
static ref BIT_DEPTH_2 : RegexHandler = RegexHandler {
    regex: Regex::new(r"(?i)\bhevc\s?10\b").unwrap(),
    remove_match: false
};

//     parser.addHandler("bitDepth", /\bhdr10/i, value("10bit"));
static ref BIT_DEPTH_3 : RegexHandler = RegexHandler {
    regex: Regex::new(r"(?i)\bhdr10").unwrap(),
    remove_match: false
};

//TODO: treat non-regexp handlers
//     parser.addHandler("bitDepth", ({ result }) => {
//         if (result.bitDepth) {
//             result.bitDepth = result.bitDepth.replace(/[ -]/, "");
//         }
//     });

}

pub fn all() -> Vec<&'static RegexHandler> {
    vec![&BIT_DEPTH_1, &BIT_DEPTH_2, &BIT_DEPTH_3]
}

pub fn handle_bit_depth(input: &str) -> (String, Vec<String>) {
    let mut result = input.to_string();
    let mut matches = Vec::<String>::new();
    for handler in all() {
        let (new_result, new_matches) = handler.handle(&mut result);
        result = new_result;
        matches.extend(new_matches);
    }
    (result, matches)
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- BIT_DEPTH_1 ---

    #[test]
    fn test_bit_depth_1_match() {
        let name = "[SweetSub][天国大魔境][Heavenly Delusion][02][WebRip][1080P][AVC 8bit][简日双语]";
        let (new_name, matches) = handle_bit_depth(name);
        assert_eq!(new_name, "[SweetSub][天国大魔境][Heavenly Delusion][02][WebRip][1080P][AVC ][简日双语]");
        assert_eq!(matches, Vec::<String>::new());
    }

    #[test]
    fn test_bit_depth_1_no_match() {
        let name = "The.Movie.2019.1080p.BluRay.x264-FOO";
        let (new_name, matches) = handle_bit_depth(name);
        assert_eq!(new_name, "The.Movie.2019.1080p.BluRay.x264-FOO");
        assert_eq!(matches, Vec::<String>::new());
    }

    // --- BIT_DEPTH_2 ---

    #[test]
    fn test_bit_depth_2_match() {
        let name = "[Anime Time] Heavenly Delusion - 03 [1080p][HEVC 10 x265][AAC][Multi Sub] [Weekly] (Tengoku Daimakyou)";
        let (new_name, matches) = handle_bit_depth(name);
        assert_eq!(new_name, "[Anime Time] Heavenly Delusion - 03 [1080p][HEVC 10 x265][AAC][Multi Sub] [Weekly] (Tengoku Daimakyou)");
        assert_eq!(matches, Vec::<String>::new());
    }

    #[test]
    fn test_bit_depth_2_no_match() {
        let name = "The.Movie.2019.1080p.BluRay.x264-FOO";
        let (new_name, matches) = handle_bit_depth(name);
        assert_eq!(new_name, "The.Movie.2019.1080p.BluRay.x264-FOO");
        assert_eq!(matches, Vec::<String>::new());
    }

    // --- BIT_DEPTH_3 ---

    #[test]
    fn test_bit_depth_3_match() {
        let name = "The.Movie.2019.1080p.BluRay.x264-FOO.HDR10";
        let (new_name, matches) = handle_bit_depth(name);
        assert_eq!(new_name, "The.Movie.2019.1080p.BluRay.x264-FOO.HDR10");
        assert_eq!(matches, Vec::<String>::new());
    }

    #[test]
    fn test_bit_depth_3_no_match() {
        let name = "The.Movie.2019.1080p.BluRay.x264-FOO";
        let (new_name, matches) = handle_bit_depth(name);
        assert_eq!(new_name, "The.Movie.2019.1080p.BluRay.x264-FOO");
        assert_eq!(matches, Vec::<String>::new());
    }
}