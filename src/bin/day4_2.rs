use std::collections::HashMap;
use std::ops::RangeInclusive;

fn main() {
    let input_path = std::env::args_os()
        .skip(1)
        .next()
        .expect("Specify input as arg 1");
    let input = std::fs::read_to_string(input_path).expect("Failed to read input");

    let required_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    // A map from field name to if it's found or not. We reset and reuse the same
    // instance for every passport.
    let mut valid_fields = HashMap::with_capacity(required_fields.len());
    for required_field in &required_fields {
        valid_fields.insert(*required_field, false);
    }

    let mut valid_passports = 0;
    for passport in input.split("\n\n") {
        for passport_entry in passport.split(&['\n', ' '][..]) {
            let mut entry_parts = passport_entry.split(':');
            let field_name = entry_parts.next().unwrap();
            let value = entry_parts.next().unwrap_or_default();
            if let Some(valid) = valid_fields.get_mut(field_name) {
                *valid = is_valid_field(field_name, value);
            }
        }
        if valid_fields.values().all(|v| *v) {
            valid_passports += 1;
        }
        valid_fields.values_mut().for_each(|v| *v = false);
    }
    println!("{}", valid_passports);
}

// byr (Birth Year) - four digits; at least 1920 and at most 2002.
// iyr (Issue Year) - four digits; at least 2010 and at most 2020.
// eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
// hgt (Height) - a number followed by either cm or in:
//     If cm, the number must be at least 150 and at most 193.
//     If in, the number must be at least 59 and at most 76.
// hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
// ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
// pid (Passport ID) - a nine-digit number, including leading zeroes.
// cid (Country ID) - ignored, missing or not.
fn is_valid_field(field: &str, value: &str) -> bool {
    match field {
        "byr" => RangeInclusive::new(1920, 2002).contains(&value.parse::<u16>().unwrap()),
        "iyr" => RangeInclusive::new(2010, 2020).contains(&value.parse::<u16>().unwrap()),
        "eyr" => RangeInclusive::new(2020, 2030).contains(&value.parse::<u16>().unwrap()),
        "hgt" => {
            if let Some(num) = value.strip_suffix("cm") {
                RangeInclusive::new(150, 193).contains(&num.parse::<u8>().unwrap_or(0))
            } else if let Some(num) = value.strip_suffix("in") {
                RangeInclusive::new(59, 76).contains(&num.parse::<u8>().unwrap_or(0))
            } else {
                false
            }
        }
        "hcl" => {
            let mut chars = value.chars();
            chars.next() == Some('#') && chars.all(|c| c.is_digit(16))
        }
        "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value),
        "pid" => value.len() == 9 && value.chars().all(|c| c.is_digit(10)),
        "cid" => true,
        _ => false,
    }
}
