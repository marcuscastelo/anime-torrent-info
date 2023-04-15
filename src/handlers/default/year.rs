use super::prelude::*;

lazy_static! {
//     parser.addHandler("date", /(?<=\W|^)([(\[]?(?:19[6-9]|20[012])[0-9]([. \-/\\])(?:0[1-9]|1[012])\2(?:0[1-9]|[12][0-9]|3[01])[)\]]?)(?=\W|$)/, date("YYYY MM DD"), { remove: true });
pub static ref DATE_1: RegexHandler = RegexHandler {
    regex: Regex::new(r"\b([(\[]?(?:19[6-9]|20[012])[0-9]([. \-/\\])(?:0[1-9]|1[012])([. \-/\\])(?:0[1-9]|[12][0-9]|3[01])[)\]]?)\b").unwrap(),
    remove_match: true
};

//     parser.addHandler("date", /(?<=\W|^)([(\[]?(?:0[1-9]|[12][0-9]|3[01])([. \-/\\])(?:0[1-9]|1[012])\2(?:19[6-9]|20[012])[0-9][)\]]?)(?=\W|$)/, date("DD MM YYYY"), { remove: true });
pub static ref DATE_2: RegexHandler = RegexHandler {
    regex: Regex::new(r"\b([(\[]?(?:0[1-9]|[12][0-9]|3[01])([. \-/\\])(?:0[1-9]|1[012])([. \-/\\])(?:19[6-9]|20[012])[0-9][)\]]?)\b").unwrap(),
    remove_match: true
};

//     parser.addHandler("date", /(?<=\W)([(\[]?(?:0[1-9]|1[012])([. \-/\\])(?:0[1-9]|[12][0-9]|3[01])\2(?:[0][1-9]|[0126789][0-9])[)\]]?)(?=\W|$)/, date("MM DD YY"), { remove: true });
pub static ref DATE_3: RegexHandler = RegexHandler {
    regex: Regex::new(r"\b([(\[]?(?:0[1-9]|1[012])([. \-/\\])(?:0[1-9]|[12][0-9]|3[01])([. \-/\\])(?:[0][1-9]|[0126789][0-9])[)\]]?)\b").unwrap(),
    remove_match: true
};

//     parser.addHandler("date", /(?<=\W)([(\[]?(?:0[1-9]|[12][0-9]|3[01])([. \-/\\])(?:0[1-9]|1[012])\2(?:[0][1-9]|[0126789][0-9])[)\]]?)(?=\W|$)/, date("DD MM YY"), { remove: true });
pub static ref DATE_4: RegexHandler = RegexHandler {
    regex: Regex::new(r"\b([(\[]?(?:0[1-9]|[12][0-9]|3[01])([. \-/\\])(?:0[1-9]|1[012])([. \-/\\])(?:[0][1-9]|[0126789][0-9])[)\]]?)\b").unwrap(),
    remove_match: true
};

//     parser.addHandler("date", /(?<=\W|^)([(\[]?(?:0?[1-9]|[12][0-9]|3[01])[. ]?(?:st|nd|rd|th)?([. \-/\\])(?:feb(?:ruary)?|jan(?:uary)?|mar(?:ch)?|apr(?:il)?|may|june?|july?|aug(?:ust)?|sept?(?:ember)?|oct(?:ober)?|nov(?:ember)?|dec(?:ember)?)\2(?:19[7-9]|20[012])[0-9][)\]]?)(?=\W|$)/i, date("DD MMM YYYY"), { remove: true });
pub static ref DATE_5: RegexHandler = RegexHandler {
    regex: Regex::new(r"(?i)\b([(\[]?(?:0?[1-9]|[12][0-9]|3[01])[. ]?(?:st|nd|rd|th)?([. \-/\\])(?:feb(?:ruary)?|jan(?:uary)?|mar(?:ch)?|apr(?:il)?|may|june?|july?|aug(?:ust)?|sept?(?:ember)?|oct(?:ober)?|nov(?:ember)?|dec(?:ember)?)([. \-/\\])(?:19[7-9]|20[012])[0-9][)\]]?)\b").unwrap(),
    remove_match: true
};

//     parser.addHandler("date", /(?<=\W|^)([(\[]?(?:0?[1-9]|[12][0-9]|3[01])[. ]?(?:st|nd|rd|th)?([. \-/\\])(?:feb(?:ruary)?|jan(?:uary)?|mar(?:ch)?|apr(?:il)?|may|june?|july?|aug(?:ust)?|sept?(?:ember)?|oct(?:ober)?|nov(?:ember)?|dec(?:ember)?)\2(?:0[1-9]|[0126789][0-9])[)\]]?)(?=\W|$)/i, date("DD MMM YY"), { remove: true });
pub static ref DATE_6: RegexHandler = RegexHandler {
    regex: Regex::new(r"(?i)\b([(\[]?(?:0?[1-9]|[12][0-9]|3[01])[. ]?(?:st|nd|rd|th)?([. \-/\\])(?:feb(?:ruary)?|jan(?:uary)?|mar(?:ch)?|apr(?:il)?|may|june?|july?|aug(?:ust)?|sept?(?:ember)?|oct(?:ober)?|nov(?:ember)?|dec(?:ember)?)([. \-/\\])(?:0[1-9]|[0126789][0-9])[)\]]?)\b").unwrap(),
    remove_match: true
};

//     parser.addHandler("date", /(?<=\W|^)([(\[]?20[012][0-9](?:0[1-9]|1[012])(?:0[1-9]|[12][0-9]|3[01])[)\]]?)(?=\W|$)/, date("YYYYMMDD"), { remove: true });
pub static ref DATE_7: RegexHandler = RegexHandler {
    regex: Regex::new(r"\b([(\[]?20[012][0-9](?:0[1-9]|1[012])(?:0[1-9]|[12][0-9]|3[01])[)\]]?)\b").unwrap(),
    remove_match: true
};

}

