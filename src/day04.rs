use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};
use std::error::Error;

pub fn run(input_path: &Path) -> bool {
    let file = File::open(input_path).expect("Could not open file");
    let reader = BufReader::new(&file);
    let passports = match parse(reader) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("There was a problem parsing the input file:");
            eprintln!("{}", e);
            return false
        }
    };

    println!("[Part 1] Num valid passports: {}",
             passports.iter().filter(|p| p.is_valid_part1()).count());
    
    println!("[Part 2] Num valid passports: {}", 
             passports.iter().filter(|p| p.is_valid_part2()).count());

    return true;
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum PassportHeight {
    Centimeters(i32),
    Inches(i32),
}

#[derive(Debug, Clone)]
struct Passport {
    byr: Option<usize>,
    iyr: Option<usize>,
    eyr: Option<usize>,
    hgt: Option<PassportHeight>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}
impl Passport {
    fn is_valid_part1(&self) -> bool {
        // Make cid not required
        return 
            self.byr.is_some() &&
            self.iyr.is_some() &&
            self.eyr.is_some() &&
            self.hgt.is_some() &&
            self.hcl.is_some() &&
            self.ecl.is_some() &&
            self.pid.is_some()
            //self.cid.is_some()
    }
    
    fn is_valid_part2(&self) -> bool {
        let valid_byr = self.byr.map_or(false, |v| v >= 1920 && v <= 2002);
        let valid_iyr = self.iyr.map_or(false, |v| v >= 2010 && v <= 2020);
        let valid_eyr = self.eyr.map_or(false, |v| v >= 2020 && v <= 2030);
        let valid_hgt = self.hgt.map_or(false, |v| {
            match v {
                PassportHeight::Centimeters(h) => h >= 150 && h <= 193,
                PassportHeight::Inches(h) => h >= 59 && h <= 76,
            }
        });
        let valid_hcl = self.hcl.as_ref().map_or(false, |v| {
            (v.chars().nth(0) == Some('#')) &&
            (v.chars().count() == 7) &&
            (v.as_str()[1..].chars().all(|c| matches!(c, '0'..='9') || matches!(c, 'a'..='f')))
        });
        let valid_ecl = self.ecl.as_ref().map_or(false, |v| {
            ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].iter().any(|color| color == &v.as_str())
        });
        let valid_pid = self.pid.as_ref().map_or(false, |v| {
            v.chars().count() == 9 &&
            v.chars().all(|c| matches!(c, '0'..='9'))
        });

        return 
            valid_byr &&
            valid_iyr &&
            valid_eyr &&
            valid_hgt &&
            valid_hcl &&
            valid_ecl &&
            valid_pid
    }
}

