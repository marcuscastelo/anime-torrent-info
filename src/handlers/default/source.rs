use super::prelude::*;

lazy_static! {

//     parser.addHandler("source", /\b(?:H[DQ][ .-]*)?CAM(?:H[DQ])?(?:[ .-]*Rip)?\b/i, value("CAM"), { remove: true });
static ref SOURCE_1 : RegexHandler = RegexHandler {
    regex: Regex::new(r"(?i)\b(?:H[DQ][ .-]*)?CAM(?:H[DQ])?(?:[ .-]*Rip)?\b").unwrap(),
    remove_match: true
};

//     parser.addHandler("source", /\b(?:H[DQ][ .-]*)?S[ .-]*print/i, value("CAM"), { remove: true });
static ref SOURCE_2 : RegexHandler = RegexHandler {
    regex: Regex::new(r"(?i)\b(?:H[DQ][ .-]*)?S[ .-]*print").unwrap(),
    remove_match: true
};

//     parser.addHandler("source", /\b(?:HD[ .-]*)?T(?:ELE)?S(?:YNC)?(?:Rip)?\b/i, value("TeleSync"), { remove: true });
static ref SOURCE_3 : RegexHandler = RegexHandler {
    regex: Regex::new(r"(?i)\b(?:HD[ .-]*)?T(?:ELE)?S(?:YNC)?(?:Rip)?\b").unwrap(),
    remove_match: true
};

//     parser.addHandler("source", /\b(?:HD[ .-]*)?T(?:ELE)?C(?:INE)?(?:Rip)?\b/, value("TeleCine"), { remove: true });
static ref SOURCE_4 : RegexHandler = RegexHandler {
    regex: Regex::new(r"(?i)\b(?:HD[ .-]*)?T(?:ELE)?C(?:INE)?(?:Rip)?\b").unwrap(),
    remove_match: true
};

//     parser.addHandler("source", /\bBlu[ .-]*Ray\b(?=.*remux)/i, value("BluRay REMUX"), { remove: true });
static ref SOURCE_5 : RegexHandler = RegexHandler {
    regex: Regex::new(r"(?i)\bBlu[ .-]*Ray\b(?=.*remux)").unwrap(),
    remove_match: true
};

//     parser.addHandler("source", /(?:BD|BR|UHD)[- ]?remux/i, value("BluRay REMUX"), { remove: true });
static ref SOURCE_6 : RegexHandler = RegexHandler {
    regex: Regex::new(r"(?i)(?:BD|BR|UHD)[- ]?remux").unwrap(),
    remove_match: true
};

//     parser.addHandler("source", /(?<=remux.*)\bBlu[ .-]*Ray\b/i, value("BluRay REMUX"), { remove: true });
static ref SOURCE_7 : RegexHandler = RegexHandler {
    regex: Regex::new(r"(?i)remux.*\bBlu[ .-]*Ray\b").unwrap(),
    remove_match: true
};

//     parser.addHandler("source", /\bBlu[ .-]*Ray\b(?![ .-]*Rip)/i, value("BluRay"), { remove: true });
static ref SOURCE_8 : RegexHandler = RegexHandler {
    regex: Regex::new(r"(?i)\bBlu[ .-]*Ray\b(?![ .-]*Rip)").unwrap(),
    remove_match: true
};

//     parser.addHandler("source", /\bUHD[ .-]*Rip\b/i, value("UHDRip"), { remove: true });
static ref SOURCE_9 : RegexHandler = RegexHandler {
    regex: Regex::new(r"(?i)\bUHD[ .-]*Rip\b").unwrap(),
    remove_match: true
};

//     parser.addHandler("source", /\bHD[ .-]*Rip\b/i, value("HDRip"), { remove: true });
static ref SOURCE_10 : RegexHandler = RegexHandler {
    regex: Regex::new(r"(?i)\bHD[ .-]*Rip\b").unwrap(),
    remove_match: true
};

//     parser.addHandler("source", /\bMicro[ .-]*HD\b/i, value("HDRip"), { remove: true });
static ref SOURCE_11 : RegexHandler = RegexHandler {
    regex: Regex::new(r"(?i)\bMicro[ .-]*HD\b").unwrap(),
    remove_match: true
};

//     parser.addHandler("source", /\b(?:BR|Blu[ .-]*Ray)[ .-]*Rip\b/i, value("BRRip"), { remove: true });
static ref SOURCE_12 : RegexHandler = RegexHandler {
    regex: Regex::new(r"(?i)\b(?:BR|Blu[ .-]*Ray)[ .-]*Rip\b").unwrap(),
    remove_match: true
};

//     parser.addHandler("source", /\bBD[ .-]*Rip\b|\bBDR\b|\bBD-RM\b|[\[(]BD[\]) .,-]/i, value("BDRip"), { remove: true });
static ref SOURCE_13 : RegexHandler = RegexHandler {
    regex: Regex::new(r"(?i)\bBD[ .-]*Rip\b|\bBDR\b|\bBD-RM\b|[\[(]BD[\]) .,-]").unwrap(),
    remove_match: true
};

//     parser.addHandler("source", /\b(?:HD[ .-]*)?DVD[ .-]*Rip\b/i, value("DVDRip"), { remove: true });
static ref SOURCE_14 : RegexHandler = RegexHandler {
    regex: Regex::new(r"(?i)\b(?:HD[ .-]*)?DVD[ .-]*Rip\b").unwrap(),
    remove_match: true
};

//     parser.addHandler("source", /\bVHS[ .-]*Rip\b/i, value("DVDRip"), { remove: true });
static ref SOURCE_15 : RegexHandler = RegexHandler {
    regex: Regex::new(r"(?i)\bVHS[ .-]*Rip\b").unwrap(),
    remove_match: true
};

//     parser.addHandler("source", /\b(?:DVD?|BD|BR)?[ .-]*Scr(?:eener)?\b/i, value("SCR"), { remove: true });
    static ref SOURCE_16 : RegexHandler = RegexHandler {
        regex: Regex::new(r"(?i)\b(?:DVD?|BD|BR)?[ .-]*Scr(?:eener)?\b").unwrap(),
        remove_match: true
    };

//     parser.addHandler("source", /\bP(?:re)?DVD(?:Rip)?\b/i, value("SCR"), { remove: true });
    static ref SOURCE_17 : RegexHandler = RegexHandler {
        regex: Regex::new(r"(?i)\bP(?:re)?DVD(?:Rip)?\b").unwrap(),
        remove_match: true
    };

//     parser.addHandler("source", /\bDVD(?:R\d?)?\b/i, value("DVD"), { remove: true });
    static ref SOURCE_18 : RegexHandler = RegexHandler {
        regex: Regex::new(r"(?i)\bDVD(?:R\d?)?\b").unwrap(),
        remove_match: true
    };

//     parser.addHandler("source", /\bVHS\b/i, value("DVD"), { remove: true });
    static ref SOURCE_19 : RegexHandler = RegexHandler {
        regex: Regex::new(r"(?i)\bVHS\b").unwrap(),
        remove_match: true
    };

//     parser.addHandler("source", /\bPPVRip\b/i, value("PPVRip"), { remove: true });
    static ref SOURCE_20 : RegexHandler = RegexHandler {
        regex: Regex::new(r"(?i)\bPPVRip\b").unwrap(),
        remove_match: true
    };

//     parser.addHandler("source", /\bHD[ .-]*TV(?:Rip)?\b/i, value("HDTV"), { remove: true });
    static ref SOURCE_21 : RegexHandler = RegexHandler {
        regex: Regex::new(r"(?i)\bHD[ .-]*TV(?:Rip)?\b").unwrap(),
        remove_match: true
    };

//     parser.addHandler("source", /\bDVB[ .-]*(?:Rip)?\b/i, value("HDTV"), { remove: true });
    static ref SOURCE_22 : RegexHandler = RegexHandler {
        regex: Regex::new(r"(?i)\bDVB[ .-]*(?:Rip)?\b").unwrap(),
        remove_match: true
    };

//     parser.addHandler("source", /\bSAT[ .-]*Rips?\b/i, value("SATRip"), { remove: true });
    static ref SOURCE_23 : RegexHandler = RegexHandler {
        regex: Regex::new(r"(?i)\bSAT[ .-]*Rips?\b").unwrap(),
        remove_match: true
    };

//     parser.addHandler("source", /\bTVRips?\b/i, value("TVRip"), { remove: true });
    static ref SOURCE_24 : RegexHandler = RegexHandler {
        regex: Regex::new(r"(?i)\bTVRips?\b").unwrap(),
        remove_match: true
    };

//     parser.addHandler("source", /\bR5\b/i, value("R5"), { remove: true });
    static ref SOURCE_25 : RegexHandler = RegexHandler {
        regex: Regex::new(r"(?i)\bR5\b").unwrap(),
        remove_match: true
    };

//     parser.addHandler("source", /\bWEB[ .-]*DL(?:Rip)?\b/i, value("WEB-DL"), { remove: true });
    static ref SOURCE_26 : RegexHandler = RegexHandler {
        regex: Regex::new(r"(?i)\bWEB[ .-]*DL(?:Rip)?\b").unwrap(),
        remove_match: true
    };

//     parser.addHandler("source", /\bWEB[ .-]*Rip\b/i, value("WEBRip"), { remove: true });
    static ref SOURCE_27 : RegexHandler = RegexHandler {
        regex: Regex::new(r"(?i)\bWEB[ .-]*Rip\b").unwrap(),
        remove_match: true
    };

//     parser.addHandler("source", /\b(?:DL|WEB|BD|BR)MUX\b/i, { remove: true });
    static ref SOURCE_28 : RegexHandler = RegexHandler {
        regex: Regex::new(r"(?i)\b(?:DL|WEB|BD|BR)MUX\b").unwrap(),
        remove_match: true
    };

//     parser.addHandler("source", /\b(DivX|XviD)\b/, { remove: true });
    static ref SOURCE_29 : RegexHandler = RegexHandler {
        regex: Regex::new(r"(?i)\b(DivX|XviD)\b").unwrap(),
        remove_match: true
    };
}

