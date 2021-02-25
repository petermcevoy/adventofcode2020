use std::path::Path;
use std::collections::{HashSet, HashMap};
use super::common;

pub fn run(input_path: &Path) -> bool {
    let input = common::file_as_string(input_path);
    let num_types = calc_num_bag_types_that_can_contain_shiny_gold(&input);
    assert_eq!(num_types, 128);
    true
}

fn calc_num_bag_types_that_can_contain_shiny_gold(input: &str) -> usize {
    let mut bag_map: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.lines() {
        let mut spec_split = line.split(" bags contain ");
        let bag_key = spec_split.next().unwrap();
        let bag_contains = spec_split.next().unwrap().split(", ");
        
        for b in bag_contains {
            let b = b.trim_end_matches(".").trim_end_matches(" bag").trim_end_matches(" bags");
            if b != "no other" {
                let b = b.trim_start_matches(|c: char| c.is_numeric()).trim_start(); // remove number and space
                let is_contained_in = bag_key;
                match bag_map.get_mut(b) {
                    Some(v) => v.push(is_contained_in),
                    None => { bag_map.insert(b, vec![bag_key]); }
                }
            }
        }
    }

    let mut types: HashSet<&str> = HashSet::new();
    let look_for_key = "shiny gold";
    let mut new_types_stack: Vec<&str> = vec![look_for_key];
    while let Some(k) = new_types_stack.pop() {
        match bag_map.get(k) {
            Some(vec) => { 
                for e in vec { 
                    types.insert(e); 
                    new_types_stack.push(e);
                } 
            },
            None => {}
        }
    }

    println!("Number of bag types that can contain at least one {}: {}", look_for_key, types.len());
    return types.len();
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn example() {
        let example_str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
";

        let num_types = calc_num_bag_types_that_can_contain_shiny_gold(example_str);
        assert_eq!(num_types, 4);
    }
}

