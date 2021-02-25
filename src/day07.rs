use std::path::Path;
use std::collections::{HashSet, HashMap};
use super::common;

pub fn run(input_path: &Path) -> bool {
    let input = common::file_as_string(input_path);
    let (num_bags_contained, num_types_that_can_contain) = calc_bags(&input);
    assert_eq!(num_types_that_can_contain, 128);
    assert_eq!(num_bags_contained, 20189);
    true
}

fn calc_bags(input: &str) -> (usize, usize) {
    let mut bag_map_contains: HashMap<&str, Vec<(usize, &str)>> = HashMap::new();
    let mut bag_map_is_contained_in: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.lines() {
        let mut spec_split = line.split(" bags contain ");
        let bag_key = spec_split.next().unwrap();
        let bag_contains = spec_split.next().unwrap().split(", ");
        
        for b in bag_contains {
            let b = b.trim_end_matches(".").trim_end_matches(" bag").trim_end_matches(" bags");
            if b != "no other" {
                let n: usize = b.matches(char::is_numeric).nth(0).unwrap().parse().unwrap();
                let b = b.trim_start_matches(|c: char| c.is_numeric() || c == ' '); // remove number and space
                
                match bag_map_contains.get_mut(bag_key) {
                    Some(v) => v.push((n, b)),
                    None => { bag_map_contains.insert(bag_key, vec![(n, b)]); }
                }

                let is_contained_in = bag_key;
                match bag_map_is_contained_in.get_mut(b) {
                    Some(v) => v.push(is_contained_in),
                    None => { bag_map_is_contained_in.insert(b, vec![is_contained_in]); }
                }
            }
        }
    }

    let mut types: HashSet<&str> = HashSet::new();
    let look_for_key = "shiny gold";
    let mut new_types_stack: Vec<&str> = vec![look_for_key];
    while let Some(k) = new_types_stack.pop() {
        if let Some(vec) = bag_map_is_contained_in.get(k) {
            for e in vec { 
                types.insert(e); 
                new_types_stack.push(e);
            } 
        }
    }

    println!("Number of bag types that can contain at least one {}: {}", look_for_key, types.len());

    let mut bag_count = 0;
    let mut search_stack: Vec<(usize, &str)> = vec![(1, look_for_key)];
    while let Some((n_search, k_search)) = search_stack.pop() {
        if let Some(vec) = bag_map_contains.get(k_search) {
            for (n, k) in vec {
                bag_count += n_search*n;
                search_stack.push((n_search*n, k));
            }
        }
    }
    println!("Number of bags that 1 {} bag contains: {}", look_for_key, bag_count);


    return (bag_count, types.len());
}


#[cfg(test)]
mod tests {
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

        let (num_bags_contained, num_types_that_can_contain) = calc_bags(example_str);
        assert_eq!(num_types_that_can_contain, 4);
        assert_eq!(num_bags_contained, 32);

    }
}

