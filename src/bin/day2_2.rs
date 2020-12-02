fn main() {
    let input_path = std::env::args_os()
        .skip(1)
        .next()
        .expect("Specify input as arg 1");
    let input = std::fs::read_to_string(input_path).expect("Failed to read input");

    let mut valid_passwords = 0;
    for entry in input.lines() {
        // <i1>-<i2> <bound char>: <password>
        let mut entry_parts = entry.split(&['-', ':', ' '][..]);

        let i1 = entry_parts.next().unwrap().parse::<usize>().unwrap() - 1;
        let i2 = entry_parts.next().unwrap().parse::<usize>().unwrap() - 1;

        let bounded_char: char = entry_parts.next().unwrap().chars().next().unwrap();
        let _ = entry_parts.next().unwrap();
        let password = entry_parts.next().unwrap();

        let i1_match = password.chars().skip(i1).next() == Some(bounded_char);
        let i2_match = password.chars().skip(i2).next() == Some(bounded_char);
        if i1_match ^ i2_match {
            valid_passwords += 1;
        }
    }
    println!("{}", valid_passwords);
}
