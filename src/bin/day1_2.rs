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
    for (i1, &entry1) in entries.iter().enumerate() {
        let remaining = match EXPECTED_SUM.checked_sub(entry1) {
            Some(remaining) => remaining,
            None => break,
        };
        for (i2, &entry2) in entries.iter().skip(i1 + 1).enumerate() {
            let remaining = match remaining.checked_sub(entry2) {
                Some(remaining) => remaining,
                None => break,
            };
            for &entry3 in entries.iter().skip(i2 + 1) {
                if entry3 == remaining {
                    println!("{}", entry1 * entry2 * entry3);
                    break;
                }
                if entry3 > remaining {
                    break;
                }
            }
        }
    }
}
