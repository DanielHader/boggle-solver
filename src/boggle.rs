use std::ops::Index;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

use trie_rs::{Trie, TrieBuilder};
use trie_rs::inc_search::{IncSearch, Position, Answer};

#[inline(always)]
fn rc_index(row: usize, col: usize, cols: usize) -> usize {
    // todo: deal with bounds
    row * cols + col
}

#[inline(always)]
fn index_rc(index: usize, cols: usize) -> (usize, usize) {
    // todo: deal with bounds
    (index / cols, index % cols)
}

#[derive(Debug)]
pub struct Dictionary {
	trie: Trie<u8>,
}

impl Dictionary {

	pub fn from_file<P>(filename: P) -> io::Result<Dictionary>
	where P: AsRef<Path> {
		let mut builder = TrieBuilder::new();
		let file = File::open(filename)?;
		for line in io::BufReader::new(file).lines().map_while(Result::ok) {
			// TODO: validate or cull words here
			builder.push(line.to_lowercase());
		}

		Ok(Dictionary {
			trie: builder.build(),
		})
	}

	pub fn contains(&self, word: &str) -> bool {
		self.trie.exact_match(word)
	}
}

#[derive(Debug)]
struct TraversalState {
    current_index: usize,
    neighbor_offset_index: usize,
	trie_position: Position,
}

#[derive(Debug)]
struct BoardIndex {
	index: usize,
	cols: usize,
}

/// The current state of a traversal through all word sequences in a Board
#[derive(Debug)]
pub struct Traversal<'a> {
    buffer: String,
    stack: Vec<TraversalState>,
    visit_mask: Vec<bool>,
    start_index: usize,

    board: &'a Board,
	dict: &'a Dictionary,
}

impl<'a> Traversal<'a> {

    fn new(board: &'a Board, dict: &'a Dictionary) -> Self {
		Traversal {
			buffer: String::new(),
			stack: Vec::new(),
			visit_mask: vec![false; board.rows * board.cols],
			start_index: 0,
			board: board,
			dict: dict,
		}
    }

	fn push(&mut self, index: usize, trie_position: Position) -> Option<Answer> {

		let cube_str = &*self.board.cubes[index];

		let mut inc_search = IncSearch::resume(&self.dict.trie.0, trie_position.clone());
		let Ok(answer) = inc_search.query_until(cube_str) else { return None; };

		self.stack.push(
			TraversalState {
				current_index: index,
				neighbor_offset_index: 0,
				trie_position: Position::from(inc_search),
			}
		);

		self.buffer += cube_str;
		self.visit_mask[index] = true;

		Some(answer)
	}

	fn pop(&mut self) {
		let Some(stack_top) = self.stack.pop() else { return; };
		let top_index = stack_top.current_index;

		self.buffer.truncate(self.buffer.len() - self.board.cubes[top_index].len());
		self.visit_mask[top_index] = false;
	}

    pub fn next(&mut self) -> Option<String> {

		const NEIGHBOR_OFFSET_TABLE: [(i32, i32); 8] = [
			(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)
		];

		let rows = self.board.rows;
		let cols = self.board.cols;

		loop {
			if self.stack.len() == 0 {
				if self.start_index == rows * cols { return None; }

				let old_index = self.start_index.clone();
				self.start_index += 1;
				match self.push(old_index, Position::from(self.dict.trie.inc_search())) {
					Some(Answer::Match) | Some(Answer::PrefixAndMatch) => {
						return Some(self.buffer.clone());
					},
					Some(Answer::Prefix) | None => {},
				};
			} else {
				let mut nbr_index: Option<usize> = None;
				
				{ // stack_top mutable borrow scope
					let mut stack_top = self.stack.last_mut().unwrap();
					let top_index = stack_top.current_index;
					let top_rc = index_rc(top_index, cols);

					loop {
						let nbr_offset = NEIGHBOR_OFFSET_TABLE[stack_top.neighbor_offset_index];
						stack_top.neighbor_offset_index += 1;
						if stack_top.neighbor_offset_index >= NEIGHBOR_OFFSET_TABLE.len() { break; }

						// todo: check bounds
						let nbr_rc = (
							top_rc.0 as i32 + nbr_offset.0,
							top_rc.1 as i32 + nbr_offset.1,
						);

						// out of bounds, todo: check bounds when converting to i32
						if nbr_rc.0 < 0 || nbr_rc.1 < 0 || nbr_rc.0 >= rows as i32 || nbr_rc.1 >= cols as i32 {
							continue;
						}

						let nbr_index_inner = rc_index(nbr_rc.0 as usize, nbr_rc.1 as usize, cols);
						if !self.visit_mask[nbr_index_inner] {
							nbr_index = Some(nbr_index_inner);
							break;
						}
					};
				}
				
				match nbr_index {
					Some(nbr_index) => { // found a valid neighbor
						let stack_top = self.stack.last().unwrap();

						match self.push(nbr_index, stack_top.trie_position.clone()) {
							Some(Answer::Match) | Some(Answer::PrefixAndMatch) => {
								return Some(self.buffer.clone());
							},
							Some(Answer::Prefix) | None => {},
						};
						// self.push(nbr_index);
						// break;
					},
					None => { // no valid neighbors, pop and try again
						self.pop();
					},
				};
			}
		};
		//Some(self.buffer.clone())
    }
}

#[derive(Debug)]
pub struct Board {
    rows: usize,
    cols: usize,
    cubes: Vec<String>,
}

impl Board {
    pub fn new(rows: usize, cols: usize, cubes: Vec<String>) -> Option<Board> {
		if rows * cols != cubes.len() {
			None
		} else {
			Some(Board {
				rows: rows,
				cols: cols,
				cubes: cubes,
			})
		}
    }
    
    pub fn traversal<'a>(&'a self, dict: &'a Dictionary) -> Traversal<'a> {
		Traversal::new(self, dict)
    }
}

impl Index<(usize, usize)> for Board {
    type Output = String;

    fn index(&self, rc: (usize, usize)) -> &Self::Output {
	&self.cubes[rc_index(rc.0, rc.1, self.cols)]
    }
}
