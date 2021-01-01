use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Seek};
use std::io;
use std::error::Error;

pub fn run(input_path: &Path) -> bool {
    let map = match parse(input_path) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("There was a problem parsing the input file:");
            eprintln!("{}", e);
            return false;
        }
    };

    let num_trees_3_1 = count_trees_along_direction(&map, 3,1);
    println!("[Part 1] Num trees along direction (3, 1): {}", num_trees_3_1);

    // Part 2
    let num_trees_1_1 = count_trees_along_direction(&map, 1,1);
    let num_trees_5_1 = count_trees_along_direction(&map, 5,1);
    let num_trees_7_1 = count_trees_along_direction(&map, 7,1);
    let num_trees_1_2 = count_trees_along_direction(&map, 1,2);
    
    let product = 
        count_trees_along_direction(&map, 1,1) *
        num_trees_3_1 *
        count_trees_along_direction(&map, 5,1) *
        count_trees_along_direction(&map, 7,1) *
        count_trees_along_direction(&map, 1,2);
    println!("[Part 2] Product of num. trees along slopes: : {}", product);

    return true;
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum TileType {
    Open,
    Tree
}

#[derive(Debug)]
struct TobogganMap {
    rows: usize,
    cols: usize,
    tiles: Vec<TileType>
}
impl TobogganMap {
    fn get_tile(&self, x: usize, y: usize) -> TileType {
        let _x = x % self.cols;
        let _y = y;

        self.tiles[ self.cols * _y + _x]
    }
}

fn parse(filepath: &Path) -> Result<TobogganMap, Box<dyn Error>> {
    let mut file = File::open(filepath).expect("Could not open file");
    let num_lines = BufReader::new(&file).lines().count();
    file.seek(io::SeekFrom::Start(0))?;

    let rows = num_lines;
    let cols;
    let mut tiles: Vec<TileType>;
    {
        
        let line = BufReader::new(&file).lines().nth(0).unwrap()?;
        cols = line.chars().count();
        tiles = vec![TileType::Open; rows*cols];
        file.seek(io::SeekFrom::Start(0))?;
    }

    let reader = BufReader::new(&file);
    for (i_line, line) in reader.lines().enumerate() {
        for (i_c, c) in line?.chars().enumerate() {
            let tile_type = match c {
                '.' => Ok(TileType::Open),
                '#' => Ok(TileType::Tree),
                _ => Err(
                    Box::new(
                        io::Error::new( 
                            io::ErrorKind::Other, 
                            format!("Could not parse row {} col {}: unexpected charachter {}", i_line, i_c, c)
                        )
                    )
                )
            }?;

            tiles[i_line * cols + i_c] = tile_type;
        }
    }

    Ok(TobogganMap {
        rows,
        cols,
        tiles
    })

}

fn count_trees_along_direction(map: &TobogganMap, dx: usize, dy: usize) -> usize { 
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut num_trees = 0;

    while y + dy <= map.rows {
        if map.get_tile(x, y) == TileType::Tree {
            num_trees += 1;
        }

        x += dx;
        y += dy;
    }

    num_trees
}
