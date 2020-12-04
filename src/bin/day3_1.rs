fn main() {
    let input_path = std::env::args_os()
        .skip(1)
        .next()
        .expect("Specify input as arg 1");
    let input = std::fs::read_to_string(input_path).expect("Failed to read input");

    let mut num_trees = 0;
    for (y, line) in input.lines().enumerate() {
        let x = y * 3;
        let is_tree = line.as_bytes()[x % line.len()] == b'#';
        if is_tree {
            num_trees += 1;
        }
    }
    println!("{}", num_trees);
}