pub fn all_sources() -> Vec<&'static RegexHandler> {
    vec![
        &SOURCE_1, &SOURCE_2, &SOURCE_3, &SOURCE_4, &SOURCE_5, &SOURCE_6, &SOURCE_7, &SOURCE_8,
        &SOURCE_9, &SOURCE_10, &SOURCE_11, &SOURCE_12, &SOURCE_13, &SOURCE_14, &SOURCE_15,
        &SOURCE_16, &SOURCE_17, &SOURCE_18, &SOURCE_19, &SOURCE_20, &SOURCE_21, &SOURCE_22,
        &SOURCE_23, &SOURCE_24, &SOURCE_25, &SOURCE_26, &SOURCE_27, &SOURCE_28, &SOURCE_29,
    ]
}

pub fn handle_source(name: &str) -> (String, Vec<String>) {
    let mut new_name = name.to_string();
    let mut matches = Vec::new();
    for source in all_sources() {
        let (new_name_, matches_) = source.handle(&new_name);
        new_name = new_name_;
        matches.extend(matches_);
    }
    (new_name, matches)
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::handlers::default::prelude::Handler;

    // ----- SOURCE 1 -----

    #[test]
    fn test_source_1_match() {
        let name =
            "Suzume no Tojimari (2022) (1080p CAM)[Official English Subs] | すずめの戸締まり";
        let (new_name, matches) = super::SOURCE_1.handle(name);
        assert_eq!(
            new_name,
            "Suzume no Tojimari (2022) (1080p )[Official English Subs] | すずめの戸締まり"
        );
        assert_eq!(matches, Vec::<String>::new());
    }

    #[test]
    fn test_source_1_no_match() {
        let name = "Suzume no Tojimari (2022) (1080p)[Official English Subs] | すずめの戸締まり";
        let (new_name, matches) = super::SOURCE_1.handle(name);
        assert_eq!(new_name, name);
        assert_eq!(matches, Vec::<String>::new());
    }

    // ----- SOURCE 2 -----

    #[test]
    fn test_source_2_match() {
        let name = "[Kaerizaki-Fansub]One_Piece_1057[VOSTFR][HD.Sprint_1280x720].mp4";
        let (new_name, matches) = super::SOURCE_2.handle(name);
        assert_eq!(
            new_name,
            "[Kaerizaki-Fansub]One_Piece_1057[VOSTFR][_1280x720].mp4"
        );
        assert_eq!(matches, Vec::<String>::new());
    }

    #[test]
    fn test_source_2_no_match() {
        let name = "[Kaerizaki-Fansub]One_Piece_1057[VOSTFR][HD_1280x720].mp4";
        let (new_name, matches) = super::SOURCE_2.handle(name);
        assert_eq!(new_name, name);
        assert_eq!(matches, Vec::<String>::new());
    }

    // ----- SOURCE 3 -----

    #[test]
    fn test_source_3_match() {
        let name = "Suzume no Tojimari (2022) (1080p TS)[Official English Subs] | すずめの戸締まり";
        let (new_name, matches) = super::SOURCE_3.handle(name);
        assert_eq!(
            new_name,
            "Suzume no Tojimari (2022) (1080p )[Official English Subs] | すずめの戸締まり"
        );
        assert_eq!(matches, Vec::<String>::new());
    }

    #[test]
    fn test_source_3_no_match() {
        let name = "Suzume no Tojimari (2022) (1080p)[Official English Subs] | すずめの戸締まり";
        let (new_name, matches) = super::SOURCE_3.handle(name);
        assert_eq!(new_name, name);
        assert_eq!(matches, Vec::<String>::new());
    }

    //TODO: more tests for source 3

    // ----- SOURCE 4 -----

    #[test]
    fn test_source_4_match() {
        let name = "Suzume no Tojimari (2022) (1080p TC)[Official English Subs] | すずめの戸締まり";
        let (new_name, matches) = super::SOURCE_4.handle(name);
        assert_eq!(
            new_name,
            "Suzume no Tojimari (2022) (1080p )[Official English Subs] | すずめの戸締まり"
        );
        assert_eq!(matches, Vec::<String>::new());
    }

    #[test]
    fn test_source_4_no_match() {
        let name = "Suzume no Tojimari (2022) (1080p)[Official English Subs] | すずめの戸締まり";
        let (new_name, matches) = super::SOURCE_4.handle(name);
        assert_eq!(new_name, name);
        assert_eq!(matches, Vec::<String>::new());
    }

    //TODO: more tests for source 4

    // ----- SOURCE 5 -----

    #[test]
    fn test_source_5_match() {
        let name = "The Dark Knight (2008) [Blu-Ray 1080p x264] [Remux]";
        let (new_name, matches) = super::SOURCE_5.handle(name);
        assert_eq!(new_name, "The Dark Knight (2008) [ 1080p x264] [Remux]");
        assert_eq!(matches, Vec::<String>::new());
    }

    #[test]
    fn test_source_5_no_match() {
        let name = "The Dark Knight (2008) [1080p x264] [Remux]";
        let (new_name, matches) = super::SOURCE_5.handle(name);
        assert_eq!(new_name, name);
        assert_eq!(matches, Vec::<String>::new());
    }

    // ----- SOURCE 6 -----

    #[test]
    fn test_source_6_match() {
        let name = "Interstellar.2014.UHD Remux";
        let (new_name, matches) = super::SOURCE_6.handle(name);
        assert_eq!(new_name, "Interstellar.2014.");
        assert_eq!(matches, Vec::<String>::new());
    }

    #[test]
    fn test_source_6_no_match() {
        let name = "Interstellar.2014.";
        let (new_name, matches) = super::SOURCE_6.handle(name);
        assert_eq!(new_name, name);
        assert_eq!(matches, Vec::<String>::new());
    }

    // ----- SOURCE 7 -----

    #[test]
    fn test_source_7_match() {
        let name = "Interstellar.2014.UHD Remux [Blu-Ray 1080p x264]";
        let (new_name, matches) = super::SOURCE_7.handle(name);
        assert_eq!(new_name, "Interstellar.2014.UHD  1080p x264]");
        assert_eq!(matches, Vec::<String>::new());
    }

    #[test]
    fn test_source_7_no_match() {
        let name = "Interstellar.2014.UHD  1080p x264]";
        let (new_name, matches) = super::SOURCE_7.handle(name);
        assert_eq!(new_name, name);
        assert_eq!(matches, Vec::<String>::new());
    }

    // ----- SOURCE 8 -----

    #[test]
    fn test_source_8_match() {
        let name = "Blade Runner (1982) [Blu-Ray 4K]";
        let (new_name, matches) = super::SOURCE_8.handle(name);
        assert_eq!(new_name, "Blade Runner (1982) [ 4K]");
        assert_eq!(matches, Vec::<String>::new());
    }

    #[test]
    fn test_source_8_no_match() {
        let name = "Blade Runner (1982) [4K]";
        let (new_name, matches) = super::SOURCE_8.handle(name);
        assert_eq!(new_name, name);
        assert_eq!(matches, Vec::<String>::new());
    }

    // TODO: more tests for source 8

    // ----- SOURCE 9 -----

    #[test]
    fn test_source_9_match() {
        let name = "The Lord of the Rings Trilogy [UHD-Rip]";
        let (new_name, matches) = super::SOURCE_9.handle(name);
        assert_eq!(new_name, "The Lord of the Rings Trilogy []");
        assert_eq!(matches, Vec::<String>::new());
    }

    #[test]
    fn test_source_9_no_match() {
        let name = "The Lord of the Rings Trilogy []";
        let (new_name, matches) = super::SOURCE_9.handle(name);
        assert_eq!(new_name, name);
        assert_eq!(matches, Vec::<String>::new());
    }

    // ----- SOURCE 10 -----

    #[test]
    fn test_source_10_match() {
        let name = "The Lord of the Rings Trilogy [HD-Rip] [Blu-Ray 1080p x264]";
        let (new_name, matches) = super::SOURCE_10.handle(name);
        assert_eq!(
            new_name,
            "The Lord of the Rings Trilogy [] [Blu-Ray 1080p x264]"
        );
        assert_eq!(matches, Vec::<String>::new());
    }

    #[test]
    fn test_source_10_no_match() {
        let name = "The Lord of the Rings Trilogy [] [Blu-Ray 1080p x264]";
        let (new_name, matches) = super::SOURCE_10.handle(name);
        assert_eq!(new_name, name);
        assert_eq!(matches, Vec::<String>::new());
    }

    // ----- SOURCE 11 -----

    #[test]
    fn test_source_11_match() {
        let name = "Spider-Man: Far From Home (2019) [Micro HD]";
        let (new_name, matches) = super::SOURCE_11.handle(name);
        assert_eq!(new_name, "Spider-Man: Far From Home (2019) []");
        assert_eq!(matches, Vec::<String>::new());
    }

    #[test]
    fn test_source_11_no_match() {
        let name = "Spider-Man: Far From Home (2019) []";
        let (new_name, matches) = super::SOURCE_10.handle(name);
        assert_eq!(new_name, name);
        assert_eq!(matches, Vec::<String>::new());
    }

    // ----- SOURCE 12 -----

    #[test]
    fn test_source_12_match() {
        let name = "The Shawshank Redemption (1994) [BR-Rip]";
        let (new_name, matches) = super::SOURCE_12.handle(name);
        assert_eq!(new_name, "The Shawshank Redemption (1994) []");
        assert_eq!(matches, Vec::<String>::new());
    }

    #[test]
    fn test_source_12_no_match() {
        let name = "The Shawshank Redemption (1994) []";
        let (new_name, matches) = super::SOURCE_12.handle(name);
        assert_eq!(new_name, name);
        assert_eq!(matches, Vec::<String>::new());
    }

    // ----- SOURCE 13 -----

    #[test]
    fn test_source_13_match() {
        let name = "The Matrix (1999) [BD-Rip]";
        let (new_name, matches) = super::SOURCE_13.handle(name);
        assert_eq!(new_name, "The Matrix (1999) Rip]");
        assert_eq!(matches, Vec::<String>::new());
    }

    #[test]
    fn test_source_13_no_match() {
        let name = "The Matrix (1999) Rip]";
        let (new_name, matches) = super::SOURCE_13.handle(name);
        assert_eq!(new_name, name);
        assert_eq!(matches, Vec::<String>::new());
    }

    // ----- SOURCE 14 -----

    #[test]
    fn test_source_14_match() {
        let name = "The Godfather (1972) HD-DVD Rip";
        let (new_name, matches) = super::SOURCE_14.handle(name);
        assert_eq!(new_name, "The Godfather (1972) ");
        assert_eq!(matches, Vec::<String>::new());
    }

    #[test]
    fn test_source_14_no_match() {
        let name = "The Godfather (1972) ";
        let (new_name, matches) = super::SOURCE_14.handle(name);
        assert_eq!(new_name, name);
        assert_eq!(matches, Vec::<String>::new());
    }

    // ----- SOURCE 15 -----

    #[test]
    fn test_source_15_match() {
        let name = "Star Wars: Episode IV - A New Hope (1977) VHS-Rip";
        let (new_name, matches) = super::SOURCE_15.handle(name);
        assert_eq!(new_name, "Star Wars: Episode IV - A New Hope (1977) ");
        assert_eq!(matches, Vec::<String>::new());
    }

    #[test]
    fn test_source_15_no_match() {
        let name = "Star Wars: Episode IV - A New Hope (1977) ";
        let (new_name, matches) = super::SOURCE_15.handle(name);
        assert_eq!(new_name, name);
        assert_eq!(matches, Vec::<String>::new());
    }

    // ----- SOURCE 16 -----

    #[test]
    fn test_source_16_match() {
        let name = "The Suicide Squad (2021) DVDScr";
        let (new_name, matches) = super::SOURCE_16.handle(name);
        assert_eq!(new_name, "The Suicide Squad (2021) ");
        assert_eq!(matches, Vec::<String>::new());
    }

    #[test]
    fn test_source_16_no_match() {
        let name = "The Suicide Squad (2021) ";
        let (new_name, matches) = super::SOURCE_16.handle(name);
        assert_eq!(new_name, name);
        assert_eq!(matches, Vec::<String>::new());
    }

    // ----- SOURCE 17 -----

    #[test]
    fn test_source_17_match() {
        let name = "Spider-Man: No Way Home (2021) PDVD";
        let (new_name, matches) = super::SOURCE_17.handle(name);
        assert_eq!(new_name, "Spider-Man: No Way Home (2021) ");
        assert_eq!(matches, Vec::<String>::new());
    }

    #[test]
    fn test_source_17_no_match() {
        let name = "The Suicide Squad (2021) ";
        let (new_name, matches) = super::SOURCE_17.handle(name);
        assert_eq!(new_name, name);
        assert_eq!(matches, Vec::<String>::new());
    }

    // ----- SOURCE 18 -----

    #[test]
    fn test_source_18_match() {
        let name = "The Lord of the Rings: The Fellowship of the Ring DVDR";
        let (new_name, matches) = super::SOURCE_18.handle(name);
        assert_eq!(
            new_name,
            "The Lord of the Rings: The Fellowship of the Ring "
        );
        assert_eq!(matches, Vec::<String>::new());
    }

    #[test]
    fn test_source_18_no_match() {
        let name = "The Suicide Squad (2021) ";
        let (new_name, matches) = super::SOURCE_18.handle(name);
        assert_eq!(new_name, name);
        assert_eq!(matches, Vec::<String>::new());
    }

    // ----- SOURCE 19 -----

    #[test]
    fn test_source_19_match() {
        let name = "The Lion King VHS";
        let (new_name, matches) = super::SOURCE_19.handle(name);
        assert_eq!(new_name, "The Lion King ");
        assert_eq!(matches, Vec::<String>::new());
    }

    #[test]
    fn test_source_19_no_match() {
        let name = "The Suicide Squad (2021) ";
        let (new_name, matches) = super::SOURCE_19.handle(name);
        assert_eq!(new_name, name);
        assert_eq!(matches, Vec::<String>::new());
    }

    // ----- SOURCE 20 -----

    #[test]
    fn test_source_20_match() {
        let name = "The Lion King PPVRip";
        let (new_name, matches) = super::SOURCE_20.handle(name);
        assert_eq!(new_name, "The Lion King ");
        assert_eq!(matches, Vec::<String>::new());
    }

    #[test]
    fn test_source_20_no_match() {
        let name = "The Suicide Squad (2021) ";
        let (new_name, matches) = super::SOURCE_20.handle(name);
        assert_eq!(new_name, name);
        assert_eq!(matches, Vec::<String>::new());
    }

    // ----- SOURCE 21 -----

    #[test]
    fn test_source_21_match() {
        let name = "Breaking Bad S01E01 720p HDTVrip";
        let (new_name, matches) = super::SOURCE_21.handle(name);
        assert_eq!(new_name, "Breaking Bad S01E01 720p ");
        assert_eq!(matches, Vec::<String>::new());
    }

    #[test]
    fn test_source_21_no_match() {
        let name = "The Suicide Squad (2021) ";
        let (new_name, matches) = super::SOURCE_21.handle(name);
        assert_eq!(new_name, name);
        assert_eq!(matches, Vec::<String>::new());
    }

    // ----- SOURCE 22 -----

    #[test]
    fn test_source_22_match() {
        let name = "BBC.The.Blitz.A.Tale.of.Two.Cities.1080i.HDTV.DVB.MPEG2.DD2.0-CtrlHD.ts";
        let (new_name, matches) = super::SOURCE_22.handle(name);
        assert_eq!(
            new_name,
            "BBC.The.Blitz.A.Tale.of.Two.Cities.1080i.HDTV.MPEG2.DD2.0-CtrlHD.ts"
        );
        assert_eq!(matches, Vec::<String>::new());
    }

    #[test]
    fn test_source_22_no_match() {
        let name = "The Suicide Squad (2021) ";
        let (new_name, matches) = super::SOURCE_22.handle(name);
        assert_eq!(new_name, name);
        assert_eq!(matches, Vec::<String>::new());
    }

    // ----- SOURCE 23 -----

    #[test]
    fn test_source_23_match() {
        let name = "[Torrent-Links] SAT-Rips of_the_80s-90s";
        let (new_name, matches) = super::SOURCE_23.handle(name);
        assert_eq!(new_name, "[Torrent-Links]  of_the_80s-90s");
        assert_eq!(matches, Vec::<String>::new());
    }

    #[test]
    fn test_source_23_no_match() {
        let name = "The Suicide Squad (2021) ";
        let (new_name, matches) = super::SOURCE_23.handle(name);
        assert_eq!(new_name, name);
        assert_eq!(matches, Vec::<String>::new());
    }

    // ----- SOURCE 24 -----

    #[test]
    fn test_source_24_match() {
        let name = "TVRip of the Week: Game of Thrones S01E01";
        let (new_name, matches) = super::SOURCE_24.handle(name);
        assert_eq!(new_name, " of the Week: Game of Thrones S01E01");
        assert_eq!(matches, Vec::<String>::new());
    }

    #[test]
    fn test_source_24_no_match() {
        let name = "The Suicide Squad (2021) ";
        let (new_name, matches) = super::SOURCE_24.handle(name);
        assert_eq!(new_name, name);
        assert_eq!(matches, Vec::<String>::new());
    }

    // ----- SOURCE 25 -----

    #[test]
    fn test_source_25_match() {
        let name = "Meu_Filme.R5.720p.mp4";
        let (new_name, matches) = super::SOURCE_25.handle(name);
        assert_eq!(new_name, "Meu_Filme..720p.mp4");
        assert_eq!(matches, Vec::<String>::new());
    }

    #[test]
    fn test_source_25_no_match() {
        let name = "The Suicide Squad (2021) ";
        let (new_name, matches) = super::SOURCE_25.handle(name);
        assert_eq!(new_name, name);
        assert_eq!(matches, Vec::<String>::new());
    }

    // ----- SOURCE 26 -----

    #[test]
    fn test_source_26_match() {
        let name = "The Matrix Reloaded WEB-DL 1080p";
        let (new_name, matches) = super::SOURCE_26.handle(name);
        assert_eq!(new_name, "The Matrix Reloaded  1080p");
        assert_eq!(matches, Vec::<String>::new());
    }

    #[test]
    fn test_source_26_no_match() {
        let name = "The Suicide Squad (2021) ";
        let (new_name, matches) = super::SOURCE_26.handle(name);
        assert_eq!(new_name, name);
        assert_eq!(matches, Vec::<String>::new());
    }

    // ----- SOURCE 27 -----

    #[test]
    fn test_source_27_match() {
        let name = "The.Big.Bang.Theory.S01E01.1080p.WEBRip.x264-RARBG";
        let (new_name, matches) = super::SOURCE_27.handle(name);
        assert_eq!(new_name, "The.Big.Bang.Theory.S01E01.1080p..x264-RARBG");
        assert_eq!(matches, Vec::<String>::new());
    }

    #[test]
    fn test_source_27_no_match() {
        let name = "The Suicide Squad (2021) ";
        let (new_name, matches) = super::SOURCE_27.handle(name);
        assert_eq!(new_name, name);
        assert_eq!(matches, Vec::<String>::new());
    }

    // ----- SOURCE 28 -----

    #[test]
    fn test_source_28_match() {
        let name = "The.Big.Bang.Theory.S01E01.Pilot.1080p.WEB-DLMUX.DD5.1.H.264.mkv";
        let (new_name, matches) = super::SOURCE_28.handle(name);
        assert_eq!(
            new_name,
            "The.Big.Bang.Theory.S01E01.Pilot.1080p.WEB-.DD5.1.H.264.mkv"
        );
        assert_eq!(matches, Vec::<String>::new());
    }

    #[test]
    fn test_source_28_no_match() {
        let name = "The Suicide Squad (2021) ";
        let (new_name, matches) = super::SOURCE_28.handle(name);
        assert_eq!(new_name, name);
        assert_eq!(matches, Vec::<String>::new());
    }

    // ----- SOURCE 29 -----

    #[test]
    fn test_source_29_match() {
        let name = "Codec: DivX";
        let (new_name, matches) = super::SOURCE_29.handle(name);
        assert_eq!(new_name, "Codec: ");
        assert_eq!(matches, vec!["DivX"]);
    }

    #[test]
    fn test_source_29_no_match() {
        let name = "Codec: aaa";
        let (new_name, matches) = super::SOURCE_29.handle(name);
        assert_eq!(new_name, name);
        assert_eq!(matches, Vec::<String>::new());
    }

    // Handle Source

    #[test]
    fn test_handle_source_1_match() {
        let name = "The.Big.Bang.Theory.S01E01.1080p.WEB-DL.DD5.1.H.264-MUX.mkv";
        let (new_name, matches) = super::handle_source(name);
        assert_eq!(
            new_name,
            "The.Big.Bang.Theory.S01E01.1080p..DD5.1.H.264-MUX.mkv"
        );
        assert_eq!(matches, Vec::<String>::new());
    }

    #[test]
    fn test_handle_source_2_match() {
        let name = "The.Big.Bang.Theory.S01E02.1080p.WEB-Rip.DD5.1.H.264-MUX.mkv";
        let (new_name, matches) = super::handle_source(name);
        assert_eq!(
            new_name,
            "The.Big.Bang.Theory.S01E02.1080p..DD5.1.H.264-MUX.mkv"
        );
        assert_eq!(matches, Vec::<String>::new());
    }

    #[test]
    fn test_handle_source_3_match() {
        let name = "The.Big.Bang.Theory.S01E03.1080p.BluRay.DD5.1.x264-MUX.mkv";
        let (new_name, matches) = super::handle_source(name);
        assert_eq!(
            new_name,
            "The.Big.Bang.Theory.S01E03.1080p..DD5.1.x264-MUX.mkv"
        );
        assert_eq!(matches, Vec::<String>::new());
    }

    #[test]
    fn test_handle_source_4_match() {
        let name = "The.Big.Bang.Theory.S01E04.1080p.HDTV.x264-MUX.mkv";
        let (new_name, matches) = super::handle_source(name);
        assert_eq!(new_name, "The.Big.Bang.Theory.S01E04.1080p..x264-MUX.mkv");
        assert_eq!(matches, Vec::<String>::new());
    }

    #[test]
    fn test_handle_source_5_match() {
        let name = "The.Big.Bang.Theory.S01E05.1080p.HD-TV.Rip.x264-MUX.mkv";
        let (new_name, matches) = super::handle_source(name);
        assert_eq!(
            new_name,
            "The.Big.Bang.Theory.S01E05.1080p..Rip.x264-MUX.mkv"
        );
        assert_eq!(matches, Vec::<String>::new());
    }

    #[test]
    fn test_handle_source_6_match() {
        let name = "The.Big.Bang.Theory.S01E06.1080p.DVB-Rip.x264-MUX.mkv";
        let (new_name, matches) = super::handle_source(name);
        assert_eq!(new_name, "The.Big.Bang.Theory.S01E06.1080p..x264-MUX.mkv");
        assert_eq!(matches, Vec::<String>::new());
    }

    #[test]
    fn test_handle_source_7_match() {
        let name = "The.Big.Bang.Theory.S01E07.1080p.VHS-Rip.x264-MUX.mkv";
        let (new_name, matches) = super::handle_source(name);
        assert_eq!(new_name, "The.Big.Bang.Theory.S01E07.1080p..x264-MUX.mkv");
        assert_eq!(matches, Vec::<String>::new());
    }

    #[test]
    fn test_handle_source_8_match() {
        let name = "The.Big.Bang.Theory.S01E08.1080p.UHD-Rip.x264-MUX.mkv";
        let (new_name, matches) = super::handle_source(name);
        assert_eq!(new_name, "The.Big.Bang.Theory.S01E08.1080p..x264-MUX.mkv");
        assert_eq!(matches, Vec::<String>::new());
    }
}
