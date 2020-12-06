fn main() {
    let input_path = std::env::args_os()
        .skip(1)
        .next()
        .expect("Specify input as arg 1");
    let input = std::fs::read_to_string(input_path).expect("Failed to read input");

    // An array where the index is the seat ID and the value is whether
    // or not it's found.
    let mut found_seat_ids = [false; u16::MAX as usize];
    for boarding_pass in input.lines() {
        let seat_id = decode_boarding_pass(boarding_pass);
        found_seat_ids[seat_id as usize] = true;
    }
    // Find the first position in the array where the content is true, false (my seat id), true
    let my_seat_id = &found_seat_ids[..]
        .windows(3)
        .position(|window| window == [true, false, true])
        .unwrap()
        + 1;
    println!("{}", my_seat_id);
}

fn decode_boarding_pass(boarding_pass: &str) -> u16 {
    let (row_data, column_data) = boarding_pass.split_at(7);
    let row = string_to_int(row_data, 7, 'F', 'B');
    let column = string_to_int(column_data, 3, 'L', 'R');
    row << 3 | column
}

fn string_to_int(data: &str, bits: usize, low: char, high: char) -> u16 {
    assert_eq!(data.len(), bits as usize, "Invalid string length");
    assert!(bits <= 16);
    let mut output = 0;
    for (i, c) in data.chars().enumerate() {
        let bit = match c {
            x if x == low => 0,
            x if x == high => 1,
            _ => panic!("Invalid indicator character"),
        };
        output |= bit << (bits - 1 - i);
    }
    output
}
