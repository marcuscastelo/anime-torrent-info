use super::prelude::*;

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