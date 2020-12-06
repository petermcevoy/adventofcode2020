use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Seek};
use std::io;
use std::error::Error;

pub fn run(input_path: &Path) -> bool {
    let records = match parse(input_path) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("There was a problem parsing the input file:");
            eprintln!("{}", e);
            return false;
        }
    };

    let num_valid_part1 = records.iter().filter(|r| check_policy1(&r)).count();
    println!("Part 1: num valid passwords: {}", num_valid_part1);
    
    let num_valid_part2 = records.iter().filter(|r| check_policy2(&r)).count();
    println!("Part 2: num valid passwords: {}", num_valid_part2);

    return true;
}

#[derive(Debug)]
struct PasswordRecord {
    range_start: usize,
    range_end: usize,
    charachter: char,
    password: String
}

fn parse_err(e: Option<&dyn Error>, line: usize) -> io::Error {
    let msg = match e {
        Some(e) => format!(": {}", e),
        None => "".to_string()
    };
    io::Error::new( io::ErrorKind::Other, format!("Could not parse line {}{}", line+1, msg))
}

fn parse(filepath: &Path) -> Result<Vec<PasswordRecord>, Box<dyn Error>> {
    let mut file = File::open(filepath).expect("Could not open file");
    let num_lines = BufReader::new(&file).lines().count();
    file.seek(io::SeekFrom::Start(0))?;
    
    let mut records = Vec::<PasswordRecord>::with_capacity(num_lines);
    let reader = BufReader::new(&file);
    for (i_line, line) in reader.lines().enumerate() {
        let line = line?;
        let parts: Vec<&str> = line.split(":").collect();
        let policy_str = parts[0];
        let password_str = parts[1];

        let policy_parts: Vec<&str> = policy_str.split(" ").collect();
        let policy_char = policy_parts[1].chars().next()
            .ok_or(parse_err(None, i_line))?;
        let range_parts: Vec<&str> = policy_parts[0].split("-").collect();
        let range_start = range_parts[0].parse::<usize>()
            .map_err(|e| parse_err(Some(&e), i_line))?;
        let range_end = range_parts[1].parse::<usize>()
            .map_err(|e| parse_err(Some(&e), i_line))?;

        records.push(PasswordRecord { 
            range_start,
            range_end,
            charachter: policy_char,
            password: String::from(password_str.trim())
        });
    }

    return Ok(records);
}

fn check_policy1(record: &PasswordRecord) -> bool {
    let char_count = record.password.matches(record.charachter).count();
    return char_count >= record.range_start && char_count <= record.range_end;
}

fn check_policy2(record: &PasswordRecord) -> bool {
    let i1 = record.range_start - 1;
    let i2 = record.range_end - 1;
    let char1 = record.password.chars().nth(i1).unwrap();
    let char2 = record.password.chars().nth(i2).unwrap();

    return (char1 == record.charachter) ^ (char2 == record.charachter);
}