fn all_dates() -> Vec<&'static RegexHandler> {
    vec![
        &DATE_1, &DATE_2, &DATE_3, &DATE_4, &DATE_5, &DATE_6, &DATE_7,
    ]
}

fn handle_date(filename: &str) -> (String, Vec<String>) {
    let mut new_name = filename.to_string();
    let mut matches = Vec::new();

    for date in all_dates() {
        let (new_name_, match_) = date.handle(&new_name);
        new_name = new_name_;
        matches.extend(match_);
    }

    (new_name, matches)
}

lazy_static! {
//     parser.addHandler("year", /[(\[]?[ .]?((?:19\d|20[012])\d[ .]?-[ .]?(?:19\d|20[012])\d)[ .]?[)\]]?/, yearRange, { remove: true });
pub static ref YEAR_1: RegexHandler = RegexHandler {
    regex: Regex::new(r"[(\[]?[ .]?((?:19\d|20[012])\d[ .]?-[ .]?(?:19\d|20[012])\d)[ .]?[)\]]?").unwrap(),
    remove_match: true
};

//     parser.addHandler("year", /[(\[][ .]?((?:19\d|20[012])\d[ .]?-[ .]?\d{2})[ .]?[)\]]/, yearRange, { remove: true });
pub static ref YEAR_2: RegexHandler = RegexHandler {
    regex: Regex::new(r"[(\[][ .]?((?:19\d|20[012])\d[ .]?-[ .]?\d{2})[ .]?[)\]]").unwrap(),
    remove_match: true
};

//TODO: alternative regex for lookaround since it's not supported in rust
//     parser.addHandler("year", /[(\[]?(?!^)(?<!\d|Cap[. ]?)((?:19\d|20[012])\d)(?!\d|kbps)[)\]]?/i, integer, { remove: true });
pub static ref YEAR_3: RegexHandler = RegexHandler {
    regex: Regex::new(r"NEVERGOINGTOMATCHANYTHINGSINCEREGEXISBIG").unwrap(),
    // regex: Regex::new(r"(?i)[(\[]?(?!^)(?<!\d|Cap[. ]?)((?:19\d|20[012])\d)(?!\d|kbps)[)\]]?").unwrap(),
    remove_match: true
};

//     parser.addHandler("year", /^[(\[]?((?:19\d|20[012])\d)(?!\d|kbps)[)\]]?/i, integer, { remove: true });
pub static ref YEAR_4: RegexHandler = RegexHandler {
    regex: Regex::new(r"(?i)^[(\[]?((?:19\d|20[012])\d)(?!\d|kbps)[)\]]?").unwrap(),
    remove_match: true
};

}

fn all_years() -> Vec<&'static RegexHandler> {
    vec![&YEAR_1, &YEAR_2, &YEAR_3, &YEAR_4]
}

fn handle_year(filename: &str) -> (String, Vec<String>) {
    let mut new_name = filename.to_string();
    let mut matches = Vec::new();

    for year in all_years() {
        let (new_name_, match_) = year.handle(&new_name);
        new_name = new_name_;
        matches.extend(match_);
    }

    (new_name, matches)
}

#[cfg(test)]
mod date_tests {
    use super::*;

    #[test]
    fn test_date_1() {
        let (new_name, match_) =
            DATE_1.handle("Tensei shitara Slime Datta Ken ep.25-26 2021-01-12 on air.mp4");

        assert_eq!(
            new_name,
            "Tensei shitara Slime Datta Ken ep.25-26  on air.mp4"
        );
        assert_eq!(match_, vec!["2021-01-12", "-", "-"]);
    }

    #[test]
    fn test_handle_date() {
        let (new_name, match_) =
            handle_date("Tensei shitara Slime Datta Ken ep.25-26 2021-01-12 on air.mp4");

        assert_eq!(
            new_name,
            "Tensei shitara Slime Datta Ken ep.25-26  on air.mp4"
        );
        assert_eq!(match_, vec!["2021-01-12", "-", "-"]);
    }
}

#[cfg(test)]
mod year_tests {
    use super::*;

    //TODO: test
    // #[test]
    // fn test_year_1() {
    //     let (new_name, match_) = handle_year("[X5-452] Hunter × Hunter (2011) - S1-S3 [Webrip][1080p][x265][Opus][Vostfr]");

    //     assert_eq!(
    //         new_name,
    //         "[X5-452] Hunter × Hunter (2011) - S1-S3 [Webrip][1080p][x265][Opus][Vostfr]"
    //     );
    //     assert_eq!(match_, vec!["2011", "-", "-"]);
    // }
    
}