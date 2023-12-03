mod digit;

use std::str::FromStr;

use once_cell::sync::Lazy;
use regex::Regex;

use digit::Digit;
use strum::VariantNames;

use crate::calibration_document::calibration_value::digit::string_digit::StringDigit;

pub struct CalibrationValue<'a>(&'a str);

impl<'a> CalibrationValue<'a> {
    pub fn new(value: &'a str) -> CalibrationValue<'a> {
        Self(value)
    }

    pub fn parse(&self) -> Option<u32> {
        static FIRST_DIGIT_RE: Lazy<Regex> = Lazy::new(|| {
            let string_digit_match = StringDigit::VARIANTS.join("|");
            Regex::new(&format!(
                ".*?((?<numeric_digit>\\d)|(?<string_digit>{string_digit_match}))"
            ))
            .unwrap()
        });
        static LAST_DIGIT_RE: Lazy<Regex> = Lazy::new(|| {
            let string_digit_match = StringDigit::VARIANTS.join("|");
            Regex::new(&format!(
                ".*((?<numeric_digit>\\d)|(?<string_digit>{string_digit_match}))"
            ))
            .unwrap()
        });

        let first_digit = {
            let capture = &FIRST_DIGIT_RE.captures(self.0)?;
            if let Some(numeric_digit) = capture.name("numeric_digit") {
                Digit::NumericalDigit(numeric_digit.as_str().parse::<u32>().unwrap())
            } else if let Some(string_digit) = capture.name("string_digit") {
                Digit::StringDigit(StringDigit::from_str(string_digit.as_str()).unwrap())
            } else {
                panic!()
            }
        };
        let last_digit = {
            let capture: &regex::Captures<'_> = &LAST_DIGIT_RE.captures(self.0)?;
            if let Some(numeric_digit) = capture.name("numeric_digit") {
                Digit::NumericalDigit(numeric_digit.as_str().parse::<u32>().unwrap())
            } else if let Some(string_digit) = capture.name("string_digit") {
                Digit::StringDigit(StringDigit::from_str(string_digit.as_str()).unwrap())
            } else {
                panic!()
            }
        };

        let value = format!("{}{}", first_digit.into_value(), last_digit.into_value());
        let value = value.parse::<u32>().unwrap();

        Some(value)
    }
}
