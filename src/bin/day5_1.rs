fn main() {
    let input_path = std::env::args_os()
        .skip(1)
        .next()
        .expect("Specify input as arg 1");
    let input = std::fs::read_to_string(input_path).expect("Failed to read input");

    let mut max_seat_id = 0;
    for boarding_pass in input.lines() {
        let seat_id = decode_boarding_pass(boarding_pass);
        if seat_id > max_seat_id {
            max_seat_id = seat_id;
        }
    }
    println!("{}", max_seat_id);
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
