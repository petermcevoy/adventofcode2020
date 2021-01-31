use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

pub fn run(input_path: &Path) -> bool {
    let file = File::open(input_path).expect("Could not open file");
    let reader = BufReader::new(&file);
    let declarations = parse(reader);

    let sum: usize = declarations.iter().map(|decl| decl.len()).sum();
    println!("Sum: {}", sum);

    true
}

type GroupDeclaration = HashSet<char>;

fn parse<R: BufRead>(reader: R) -> Vec<GroupDeclaration> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let example_input = "\
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
        let declarations = parse(BufReader::new(example_input.as_bytes()));
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
}

