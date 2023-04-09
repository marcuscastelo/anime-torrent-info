// const moment = require("moment");

pub trait Transformer<T, U> {
    fn transform(&self, input: T) -> U;
}

pub struct NoneTransformer;

impl <T: ToOwned> Transformer<T, T::Owned> for NoneTransformer {
    fn transform(&self, input: T) -> T::Owned {
        input.to_owned()
    }
}

pub struct ValueTransformer {
    value: String,
}

impl ValueTransformer {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_owned(),
        }
    }
}

impl Transformer<&str, String> for ValueTransformer {
    fn transform(&self, input: &str) -> String {
        self.value.replace("$1", input)
    }
}

pub struct IntegerTransformer;
impl Transformer<&str, i32> for IntegerTransformer {
    fn transform(&self, input: &str) -> i32 {
        // parse like JS parseInt
        input
            .replace(|c: char| !c.is_ascii_digit(), "")
            .trim_start_matches('0')
            .parse::<i32>()
            .unwrap()
    }
}

pub struct BooleanTransformer;
impl Transformer<&str, bool> for BooleanTransformer {
    fn transform(&self, _input: &str) -> bool {
        true //TODO: check if this is correct
    }
}

pub struct LowercaseTransformer;
impl Transformer<&str, String> for LowercaseTransformer {
    fn transform(&self, input: &str) -> String {
        input.to_lowercase()
    }
}

pub struct UppercaseTransformer;
impl Transformer<&str, String> for UppercaseTransformer {
    fn transform(&self, input: &str) -> String {
        input.to_uppercase()
    }
}

pub struct DateTransformer {
    date_format: String,
}

impl DateTransformer {
    pub fn new(date_format: &str) -> Self {
        Self {
            date_format: date_format.to_owned(),
        }
    }
}

impl Transformer<&str, String> for DateTransformer {
    fn transform(&self, input: &str) -> String {
        let sanitized = input.trim();
        let date = chrono::NaiveDate::parse_from_str(sanitized, self.date_format.as_str()).unwrap();
        date.format("%Y-%m-%d").to_string()
    }
}

pub struct RangeTransformer;
impl Transformer<&str, Vec<i32>> for RangeTransformer {
    fn transform(&self, input: &str) -> Vec<i32> {
        let mut array = input
            .replace(|c: char| !c.is_ascii_digit(), " ")
            .trim()
            .split(" ")
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        if array.len() == 2 && array[0] < array[1] {
            array = (array[0]..=array[1]).collect();
        }
        if !array
            .iter()
            .enumerate()
            .all(|(idx, number)| idx == array.len() - 1 || number + 1 == array[idx + 1])
        {
            array = vec![]; //TODO: Result/Option instead?
        }
        array
    }
}

pub struct YearRangeTransformer;

impl Transformer<&str, String> for YearRangeTransformer {
    fn transform(&self, input: &str) -> String {
        let parts = input
            .split(|c: char| !c.is_ascii_digit())
            .collect::<Vec<&str>>();
        let start = match parts.get(0) {
            Some(value) => value.parse::<i32>().unwrap(),
            None => input.parse::<i32>().unwrap(),
        };
    
        let mut end = match parts.get(1) {
            Some(value) => value.parse::<i32>().unwrap(),
            None => start,
        };

        if end < 100 {
            end = end + start - start % 100;
        }
        if end <= start {
            "".to_owned() //TODO: Result/Option instead?
        } else {
            format!("{}-{}", start, end)
        }
    }
}

pub struct ArrayTransformer<T, U> {
    chain: Box<dyn Transformer<T, U>>,
}

impl <T, U> ArrayTransformer<T, U> {
    pub fn new(chain: Box<dyn Transformer<T, U>>) -> Self {
        Self { chain }
    }
}

impl <T, U: ToOwned> Transformer<T, Vec<U::Owned>> for ArrayTransformer<T, U> {
    fn transform(&self, input: T) -> Vec<U::Owned> {
        let value = self.chain.transform(input);
        vec![value.to_owned()]
    }
}

pub struct UniqConcatTransformer<T, U> {
    pub chain: Box<dyn Transformer<T, U>>,
}

impl <T, U> UniqConcatTransformer<T, U> {
    pub fn new(chain: Box<dyn Transformer<T, U>>) -> Self {
        Self { chain }
    }
}

impl<'a> Transformer<(&'static str, &'a mut Vec<String>), &'a mut Vec<String>> for UniqConcatTransformer<&str, String> {
    fn transform(&self, fields: (&'static str, &'a mut Vec<String>)) -> &'a mut Vec<String> {
        let (input, result) = fields;

        let value = self.chain.transform(input);
        if !result.contains(&value) {
            result.push(value);
        }
        result
    }
}

// exports.none = input => input;

// exports.value = value => input => {
//     if (typeof value === "string") {
//         return value.replace("$1", input);
//     }
//     return value;
// };

