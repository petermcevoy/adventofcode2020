use std::path::Path;
use std::collections::HashSet;
use super::common;

pub fn run(input_path: &Path) -> bool {
    let input = common::file_as_string(input_path);
    
    println!("Part1");
    let declarations_part1 = parse_part1(input.as_str());
    let sum: usize = declarations_part1.iter().map(|decl| decl.len()).sum();
    println!("Sum: {}", sum);
    assert_eq!(sum, 6443);
    
    println!("Part2");
    let v = parse_part2(input.as_str());
    let sum: u32 = v.iter().map(|decl| decl).sum();
    println!("Sum: {}", sum);
    assert_eq!(sum, 3232);

    true
}

type GroupDeclaration = HashSet<char>;

fn parse_part1(input: &str) -> Vec<GroupDeclaration> {
    let mut declarations: Vec<GroupDeclaration> = Vec::new();

    for group_decl in input.split("\n\n") {
        declarations.push(group_decl.chars().filter(|&c| c != '\n').collect());
    }
    declarations
}

fn parse_part2(input: &str) -> Vec<u32> {
    let mut declarations: Vec<u32> = Vec::new();

    for group_decl in input.split("\n\n") {
        // Within the group, how many questions did everyone answer yes to?
        // answers are from a-z. We need 26 positions -> 32 bits will do.
        let group_bitflags = group_decl.lines()
            .map(|l| {
                // Turn individual char string into a 32bit bitfield
                l.chars().fold(0u32, |mask, c| {
                    assert!(('a'..='z').contains(&c));
                    let mut b = [0; 1];
                    c.encode_utf8(&mut b);
                    let asci_count: u8 = b[0] - b'a';

                    mask | (0b1 << asci_count)
                })
            })
            .fold(0xffffffff, |mask, c| mask & c);

        let num_yes = group_bitflags.count_ones();
        declarations.push(num_yes);
    }
    declarations
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_INPUT: &str = "\
abc

a
b
c

ab
ac

a
a
a
a

b";

    #[test]
    fn example_part1() {
        let declarations = parse_part1(EXAMPLE_INPUT);
        for (i, dec) in declarations.iter().enumerate() {
            println!("{}: {}", i, dec.len());
            match i {
                0 | 1 | 2 => {
                    assert_eq!(dec.len(), 3);
                    assert_eq!(dec.contains(&'a'), true);
                    assert_eq!(dec.contains(&'b'), true);
                    assert_eq!(dec.contains(&'c'), true);
                    assert_eq!(dec.contains(&'d'), false);
                },
                3 => {
                    assert_eq!(dec.len(), 1);
                    assert_eq!(dec.contains(&'a'), true);
                    assert_eq!(dec.contains(&'b'), false);
                },
                4 => {
                    assert_eq!(dec.len(), 1);
                    assert_eq!(dec.contains(&'a'), false);
                    assert_eq!(dec.contains(&'b'), true);
                },
                _ => { panic!("Unexpected group"); }
            }
        }

        let sum: usize = declarations.iter().map(|decl| decl.len()).sum();
        assert_eq!(11, sum);
    }
    
    #[test]
    fn example_part2() {
        let declarations = parse_part2(EXAMPLE_INPUT);
        for (i, dec) in declarations.iter().enumerate() {
            println!("{}: {}", i, dec);
            match i {
                0 => {
                    assert_eq!(dec, &3);
                },
                1 => {
                    assert_eq!(dec, &0);
                },
                2 | 3 => {
                    assert_eq!(dec, &1);
                },
                4 => {
                    assert_eq!(dec, &1);
                },
                _ => { panic!("Unexpected group"); }
            }
        }

        let sum: u32 = declarations.iter().map(|decl| decl).sum();
        assert_eq!(6, sum);
    }
}

