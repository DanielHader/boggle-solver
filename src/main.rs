mod boggle;

use boggle::{Board, Dictionary};

use std::env;

fn main() {

    let dict_file = env::var("DICTIONARY_FILE").unwrap();
    let dict = Dictionary::from_file(dict_file).unwrap();

    println!("dictionary contains \"apple\": {}", dict.contains("apple"));

    let cubes = vec![
        "e", "g", "t", "e",
        "qu", "i", "n", "a",
        "a", "p", "e", "s",
        "c", "o", "r", "t",
    ].iter().map(|&s| s.to_owned()).collect();
    let board = Board::new(4, 4, cubes).unwrap();

    println!("Board position (1, 2) is: {}", board[(1, 2)]);

    let cubes_small = vec![
        "b", "s",
    	"t", "a",
    ].iter().map(|&s| s.to_owned()).collect();
    let board_small = Board::new(2, 2, cubes_small).unwrap();

    let mut traversal = board.traversal(&dict);

    while let Some(seq) = traversal.next() {
        println!("Next sequence in traversal is: {:?}", seq);
    }
}
