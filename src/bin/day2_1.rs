use std::convert::TryInto;
use std::ops::RangeInclusive;

fn main() {
    let input_path = std::env::args_os()
        .skip(1)
        .next()
        .expect("Specify input as arg 1");
    let input = std::fs::read_to_string(input_path).expect("Failed to read input");

    let mut valid_passwords = 0;
    for entry in input.lines() {
        // <lower>-<upper> <bound char>: <password>
        let mut entry_parts = entry.split(&['-', ':', ' '][..]);

        let lower_bound: u32 = entry_parts.next().unwrap().parse().unwrap();
        let upper_bound: u32 = entry_parts.next().unwrap().parse().unwrap();
        let bound = RangeInclusive::new(lower_bound, upper_bound);

        let bounded_char: char = entry_parts.next().unwrap().chars().next().unwrap();
        let _ = entry_parts.next().unwrap();
        let password = entry_parts.next().unwrap();

        let bounded_char_count: u32 = password
            .chars()
            .filter(|&c| c == bounded_char)
            .count()
            .try_into()
            .unwrap();
        if bound.contains(&bounded_char_count) {
            valid_passwords += 1;
        }
    }
    println!("{}", valid_passwords);
}
