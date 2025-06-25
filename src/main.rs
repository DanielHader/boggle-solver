mod boggle;

use boggle::Board;

use std::env;

fn main() {

    let dict_file = env::var("DICTIONARY_FILE").unwrap();
    println!("{:?}", dict_file);

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
