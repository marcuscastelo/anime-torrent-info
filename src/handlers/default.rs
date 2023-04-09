use super::handler::Handler;
use super::regex::RegexHandler;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref EPISODE_CODE_1: RegexHandler = RegexHandler {
        regex: Regex::new(r"[\[(]([a-zA-Z0-9]{8})[\])](?:\.[a-zA-Z0-9]{1,5}$|$)").unwrap(),
        remove_match: true
    };
    pub static ref EPISODE_CODE_2: RegexHandler = RegexHandler {
        regex: Regex::new(r"\[(\[A-Z0-9]{8})]").unwrap(),
        remove_match: false
    };
}

lazy_static! {
    //     parser.addHandler("resolution", /\b[(\[]?4k[)\]]?\b/i, value("4k"), { remove: true });
    pub static ref RESOLUTION_4K_1: RegexHandler = RegexHandler {
        regex: Regex::new(r"(?i)\b[(\[]?4k[)\]]?\b").unwrap(),
        remove_match: true
    };

    //     parser.addHandler("resolution", /21600?[pi]/i, value("4k"), { skipIfAlreadyFound: false, remove: true });
    pub static ref RESOLUTION_4K_2: RegexHandler = RegexHandler {
        regex: Regex::new(r"21600?[pi]").unwrap(),
        remove_match: true
    };

    //     parser.addHandler("resolution", /[(\[]?3840x\d{4}[)\]]?/i, value("4k"), { remove: true });
    pub static ref RESOLUTION_4K_3: RegexHandler = RegexHandler {
        regex: Regex::new(r"[(\[]?3840x\d{4}[)\]]?").unwrap(),
        remove_match: true
    };

    //     parser.addHandler("resolution", /[(\[]?1920x\d{3,4}[)\]]?/i, value("1080p"), { remove: true });
    pub static ref RESOLUTION_1080P_1: RegexHandler = RegexHandler {
        regex: Regex::new(r"[(\[]?1920x\d{3,4}[)\]]?").unwrap(),
        remove_match: true
    };

    //     parser.addHandler("resolution", /[(\[]?1280x\d{3}[)\]]?/i, value("720p"), { remove: true });
    pub static ref RESOLUTION_720P_1: RegexHandler = RegexHandler {
        regex: Regex::new(r"[(\[]?1280x\d{3}[)\]]?").unwrap(),
        remove_match: true
    };

    //     parser.addHandler("resolution", /[(\[]?\d{3,4}x(\d{3,4})[)\]]?/i, value("$1p"), { remove: true });
    pub static ref RESOLUTION_1: RegexHandler = RegexHandler {
        regex: Regex::new(r"[(\[]?\d{3,4}x(\d{3,4})[)\]]?").unwrap(),
        remove_match: true
    };

    //     parser.addHandler("resolution", /(480|720|1080)0[pi]/i, value("$1p"), { remove: true });
    pub static ref RESOLUTION_2: RegexHandler = RegexHandler {
        regex: Regex::new(r"(480|720|1080)0[pi]").unwrap(),
        remove_match: true
    };

    //     parser.addHandler("resolution", /(?:BD|HD|M)(720|1080|2160)/, value("$1p"), { remove: true });
    pub static ref RESOLUTION_3: RegexHandler = RegexHandler {
        regex: Regex::new(r"(?:BD|HD|M)(720|1080|2160)").unwrap(),
        remove_match: true
    };

    //     parser.addHandler("resolution", /(480|576|720|1080|2160)[pi]/i, value("$1p"), { remove: true });
    pub static ref RESOLUTION_4: RegexHandler = RegexHandler {
        regex: Regex::new(r"(480|576|720|1080|2160)[pi]").unwrap(),
        remove_match: true
    };

    //     parser.addHandler("resolution", /(?:^|\D)(\d{3,4})[pi]/i, value("$1p"), { remove: true });
    pub static ref RESOLUTION_5: RegexHandler = RegexHandler {
        regex: Regex::new(r"(?:^|\D)(\d{3,4})[pi]").unwrap(),
        remove_match: true
    };

}

fn all_resolutions() -> Vec<&'static RegexHandler> {
    vec![
        &*RESOLUTION_4K_1,
        &*RESOLUTION_4K_2,
        &*RESOLUTION_4K_3,
        &*RESOLUTION_1080P_1,
        &*RESOLUTION_720P_1,
        &*RESOLUTION_1,
        &*RESOLUTION_2,
        &*RESOLUTION_3,
        &*RESOLUTION_4,
        &*RESOLUTION_5,
    ]
}

fn handle_resolution(name: &str) -> (String, Vec<String>) {
    let mut new_name = name.to_string();
    let mut matches = Vec::new();

    for resolution in all_resolutions() {
        let (new_name_, match_) = resolution.handle(&new_name);

        if !match_.is_empty() {
            new_name = new_name_;
            matches.push(match_.join(", "));
        }
    }

    (new_name, matches)
}

#[cfg(test)]
mod resolution_tests {
    use super::*;
    use paste::paste;

