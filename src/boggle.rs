use std::ops::Index;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

use trie_rs::{Trie, TrieBuilder};

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
}

/// The current state of a traversal through all word sequences in a Board
#[derive(Debug)]
pub struct Traversal<'a> {
    buffer: String,
    stack: Vec<TraversalState>,
    visit_mask: Vec<bool>,
    start_index: usize,

    board: &'a Board,
}

impl<'a> Traversal<'a> {

    fn new(board: &'a Board) -> Self {
		Traversal {
			buffer: String::new(),
			stack: Vec::new(),
			visit_mask: vec![false; board.rows * board.cols],
			start_index: 0,
			board: board
		}
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

				self.stack.push(
					TraversalState {
						current_index: self.start_index,
						neighbor_offset_index: 0,
					}
				);

				self.buffer += &*self.board.cubes[self.start_index];
				self.visit_mask[self.start_index] = true;
				
				self.start_index += 1;
				break;
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

						// out of bounds, todo: check biunds when converting to i32
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
						self.stack.push(
							TraversalState {
								current_index: nbr_index,
								neighbor_offset_index: 0,
							}
						);

						self.buffer += &*self.board.cubes[nbr_index];
						self.visit_mask[nbr_index] = true;
						break;
					},
					None => { // no valid neighbors, pop and try again
						let stack_top = self.stack.pop().unwrap();
						let top_index = stack_top.current_index;

						self.buffer.truncate(self.buffer.len() - self.board.cubes[top_index].len());
						self.visit_mask[top_index] = false;
					},
				};
			}
		}
		Some(self.buffer.clone())
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
    
    pub fn traversal(&self) -> Traversal {
	Traversal::new(self)
    }
}

impl Index<(usize, usize)> for Board {
    type Output = String;

    fn index(&self, rc: (usize, usize)) -> &Self::Output {
	&self.cubes[rc_index(rc.0, rc.1, self.cols)]
    }
}
