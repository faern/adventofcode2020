fn main() {
    let input_path = std::env::args_os()
        .skip(1)
        .next()
        .expect("Specify input as arg 1");
    let input = std::fs::read_to_string(input_path).expect("Failed to read input");

    let mut sum = 0;
    for group_questions in input.split("\n\n") {
        // Start out by saying the group answered yes to all questions.
        let mut group_yes_answers: u32 = 0b00000011_11111111_11111111_11111111;
        for person_questions in group_questions.split("\n") {
            // Catch an empty trailing line in the input data and skip it
            if person_questions.is_empty() {
                continue;
            }
            let mut person_yes_answers: u32 = 0;
            for question in person_questions.as_bytes() {
                person_yes_answers |= 1 << (question - b'a');
            }
            // Keep only 1-bits where the group AND this person has answered yes.
            group_yes_answers &= person_yes_answers;
        }
        sum += group_yes_answers.count_ones();
    }
    println!("{}", sum);
}