// exports.integer = input => parseInt(input, 10);

// exports.boolean = () => true;

// exports.lowercase = input => input.toLowerCase();

// exports.uppercase = input => input.toUpperCase();

// exports.date = dateFormat => input => {
//     const sanitized = input.replace(/\W+/g, " ").trim();
//     const date = moment(sanitized, dateFormat);

//     return date.format("YYYY-MM-DD");
// };

// exports.range = input => {
//     let array = input
//         .replace(/\D+/g, " ")
//         .trim()
//         .split(" ")
//         .map(str => parseInt(str, 10));

//     if (array.length === 2 && array[0] < array[1]) {
//         array = Array(array[1] - array[0] + 1).fill().map((_, idx) => array[0] + idx);
//     }
//     if (!array.every((number, idx) => idx === array.length - 1 || number + 1 === array[idx + 1])) {
//         return null; // array is not in sequence and ascending order
//     }
//     return array;
// };

// exports.yearRange = input => {
//     const parts = input.split(/\D+/);
//     const start = parts[0] && parseInt(parts[0], 10);
//     let end = parts[1] && parseInt(parts[1], 10);

//     if (!end) {
//         return start;
//     }
//     if (end < 100) {
//         end = end + start - start % 100;
//     }
//     if (end <= start) {
//         return null; // not a year range, try another year handler
//     }
//     return `${start}-${end}`;
// };

// exports.array = chain => input => [chain ? chain(input) : input];

// exports.uniqConcat = chain => (input, result) => {
//     const newResult = result || [];
//     const value = chain(input);

//     return newResult.includes(value) ? newResult : newResult.concat(value);
// };

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_none() {
        let transformer = NoneTransformer;
        assert_eq!(transformer.transform("foo"), "foo");
    }

    #[test]
    fn test_value() {
        let transformer = ValueTransformer::new("foo");
        assert_eq!(transformer.transform("bar"), "foo");
        assert_eq!(transformer.transform("baz"), "foo");

        let transformer = ValueTransformer::new("foo$1");
        assert_eq!(transformer.transform("bar"), "foobar");
        assert_eq!(transformer.transform("baz"), "foobaz");
    }

    #[test]
    fn test_integer() {
        let transformer = IntegerTransformer;
        assert_eq!(transformer.transform("1234"), 1234);
        assert_eq!(transformer.transform("1234foo"), 1234);
        assert_eq!(transformer.transform("foo1234"), 1234);
        assert_eq!(transformer.transform("foo1234foo"), 1234);
    }

    #[test]
    fn test_boolean() {
        let transformer = BooleanTransformer;
        assert_eq!(transformer.transform("foo"), true);
        assert_eq!(transformer.transform("bar"), true);
        assert_eq!(transformer.transform("baz"), true);
    }

    #[test]
    fn test_lowercase() {
        let transformer = LowercaseTransformer;
        assert_eq!(transformer.transform("FOO"), "foo");
        assert_eq!(transformer.transform("Bar"), "bar");
        assert_eq!(transformer.transform("BAZ"), "baz");
    }

    #[test]
    fn test_uppercase() {
        let transformer = UppercaseTransformer;
        assert_eq!(transformer.transform("foo"), "FOO");
        assert_eq!(transformer.transform("Bar"), "BAR");
        assert_eq!(transformer.transform("baz"), "BAZ");
    }

    #[test]
    fn test_date() {
        let transformer = DateTransformer {
            date_format: "%Y-%m-%d".to_owned(),
        };
        assert_eq!(transformer.transform("2020-01-01"), "2020-01-01");

        let transformer = DateTransformer {
            date_format: "%Y-%m-%d %H:%M:%S".to_owned(),
        };
        assert_eq!(transformer.transform("2020-01-01 00:00:00"), "2020-01-01");
    }

    #[test]
    fn test_range() {
        let transformer = RangeTransformer;
        assert_eq!(
            transformer.transform("1-10"),
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        );
        assert_eq!(transformer.transform("1,2,3,4,5"), vec![1, 2, 3, 4, 5]);
        assert_eq!(transformer.transform("1"), vec![1]);
    }

    #[test]
    fn test_year() {
        let transformer = YearRangeTransformer;
        assert_eq!(transformer.transform("2020"), "");
        assert_eq!(transformer.transform("2020-2021"), "2020-2021");
        assert_eq!(
            transformer.transform("2020-2021, 2022-2023"),
            "2020-2021"
        );
    }

    #[test]
    fn test_array() {
        let transformer: ArrayTransformer<&str, &str> = ArrayTransformer::new(Box::from(NoneTransformer));
        assert!(transformer.transform("foo").contains(&"foo"));

        let transformer: ArrayTransformer<i32, i32> = ArrayTransformer::new(Box::from(NoneTransformer));
        assert!(transformer.transform(1).contains(&1));
    }
}
