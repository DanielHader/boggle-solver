mod boggle;

use boggle::{Board, Dictionary};

use trie_rs::{Trie, TrieBuilder};
use trie_rs::inc_search::{IncSearch, Position, Answer};

#[unsafe(no_mangle)]
pub fn get_dictionary_size() -> usize {

    let trie = Trie::from_iter(["a", "app", "apple", "better", "application"]);

    // let dict_file = env::var("DICTIONARY_FILE").unwrap();
    // let dict = Dictionary::from_file(dict_file).unwrap();

    //println!("dictionary contains \"apple\": {}", dict.contains("apple"));

    return trie.iter().collect::<Vec<String>>().len(); //100000;
}