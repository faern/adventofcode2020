// byr (Birth Year)
// iyr (Issue Year)
// eyr (Expiration Year)
// hgt (Height)
// hcl (Hair Color)
// ecl (Eye Color)
// pid (Passport ID)
// cid (Country ID)
//
// All fields must exist, except cid which is optional

use std::collections::HashMap;

fn main() {
    let input_path = std::env::args_os()
        .skip(1)
        .next()
        .expect("Specify input as arg 1");
    let input = std::fs::read_to_string(input_path).expect("Failed to read input");

    let required_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    // A map from field name to if it's found or not. We reset and reuse the same
    // instance for every passport.
    let mut found_fields = HashMap::with_capacity(required_fields.len());
    for required_field in &required_fields {
        found_fields.insert(*required_field, false);
    }

    let mut valid_passports = 0;
    for passport in input.split("\n\n") {
        for field in passport.split(&['\n', ' '][..]) {
            let field_name = field.split(':').next().unwrap();
            if let Some(found) = found_fields.get_mut(field_name) {
                *found = true;
            }
        }
        if found_fields.values().all(|v| *v) {
            valid_passports += 1;
        }
        found_fields.values_mut().for_each(|v| *v = false);
    }
    println!("{}", valid_passports);
}