    #[test]
    fn test_evangelion_3_resolution() {
        let name = "Evangelion: 3.0+1.11 ｢Thrice Upon a Time｣ Movie (BDRip 1080p & UHDRip 2160p / 4K / x265 / HEVC / EAC3 / AC3) MKV";

        let (new_name, match_) = RESOLUTION_4K_1.handle(name);

        assert_eq!(new_name, "Evangelion: 3.0+1.11 ｢Thrice Upon a Time｣ Movie (BDRip 1080p & UHDRip 2160p /  / x265 / HEVC / EAC3 / AC3) MKV");
        assert_eq!(match_, Vec::<String>::new());

        let (new_name, match_) = RESOLUTION_4K_2.handle(&new_name);

        assert_eq!(new_name, "Evangelion: 3.0+1.11 ｢Thrice Upon a Time｣ Movie (BDRip 1080p & UHDRip  /  / x265 / HEVC / EAC3 / AC3) MKV");
        assert_eq!(match_, Vec::<String>::new());

        let (new_name, match_) = RESOLUTION_4K_3.handle(&new_name);

        assert_eq!(new_name, "Evangelion: 3.0+1.11 ｢Thrice Upon a Time｣ Movie (BDRip 1080p & UHDRip  /  / x265 / HEVC / EAC3 / AC3) MKV");
        assert_eq!(match_, Vec::<String>::new());

        let (new_name, match_) = RESOLUTION_1080P_1.handle(&new_name);

        assert_eq!(new_name, "Evangelion: 3.0+1.11 ｢Thrice Upon a Time｣ Movie (BDRip 1080p & UHDRip  /  / x265 / HEVC / EAC3 / AC3) MKV");
        assert_eq!(match_, Vec::<String>::new());

        let (new_name, match_) = RESOLUTION_720P_1.handle(&new_name);

        assert_eq!(new_name, "Evangelion: 3.0+1.11 ｢Thrice Upon a Time｣ Movie (BDRip 1080p & UHDRip  /  / x265 / HEVC / EAC3 / AC3) MKV");
        assert_eq!(match_, Vec::<String>::new());

        let (new_name, match_) = RESOLUTION_1.handle(&new_name);

        assert_eq!(new_name, "Evangelion: 3.0+1.11 ｢Thrice Upon a Time｣ Movie (BDRip 1080p & UHDRip  /  / x265 / HEVC / EAC3 / AC3) MKV");
        assert_eq!(match_, Vec::<String>::new());

        let (new_name, match_) = RESOLUTION_2.handle(&new_name);

        assert_eq!(new_name, "Evangelion: 3.0+1.11 ｢Thrice Upon a Time｣ Movie (BDRip 1080p & UHDRip  /  / x265 / HEVC / EAC3 / AC3) MKV");
        assert_eq!(match_, Vec::<String>::new());

        let (new_name, match_) = RESOLUTION_3.handle(&new_name);

        assert_eq!(new_name, "Evangelion: 3.0+1.11 ｢Thrice Upon a Time｣ Movie (BDRip 1080p & UHDRip  /  / x265 / HEVC / EAC3 / AC3) MKV");
        assert_eq!(match_, Vec::<String>::new());

        let (new_name, match_) = RESOLUTION_4.handle(&new_name);

        assert_eq!(new_name, "Evangelion: 3.0+1.11 ｢Thrice Upon a Time｣ Movie (BDRip  & UHDRip  /  / x265 / HEVC / EAC3 / AC3) MKV");
        assert_eq!(match_, vec!["1080".to_string()]);

        let (new_name, match_) = RESOLUTION_5.handle(&new_name);

        assert_eq!(new_name, "Evangelion: 3.0+1.11 ｢Thrice Upon a Time｣ Movie (BDRip  & UHDRip  /  / x265 / HEVC / EAC3 / AC3) MKV");
        assert_eq!(match_, Vec::<String>::new());
    }

    macro_rules! test_resolution {
        ($($name:ident: $value:expr,)*) => {
            $(
            paste! {
                    #[test]
                    fn [<test_ $name>]()
                    {
                        let (input, expected) = $value;

                        assert_eq!(expected, handle_resolution(input));
                    }
                }
            )*
        }
    }

    test_resolution! {
        bocchi_ohys: (
            "[Ohys-Raws] Bocchi the Rock! - 08 (BS11 1280x720 x264 AAC).mp4",
            (
                "[Ohys-Raws] Bocchi the Rock! - 08 (BS11  x264 AAC).mp4".to_owned(),
                vec!["720".to_string()]
            )
        ),
        bocchi_s3rnx: (
            "[S3rNx] Bocchi the Rock! Vol.3 (BDRip 1920x1080 x264 FLAC)",
            (
                "[S3rNx] Bocchi the Rock! Vol.3 (BDRip  x264 FLAC)".to_owned(),
                vec!["1080".to_string()]
            )
        ),
        bocchi_uccuss: (
            "[UCCUSS] Bocchi The Rock! ぼっち・ざ・ろっく! 第2巻 (BD 1920x1080p AVC FLAC)",
            (
                "[UCCUSS] Bocchi The Rock! ぼっち・ざ・ろっく! 第2巻 (BD  AVC FLAC)".to_owned(),
                vec!["1080".to_string()]
            )
        ),
        tensura_nores: (
            "Tensei shitara Slime Datta Ken ep.25-26 2021-01-12 on air.mp4",
            (
                "Tensei shitara Slime Datta Ken ep.25-26 2021-01-12 on air.mp4".to_owned(),
                Vec::<String>::new()
            )
        ),
        tensura_ohys: (
            "[Ohys-Raws] Tensei Shitara Slime Datta Ken (2019) - OAD3 (DVD 1024x576 x264 AAC).mp4",
            (
                "[Ohys-Raws] Tensei Shitara Slime Datta Ken (2019) - OAD3 (DVD  x264 AAC).mp4".to_owned(),
                vec!["576".to_string()]
            )
        ),
    }
}
