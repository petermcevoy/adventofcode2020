use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};

pub fn run(input_path: &Path) -> bool {
    let file = File::open(input_path).expect("Could not open file");
    let reader = BufReader::new(&file);

    let lines = reader.lines();
    let mut decoded_passes: Vec<BoardingPassDecoded> = lines
        .map(|l| BoardingPass::new(&l.unwrap()).decode())
        .collect();

    let max_id = decoded_passes.iter().map(|p| p.seat_id).max().unwrap();
    println!("Part 1: Highest seat id: {}", max_id);

    // Part 2
    // Missing boarding passes for rows at very front and back.
    // My boarding pass is the only missing one.

    // Find missing bording pass with non-missing bording passes around it.
    decoded_passes.sort_unstable_by(|a, b| a.seat_id.partial_cmp(&b.seat_id).unwrap());
    let mut missing_seat_id = 0;
    for (i, pass) in decoded_passes.iter().enumerate() {
        if i > 0 {
            let prev_seat_id = decoded_passes[i-1].seat_id;
            let curr_seat_id = pass.seat_id;

            if curr_seat_id - prev_seat_id != 1 {
                missing_seat_id = prev_seat_id + 1;
                break;
            }
        }
    }
    println!("Part 2: Missing seat id: {}", missing_seat_id);

    true
}

#[derive(Debug, PartialEq)]
struct BoardingPass {
    passtr: [char; 10]
}
impl BoardingPass {
    fn new(s: &str) -> BoardingPass { 
        let mut pass = BoardingPass{passtr: ['\0'; 10]};
        for (dest, src) in pass.passtr.iter_mut().zip(s.chars()) {
            *dest = src
        }
        pass
    }
    fn decode(&self) -> BoardingPassDecoded { 
        let mut rows = 0..128;
        let mut cols = 0..8;
        let rowstr = &self.passtr[0..7];
        let colstr = &self.passtr[7..10];
        for c in rowstr.iter() {
            match c {
                // Upper half
                'B' => { rows.start += (rows.end - rows.start) / 2; },
                // Lower half
                'F' => { rows.end -= (rows.end - rows.start) / 2; },
                _ => panic!("Unexpected character")
            }
        }
        for c in colstr.iter() {
            match c {
                // Upper half
                'R' => { cols.start += (cols.end - cols.start) / 2; },
                // Lower half
                'L' => { cols.end -= (cols.end - cols.start) / 2; },
                _ => panic!("Unexpected character")
            }
        }
        assert_eq!(rows.end - rows.start, 1);
        assert_eq!(cols.end - cols.start, 1);
        BoardingPassDecoded {
            column: cols.start,
            row: rows.start,
            seat_id: rows.start * 8 + cols.start
        }
    }
}

#[derive(Debug, PartialEq)]
struct BoardingPassDecoded {
    column: usize,
    row: usize,
    seat_id: usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(BoardingPass::new("FBFBBFFRLR").decode(),
            BoardingPassDecoded{column: 5, row: 44, seat_id: 357 });
        assert_eq!(BoardingPass::new("BFFFBBFRRR").decode(),
            BoardingPassDecoded{column: 7, row: 70, seat_id: 567 });
        assert_eq!(BoardingPass::new("FFFBBBFRRR").decode(),
            BoardingPassDecoded{column: 7, row: 14, seat_id: 119 });
        assert_eq!(BoardingPass::new("BBFFBBFRLL").decode(),
            BoardingPassDecoded{column: 4, row: 102, seat_id: 820 });
    }
}

