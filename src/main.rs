mod boggle;

use boggle::Board;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> + std::fmt::Debug, {
    println!("{:?}", filename);
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {

    let dict_file = env::var("DICTIONARY_FILE").unwrap();
    match read_lines(&dict_file) {
        Ok(lines) => {
            let mut i: usize = 0;
            for line in lines.map_while(Result::ok) {
                println!("{}", line);
                
                i += 1;
                if i == 10 { break; }
            }
        },
        Err(_) => {
            println!("unable to load file {}", &dict_file);
        }
    }

    let cubes = vec![
        "E", "G", "T", "E",
        "Qu", "I", "N", "A",
        "A", "P", "E", "S",
        "C", "O", "R", "T",
    ].iter().map(|&s| s.to_owned()).collect();
    let board = Board::new(4, 4, cubes).unwrap();

    println!("Board position (1, 2) is: {}", board[(1, 2)]);

    let cubes_small = vec![
        "A", "B",
    	"C", "D",
    ].iter().map(|&s| s.to_owned()).collect();
    let board_small = Board::new(2, 2, cubes_small).unwrap();

    let mut traversal = board_small.traversal();

    while let Some(seq) = traversal.next() {
        println!("Next sequence in traversal is: {:?}", seq);
    }
}
