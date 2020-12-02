const EXPECTED_SUM: u64 = 2020;

fn main() {
    let input_path = std::env::args_os()
        .skip(1)
        .next()
        .expect("Specify input as arg 1");
    let mut entries = std::fs::read_to_string(input_path)
        .expect("Failed to read input")
        .lines()
        .map(|l| u64::from_str_radix(l, 10))
        .collect::<Result<Vec<_>, _>>()
        .expect("Input is not valid integers");

    entries.sort_unstable();
    for (i, &entry) in entries.iter().enumerate() {
        let remaining = match EXPECTED_SUM.checked_sub(entry) {
            Some(remaining) => remaining,
            // entry is larger than EXPECTED_SUM. No need to continue
            None => break,
        };
        for &other_entry in entries.iter().skip(i + 1) {
            if other_entry == remaining {
                println!("{}", entry * other_entry);
                break;
            }
            if other_entry > remaining {
                // We went too far. Everything after this will be too large
                break;
            }
        }
    }
}