fn parse<R: BufRead>(reader: R) -> Result<Vec<Passport>, Box<dyn Error>> {
    let mut passports: Vec<Passport> = Vec::new();
    let mut lines = reader.lines();

    let mut reached_eof = false;
    while !reached_eof {
        let mut passport = Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        };

        // keep iterating lines until we have an empty line,
        // indicating a new passport
        loop {
            let current_line = lines.next();
            let line_string = match current_line {
                Some(Ok(s)) => s,
                Some(Err(e)) => panic!(e),
                None => {
                    // We have reached the end of the file.
                    reached_eof = true;
                    break;
                } 
            };

            let pairs = line_string.split(' ');
            if line_string.trim() == "" {
                // No pairs, expect a new passport next
                break;
            }

            for pair in pairs {
                let mut key_value_iter = pair.split(':');
                let key = key_value_iter.next().expect("Expected key");
                let value = key_value_iter.next().expect("Expected value");
                match key {
                    "byr" => passport.byr = Some(value.parse::<usize>()?),
                    "iyr" => passport.iyr = Some(value.parse::<usize>()?),
                    "eyr" => passport.eyr = Some(value.parse::<usize>()?),
                    "hgt" => passport.hgt = {
                        let unit = &value[value.len()-2..];
                        let number = &value[..value.len()-2];
                        match unit {
                            "cm" => Some(PassportHeight::Centimeters(number.parse::<i32>()?)),
                            "in" => Some(PassportHeight::Inches(number.parse::<i32>()?)),
                            _ => None,
                        }
                    },
                    "hcl" => passport.hcl = Some(String::from(value)),
                    "ecl" => passport.ecl = Some(String::from(value)),
                    "pid" => passport.pid = Some(String::from(value)),
                    "cid" => passport.cid = Some(String::from(value)),
                    _ => panic!("Unexpected key {}", key)
                }
            }
        }

        passports.push(passport)
    }

    Ok(passports)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_input_part1() {
        let data = 
r#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"#;

        let passports = parse(BufReader::new(data.as_bytes())).unwrap();
        println!("{:?}", passports[0]);
        
        assert_eq!(passports[0].byr, Some(1937));
        assert_eq!(passports[0].iyr, Some(2017));
        assert_eq!(passports[0].eyr, Some(2020));
        assert_eq!(passports[0].hgt, Some(PassportHeight::Centimeters(183)));
        assert_eq!(passports[0].hcl, Some(String::from("#fffffd")));
        assert_eq!(passports[0].ecl, Some(String::from("gry")));
        assert_eq!(passports[0].pid, Some(String::from("860033327")));
        assert_eq!(passports[0].cid, Some(String::from("147")));

        assert_eq!(passports[0].is_valid_part1(), true);
        assert_eq!(passports[1].is_valid_part1(), false);
        assert_eq!(passports[2].is_valid_part1(), true);
        assert_eq!(passports[3].is_valid_part1(), false);
    }
    
    #[test]
    fn verify_part2() {
        let passport_valid = Passport {
            byr: Some(2002),
            iyr: Some(2015),
            eyr: Some(2025),
            hgt: Some(PassportHeight::Inches(65)),
            hcl: Some(String::from("#123abc")),
            ecl: Some(String::from("brn")),
            pid: Some(String::from("000000001")),
            cid: None,
        };
        assert_eq!(passport_valid.is_valid_part2(), true);
        
        let mut passport_tmp;
        
        passport_tmp = passport_valid.clone(); 
        passport_tmp.byr = Some(2003);
        assert_eq!(passport_tmp.is_valid_part2(), false);
        
        passport_tmp = passport_valid.clone(); 
        passport_tmp.hgt = None;
        assert_eq!(passport_tmp.is_valid_part2(), false);
        
        passport_tmp = passport_valid.clone(); 
        passport_tmp.hgt = Some(PassportHeight::Centimeters(149));
        assert_eq!(passport_tmp.is_valid_part2(), false);
        
        passport_tmp = passport_valid.clone(); 
        passport_tmp.hgt = Some(PassportHeight::Centimeters(194));
        assert_eq!(passport_tmp.is_valid_part2(), false);
        
        passport_tmp = passport_valid.clone(); 
        passport_tmp.hgt = Some(PassportHeight::Inches(58));
        assert_eq!(passport_tmp.is_valid_part2(), false);
        
        passport_tmp = passport_valid.clone(); 
        passport_tmp.hgt = Some(PassportHeight::Inches(77));
        assert_eq!(passport_tmp.is_valid_part2(), false);
        
        passport_tmp = passport_valid.clone(); 
        passport_tmp.hcl = Some(String::from("!123abc"));
        assert_eq!(passport_tmp.is_valid_part2(), false);
        
        passport_tmp = passport_valid.clone(); 
        passport_tmp.hcl = Some(String::from("#123ab"));
        assert_eq!(passport_tmp.is_valid_part2(), false);

        passport_tmp = passport_valid.clone(); 
        passport_tmp.hcl = Some(String::from("#123abz"));
        assert_eq!(passport_tmp.is_valid_part2(), false);
        
        passport_tmp = passport_valid.clone(); 
        passport_tmp.ecl = Some(String::from("wat"));
        assert_eq!(passport_tmp.is_valid_part2(), false);
        
        passport_tmp = passport_valid.clone(); 
        passport_tmp.pid = Some(String::from("0123456789"));
        assert_eq!(passport_tmp.is_valid_part2(), false);
    }
}

