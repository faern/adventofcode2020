use std::collections::HashSet;

fn main() {
    let input_path = std::env::args_os()
        .skip(1)
        .next()
        .expect("Specify input as arg 1");
    let rules = std::fs::read_to_string(input_path).expect("Failed to read input");

    let mut outermost_bag_types = bag_types_that_can_hold_bag_type(&rules, "shiny gold");
    let mut outer_bag_types = outermost_bag_types.clone();
    loop {
        let mut new_outer_bag_types = HashSet::new();
        for bag_type in &outer_bag_types {
            new_outer_bag_types.extend(bag_types_that_can_hold_bag_type(&rules, bag_type));
        }
        if new_outer_bag_types.is_empty() {
            break;
        }
        outermost_bag_types.extend(&new_outer_bag_types);
        outer_bag_types = new_outer_bag_types;
    }
    println!("{}", outermost_bag_types.len());
}

fn bag_types_that_can_hold_bag_type<'a>(rules: &'a str, inner_bag_type: &str) -> HashSet<&'a str> {
    let mut outer_bag_types = HashSet::new();
    for rule in rules.split("\n") {
        let mut rule_parts = rule.split(" bags contain ");
        let outer_bag_type = rule_parts.next().unwrap();
        let bag_content = rule_parts.next().unwrap();
        if bag_content.contains(inner_bag_type) {
            outer_bag_types.insert(outer_bag_type);
        }
    }
    outer_bag_types
}
