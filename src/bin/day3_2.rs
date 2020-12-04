// Right 1, down 1.
// Right 3, down 1. (This is the slope you already checked.)
// Right 5, down 1.
// Right 7, down 1.
// Right 1, down 2.

fn main() {
    let input_path = std::env::args_os()
        .skip(1)
        .next()
        .expect("Specify input as arg 1");
    let input = std::fs::read_to_string(input_path).expect("Failed to read input");

    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut collision_product = 1;
    for (x_step, y_step) in &slopes {
        collision_product *= trees_in_slope(input.lines(), *x_step, *y_step);
    }
    println!("{}", collision_product);
}

fn trees_in_slope<'a>(map: impl Iterator<Item = &'a str>, x_step: usize, y_step: usize) -> u32 {
    let mut num_trees = 0;
    for (y, line) in map.step_by(y_step).enumerate() {
        let x = y * x_step;
        let is_tree = line.as_bytes()[x % line.len()] == b'#';
        if is_tree {
            num_trees += 1;
        }
    }
    num_trees
}
