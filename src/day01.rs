use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};

pub fn run(input_path: &Path) -> bool {
    let file = File::open(input_path).expect("Could not open path");
    let reader = BufReader::new(file);
    let lines: Vec<u32> = reader.lines().map(|l| l.unwrap().parse::<u32>().unwrap() ).collect();

    print!("Part 1: ");
    let _product_part1 = find_matches(&lines, 2020, 2);
    print!("Part 2: ");
    let _product_part2 = find_matches(&lines, 2020, 3);

    return true;
}

fn find_matches(lines: &Vec<u32>, sum_match: u32, num_terms: usize) -> Option<u32> {
    let mut indices: Vec<usize> = vec![0; num_terms];
    for i in 0..num_terms { indices[i] = i; }
    
    loop {
        let mut sum = 0;
        for i in 0..num_terms { sum += lines[indices[i]]; }
        if sum == sum_match { 
            let mut product = 1;
            for i in 0..num_terms { 
                print!("{}", {lines[indices[i]]});
                product *= lines[indices[i]];
                if i < (num_terms - 1) { print!(" + "); }
            }
            println!(" = {} => product = {}", sum, product);
            return Some(product);
        }

        // Increment last index
        indices[num_terms - 1] += 1;
        
        // Check overflows.
        for i in (1..num_terms).rev() {
            if indices[i] == lines.len() {
                indices[i - 1] += 1;
                indices[i] = (indices[i - 1] + 1).min(lines.len()-1);
            }
        }
        
        if indices[0] == (lines.len() - 1) {break;}
    }

    return None;
}
