use std::fs;
use std::ops::Not;
use regex::Regex;


fn is_valid(passport: &str) -> bool {
    let required_fields = ["byr:", "iyr:", "eyr:", "hgt:", "hcl:", "ecl:", "pid:"];
    for required_field in &required_fields {
        if passport.matches(required_field).count() != 1 {
            return false;
        }
    }
    return true;
}

pub fn one() -> usize {
    let input = fs::read_to_string("src/day4/input").expect("Unable to read file");
    let passports: Vec<&str> = input.split("\n\n").collect();

    let mut valid_passports = 0;
    for passport in passports {
        if is_valid(passport) {
            valid_passports += 1;
        }
    }

    return valid_passports;

}

pub fn two() -> usize {
    let input = fs::read_to_string("src/day4/input").expect("Unable to read file");
    let passports: Vec<&str> = input.split("\n\n").collect();

    let mut valid_passports = 0;
    for passport in passports {
        if is_valid(passport).not() {
            continue;
        }
        let mut passport_struct = Passport::default();
        let working_passport = passport.replace("\n", " ");
        let attributes : Vec<&str> = working_passport.split(" ").collect();
        for attribute in attributes {
            let key_value_attribute: Vec<&str> = attribute.split(":").collect();
            match key_value_attribute[0] {
                //["byr:", "iyr:", "eyr:", "hgt:", "hcl:", "ecl:", "pid:"]
                "byr" => passport_struct.birth_year = key_value_attribute[1].parse().unwrap(),
                "iyr" => passport_struct.issue_year = key_value_attribute[1].parse().unwrap(),
                "eyr" => passport_struct.expiration_year = key_value_attribute[1].parse().unwrap(),
                "hgt" => passport_struct.height = key_value_attribute[1].to_string(),
                "hcl" => passport_struct.hair_color = key_value_attribute[1].to_string(),
                "ecl" => passport_struct.eyes_color = key_value_attribute[1].to_string(),
                "pid" => passport_struct.passport_id = key_value_attribute[1].to_string(),
                "cid" => passport_struct.cid = key_value_attribute[1].to_string(),
                _ => println!("Weird key: {}", key_value_attribute[0])
            }
        }

        if passport_struct.is_valid() {
            //println!("{:?}", passport_struct);
            valid_passports += 1;
        }
        drop(passport_struct);
    }

    return valid_passports;

}

#[derive(Debug, Default)]
struct Passport {
    birth_year: i32, // >= 1920 <= 2002
    issue_year: i32, // >= 2010 <= 2020
    expiration_year: i32, // >=2020, <= 2030
    height: String, // XXXcm or XXin cm: >= 150 <= 193; in: >= 59 <= 76
    hair_color: String, // #[0-9a-f]{6}
    eyes_color: String, // amb|blu|brn|gry|grn|hzl|oth
    passport_id: String, // [0-9]{9}
    cid: String // no validation
}

impl Passport {
    fn is_valid(&self) -> bool {
        if self.birth_year < 1920 {
            return false;
        }
        if self.birth_year > 2002 {
            return false;
        }
        if self.issue_year < 2010 {
            return false;
        }
        if self.issue_year > 2020 {
            return false;
        }
        if self.expiration_year < 2020 {
            return false;
        }
        if self.expiration_year > 2030 {
            return false;
        }
        let height_regex = Regex::new(r"^(1([5-8]{1}\d{1}|9[0-3])cm|(6\d{1}|7[0-6]|59)in)$").unwrap();
        if height_regex.is_match(&self.height).not() {
            return false;
        }
        let hair_color_regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        if hair_color_regex.is_match(&self.hair_color).not() {
            return false;
        }
        let eyes_color_regex = Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap();
        if eyes_color_regex.is_match(&self.eyes_color).not() {
            return false;
        }
        let passport_id_regex = Regex::new(r"^[0-9]{9}$").unwrap();
        if passport_id_regex.is_match(&self.passport_id).not() {
            return false;
        }
        return true
    }
}
