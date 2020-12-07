use regex::Regex;
use std::collections::HashMap;

lazy_static::lazy_static! {
    static ref BAG_RULE_RE: Regex = Regex::new(r"(\d+) (\w+ \w+) bags?").unwrap();
}

fn main() {
    let input_path = std::env::args_os()
        .skip(1)
        .next()
        .expect("Specify input as arg 1");
    let input = std::fs::read_to_string(input_path).expect("Failed to read input");

    // A map from bag type to a list of how many of which bag type it contains.
    let mut rules: HashMap<&str, Vec<(u32, &str)>> = HashMap::new();
    for rule in input.split("\n") {
        let mut rule_parts = rule.split(" bags contain ");
        let rule_bag_type = rule_parts.next().unwrap();
        let mut rule_bag_content = Vec::new();
        for inner_bag in BAG_RULE_RE.captures_iter(rule_parts.next().unwrap()) {
            let num = inner_bag.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let bag_type = inner_bag.get(2).unwrap().as_str();
            rule_bag_content.push((num, bag_type));
        }
        rules.insert(rule_bag_type, rule_bag_content);
    }

    println!("{}", num_bags_in_bag("shiny gold", &rules));
}

fn num_bags_in_bag<'a>(bag_type: &'a str, rules: &HashMap<&'a str, Vec<(u32, &'a str)>>) -> u32 {
    let mut bags_in_bag_type: u32 = 0;
    for (num, inner_bag_type) in rules.get(bag_type).unwrap() {
        bags_in_bag_type += num * (1 + num_bags_in_bag(inner_bag_type, rules));
    }
    bags_in_bag_type
}
