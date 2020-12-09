#[macro_use]
extern crate lazy_static;

use std::env;
use std::fs;
use std::fmt;
use std::str::Split;
use regex::{Regex, Captures};
use itertools::Itertools;
use validator::{Validate, ValidationError};

fn main() {
    let args: Vec<String> = env::args().collect();
    let raw_input = fs::read_to_string(&args[1])
        .expect("Something went wrong reading the file");
    

    //TODO: implement the solver function
    println!("Solution Part 1:  {:?}", solve_part_1(raw_input.as_str()));
    println!("Solution Part 2:  {:?}", solve_part_2(raw_input.as_str()));
}

fn parse(raw_input: &str) {
    raw_input.lines()
        .map(|s| s)
        .collect::<Vec<&str>>();
}

fn solve_part_1(raw_input: &str) -> u32 {
    let parser = PassportParser::new(raw_input);
    let mut valid_count: u32 = 0;
    for passport in parser {
        if passport.is_valid() {
            valid_count += 1;
        }
    }
    valid_count
}

fn solve_part_2(raw_input: &str) -> u32 {
    let parser = PassportParser::new(raw_input);
    let mut valid_count: u32 = 0;
    for passport in parser {
        if passport.is_valid() {
            match passport.validate() {
                Ok(_) => valid_count += 1,
                Err(e) => {
                    println!("Invalid Passport {:?}\n{}", passport, e);
                },
            };
        }
    }
    valid_count
}

struct PassportParser<'a> {
    batch: Split<'a, &'a str>,
}
impl<'a> PassportParser<'a> {
    fn new(batch: &str) -> PassportParser {
        PassportParser {
            batch: batch.split("\n\n"),
        }
    }

    fn parse(passport_str: &str) -> Passport {
        let separator = Regex::new(r"\s").expect("Invalid regex");
        let mut passport = Passport::new();
        for token in separator.split(passport_str).into_iter() {
            if token.trim().is_empty() {
                continue;
            }
            let (key, val) = token.splitn(2, ":").collect_tuple().unwrap();
            match key {
                "byr" => passport.birth_year = Some(val.parse().unwrap()),
                "iyr" => passport.issue_year = Some(val.parse().unwrap()),
                "eyr" => passport.expiration_year = Some(val.parse().unwrap()),
                "hgt" => passport.height = Some(String::from(val)),
                "hcl" => passport.hair_color = Some(String::from(val)),
                "ecl" => passport.eye_color = Some(String::from(val)),
                "pid" => passport.passport_id = Some(val.parse().unwrap()),
                "cid" => passport.country_id = Some(String::from(val)),
                _ => panic!("Unknown token found during parsing batch files!"),
            }
        }
        passport
    }
}
impl<'a> Iterator for PassportParser<'a> {
    type Item = Passport;

    fn next(&mut self) -> Option<Self::Item> {
        match self.batch.next() {
            None => None,
            Some(s) => Some(PassportParser::parse(s)),
        }
    }
}

lazy_static! {
    static ref RE_PASSPORT_ID: Regex = Regex::new(r"^\d{9}$").unwrap();
    static ref RE_HEX_COLOR: Regex = Regex::new(r"^#[0-9a-fA-F]{6}$").unwrap();
}

fn validate_eye_color(eye_color: &str) -> Result<(), ValidationError> {
    if !["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&eye_color) {
        return Err(ValidationError::new("invalid_eye_color"));
    }
    Ok(())
}

fn validate_height(height: &str) -> Result<(), ValidationError> {
    let height_pattern = Regex::new(r"^(\d+)(in|cm)$").expect("There was an error in the regex pattern!");
    if !height_pattern.is_match(height) {
    }
    match height_pattern.captures(height) {
        // Access captures groups via Captures::at
        // Prints Some("2016")
        Some(x) => {
            let height_num: u32 = x.get(1).map_or(0, |m| m.as_str().parse::<u32>().unwrap());
            let height_unit: &str = x.get(2).map_or("", |m| m.as_str());
            match height_unit {
                "in" => {
                    //    If in, the number must be at least 59 and at most 76.
                    if height_num < 59 || height_num > 76 {
                        return Err(ValidationError::new("invalid_height_value"));
                    }
                },
                "cm" => {
                    //    If cm, the number must be at least 150 and at most 193.
                    if height_num < 150 || height_num > 193 {
                        return Err(ValidationError::new("invalid_height_value"));
                    }
                },
                _ => return Err(ValidationError::new("invalid_height_unit")),
            }
        },
        None => return Err(ValidationError::new("invalid_height_format")),
    }
    
    Ok(())
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Validate)]
struct Passport {
    // byr (Birth Year) - four digits; at least 1920 and at most 2002.
    #[validate(range(min = 1920, max = 2002))]
    birth_year: Option<u32>,
    
    // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    #[validate(range(min = 2010, max = 2020))]
    issue_year: Option<u32>,

    // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    #[validate(range(min = 2020, max = 2030))]
    expiration_year: Option<u32>,

    // hgt (Height) - a number followed by either cm or in:
    //    If cm, the number must be at least 150 and at most 193.
    //    If in, the number must be at least 59 and at most 76.
    #[validate(custom = "validate_height")]
    height: Option<String>,

    // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    #[validate(regex = "RE_HEX_COLOR")]
    hair_color: Option<String>,

    // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    #[validate(custom = "validate_eye_color")]
    eye_color: Option<String>,
    
    // pid (Passport ID) - a nine-digit number, including leading zeroes.
    #[validate(regex = "RE_PASSPORT_ID")]
    passport_id: Option<String>,

    // cid (Country ID) - ignored, missing or not.
    country_id: Option<String>,
}
impl Passport {
    fn new() -> Passport {
        Passport {
            birth_year: Option::None,
            issue_year: Option::None,
            expiration_year: Option::None,
            height: Option::None,
            hair_color: Option::None,
            eye_color: Option::None,
            passport_id: Option::None,
            country_id: Option::None,
        }
    }
    
    fn is_valid(&self) -> bool {
        let mut is_valid = self.birth_year.is_some()
            && self.issue_year.is_some()
            && self.expiration_year.is_some()
            && self.height.is_some()
            && self.hair_color.is_some()
            && self.eye_color.is_some()
            && self.passport_id.is_some();
        
        // validate the fields
        
        // byr (Birth Year) - four digits; at least 1920 and at most 2002.
        //let birth_year_pattern = Regex::new(r"^\d{4}$").expect("There was an error in the regex pattern!");
        //is_valid = is_valid && birth_year_pattern.is_match(self.birth_year.unwrap().as_str());
        
        // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
        
        // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
        
        // hgt (Height) - a number followed by either cm or in:
        //    If cm, the number must be at least 150 and at most 193.
        //    If in, the number must be at least 59 and at most 76.
        
        // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
        
        // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
        
        // pid (Passport ID) - a nine-digit number, including leading zeroes.
        
        // cid (Country ID) - ignored, missing or not.

        is_valid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part_1_a() {
        let expected = 2;
        let raw_input = "\
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

        assert_eq!(expected, solve_part_1(raw_input));
    }

    #[test]
    fn test_passport_parser_1() {
        let expected = 4;
        let raw_input = "\
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

        let parser = PassportParser::new(raw_input);

        assert_eq!(expected, parser.count());
    }

    #[test]
    fn test_passport_is_valid() {
        let passport = Passport {
            birth_year: Some(1937),
            issue_year: Some(2017),
            expiration_year: Some(2020),
            height: Some(String::from("183cm")),
            hair_color: Some(String::from("#fffffd")),
            eye_color: Some(String::from("gry")),
            passport_id: Some(String::from("860033327")),
            country_id: Some(String::from("147")),
        };

        assert_eq!(true, passport.is_valid());
    }

    #[test]
    fn test_passport_is_invalid() {
        let passport = Passport {
            birth_year: None,
            issue_year: Some(2017),
            expiration_year: Some(2020),
            height: Some(String::from("183cm")),
            hair_color: Some(String::from("#fffffd")),
            eye_color: Some(String::from("gry")),
            passport_id: Some(String::from("860033327")),
            country_id: Some(String::from("147")),
        };

        assert_eq!(false, passport.is_valid());
    }

    #[test]
    fn test_passport_cid_is_optional() {
        let passport = Passport {
            birth_year: Some(1937),
            issue_year: Some(2017),
            expiration_year: Some(2020),
            height: Some(String::from("183cm")),
            hair_color: Some(String::from("#fffffd")),
            eye_color: Some(String::from("gry")),
            passport_id: Some(String::from("860033327")),
            country_id: None,
        };

        assert_eq!(true, passport.is_valid());
    }

    #[test]
    fn test_passport_parse() {
        // given
        let raw_input = "\
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm";
        let expected_passport = Passport {
            birth_year: Some(1937),
            issue_year: Some(2017),
            expiration_year: Some(2020),
            height: Some(String::from("183cm")),
            hair_color: Some(String::from("#fffffd")),
            eye_color: Some(String::from("gry")),
            passport_id: Some(String::from("860033327")),
            country_id: Some(String::from("147")),
        };
        
        // when
        let parsed_passport = PassportParser::parse(raw_input);

        assert_eq!(expected_passport, parsed_passport);
    }
}