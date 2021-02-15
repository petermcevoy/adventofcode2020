use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

pub fn run(input_path: &Path) -> bool {
    
    println!("Part1");
    let file = File::open(input_path).expect("Could not open file");
    let declarations_part1 = parse_part1(BufReader::new(&file));
    let sum: usize = declarations_part1.iter().map(|decl| decl.len()).sum();
    println!("Sum: {}", sum);
    assert_eq!(sum, 6443);
    
    println!("Part2");
    let file = File::open(input_path).expect("Could not open file");
    let v = parse_part2(BufReader::new(&file));
    let sum: u32 = v.iter().map(|decl| decl).sum();
    println!("Sum: {}", sum);
    assert_eq!(sum, 3232);

    true
}

type GroupDeclaration = HashSet<char>;

fn parse_part1<R: BufRead>(reader: R) -> Vec<GroupDeclaration> {
    let mut declarations: Vec<GroupDeclaration> = Vec::new();
    let mut current_declaration = GroupDeclaration::new();

    for line in reader.lines() {
        match line.as_deref() {
            Ok("") => {
                declarations.push(current_declaration);
                current_declaration = GroupDeclaration::new();
            }, 
            Ok(v) => {
                for c in v.chars() {
                    current_declaration.insert(c);
                }
            }, 
            _ => panic!("Unexpected line")
        }
    }
    declarations.push(current_declaration); // Add final declaration
    declarations
}

fn parse_part2<R: BufRead>(reader: R) -> Vec<u32> {
    let mut declarations: Vec<u32> = Vec::new();

    let input_vec: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
    let input = input_vec.join("\n");
    
    for group_decl_str in input.split("\n\n") {
        
        // Within the group, how many questions did everyone answer yes to?
        // answers are from a-z. We need 26 positions -> 32 bits will do.
        let mut group_bitflags: u32 = 0xffffffff;

        for individual_decl_str in group_decl_str.lines() {
            // For the individual within the group, make a mask of all
            // the questions they answered yes to.
            let mut individual_mask: u32 = 0x0;
            for c in individual_decl_str.chars() {
                if ('a'..='z').contains(&c) {
                    let mut b = [0; 1];
                    c.encode_utf8(&mut b);
                    let asci_count: u8 = b[0] - b'a';
                    let bitflag: u32 = 0b1 << asci_count;
                    individual_mask = individual_mask | bitflag;
                } else {
                    panic!("Unexpected answer, should be a..z!");
                }

            }
            
            group_bitflags = group_bitflags & individual_mask;
        }
        let num_yes = group_bitflags.count_ones();
        declarations.push(num_yes);
        //println!("---\n{}\ncount: {}", group_decl_str, num_yes);
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
        let declarations = parse_part1(BufReader::new(EXAMPLE_INPUT.as_bytes()));
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
        let declarations = parse_part2(BufReader::new(EXAMPLE_INPUT.as_bytes()));
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
        assert_eq!(7, sum);
    }
}

