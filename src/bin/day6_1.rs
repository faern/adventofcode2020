fn main() {
    let input_path = std::env::args_os()
        .skip(1)
        .next()
        .expect("Specify input as arg 1");
    let input = std::fs::read_to_string(input_path).expect("Failed to read input");

    let mut sum: u32 = 0;
    for group_questions in input.split("\n\n") {
        let mut yes_answers = [0u8; 26];
        for person_questions in group_questions.split("\n") {
            for question in person_questions.as_bytes() {
                yes_answers[(question - b'a') as usize] = 1;
            }
        }
        sum += yes_answers.iter().sum::<u8>() as u32;
    }
    println!("{}", sum);
}
