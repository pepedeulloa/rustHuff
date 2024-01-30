pub mod cli {
 use std::{path::PathBuf, io::BufReader, error::Error, fs::File};
 use clap::Parser;

 #[derive(Parser,Debug)]
 #[command(author, version, about, long_about = None)]
 pub struct Cli {
  /// Path of the file to read
  #[arg(id="FILE")]
  pub file: PathBuf,
 }

 pub fn get_args () -> Result< Cli , Box<dyn Error>>{
  let cli = Cli::parse();

  Ok(cli)
 }
 pub fn open_file (filename: PathBuf) -> Result<BufReader<File>,Box<dyn Error>> {
  let file = File::open(filename)?;
  let reader: BufReader<File>;

  reader = BufReader::new(file);
  Ok(reader)
 }
}

pub mod huff_btree {
	#[derive(Debug)]
	pub enum HuffNode{
		Branch {
			weight: usize,
			left: Box<HuffNode>,
			right: Box<HuffNode>,
		},
		Leaf {
			char: char,
			weight: usize,
		}
	}

trait HuffMethods {

	fn get_weight(&self) -> usize;

	fn create_branch(left: Box<HuffNode>, right: Box<HuffNode>) -> HuffNode;

	fn create_leaf(char: char, weight: usize) -> HuffNode;

}

impl HuffMethods for HuffNode {

	fn get_weight(&self) -> usize {
		match *self {
			HuffNode::Branch { weight, .. }=> return weight,
			HuffNode::Leaf { weight, .. }=> return weight,
		}
	}

	fn create_branch(left: Box<HuffNode>, right: Box<HuffNode>) -> HuffNode {
		let weight = left.get_weight() + right.get_weight();
		HuffNode::Branch { weight, left: Box::new(*left), right: Box::new(*right) }
	}

	fn create_leaf(char: char, weight: usize) -> HuffNode {

		HuffNode::Leaf { char, weight }
	}

}

#[derive(Debug)]
	pub struct HuffBTree {
		root: Option<Box<HuffNode>>
	}

	pub	trait HuffBTreeMethods {
			fn new(root: Box<HuffNode>) -> Self;

			fn run(frequencies: &mut HuffVector) -> HuffBTree;

			fn gen_table(&self) -> Vec<(char,usize,String,usize)>;
	}

	impl HuffBTreeMethods for HuffBTree {

		fn new (root: Box<HuffNode>) -> Self {
			HuffBTree {
				root: Some(root)
			}
		}
		fn run(v:&mut HuffVector) -> HuffBTree {
			while v.frequencies.len() > 1 {
				let left = v.frequencies.remove(0);
				let right = v.frequencies.remove(0);

				let new_branch = HuffNode::create_branch(left, right);
				v.insert_node(Box::new(new_branch));
				
				/* println!("frequencies:");
				for f in v.frequencies.iter() {
					println!("{:?}",f);
				} */
			}

			HuffBTree::new(v.frequencies.remove(0))
		}

		fn gen_table(&self) -> Vec<(char,usize,String,usize)> {

			let mut huffman_table = Vec::new();
			let mut code = String::new();

			fn traverse(node: &HuffNode, code: &mut String, huffman_table: &mut Vec<(char, usize, String, usize)>) {
				match node {
					HuffNode::Branch {  left, right , ..} => {
						code.push('0');
						traverse(&left, code, huffman_table);
						code.pop();

						code.push('1');
						traverse(&right, code, huffman_table);
						code.pop();
					},
					HuffNode::Leaf { char, weight } => {
						huffman_table.push((*char, *weight, code.clone(), code.len()));
					}
				}
			}
			
			if let Some(root) = &self.root {
				traverse(root, &mut code, &mut huffman_table);
			}
			huffman_table
		}
	}

	#[derive(Debug)]
	pub struct HuffVector {
		frequencies: Vec<Box<HuffNode>>
	}

	pub trait HuffVectorMethods {
		fn new() -> Self;
		
		fn insert(&mut self, freq: (char, usize));

		fn insert_node(&mut self, node: Box<HuffNode>);
	}

impl HuffVectorMethods for HuffVector {
		fn new() -> Self {
			HuffVector { 
				frequencies: Vec::new()
			}
		}
		
		fn insert(&mut self, freq: (char, usize)) {

			let new_leaf = HuffNode::create_leaf(freq.0, freq.1);
			let mut new_leaf_index = None;

			for (index,freq) in self.frequencies.iter().enumerate() {
				if new_leaf.get_weight() < freq.get_weight() {
					new_leaf_index = Some(index);
					break;
				}
			}

			match new_leaf_index {
				Some(index) => self.frequencies.insert(index, Box::new(new_leaf)),
				None => self.frequencies.push(Box::new(new_leaf))
			}
		}

		fn insert_node(&mut self, node: Box<HuffNode>){
			let mut new_leaf_index = None;

			for (index,freq) in self.frequencies.iter().enumerate() {
				if node.get_weight() < freq.get_weight() {
					new_leaf_index = Some(index);
					break;
				}
			}

			match new_leaf_index {
				Some(index) => self.frequencies.insert(index, Box::new(*node)),
				None => self.frequencies.push(Box::new(*node))
			}
		}

	}
	
}

pub mod freq_calc {
 use std::{collections::BTreeMap, error::Error};
	use crate::huff_btree::*;

 pub fn calculate_frequencies (file: String ) -> Result<BTreeMap<char,usize>, Box<dyn Error>> {
		let mut frequencies_map: BTreeMap<char, usize> = BTreeMap::new();
		for character in file.chars() {
			*frequencies_map.entry(character).or_insert(0) += 1;
		}
		Ok(frequencies_map)
 }

	pub fn sort_freq_in_huff_vector(vector: Vec<(char,usize)>) -> HuffVector {
		let mut huff_vector = HuffVector::new();
		for tuple in vector {
			huff_vector.insert(tuple);
		}
		huff_vector
	}
}

#[cfg(test)]
mod tests {
 use std::{collections::BTreeMap, error::Error, io::Read, path::PathBuf, str::FromStr};
 use crate::{cli::open_file, freq_calc::calculate_frequencies};
	
	fn run_file() -> Result<BTreeMap<char,usize>,Box<dyn Error>> {
		let filepath = PathBuf::from_str("/Users/lucascounagobarreiro/Documents/GitHub/rustHuff/docs/lesMiserables.txt");
 	let mut reader = open_file(filepath.unwrap()).unwrap();

 	let mut file_str = String::new();

 	Read::read_to_string(&mut reader, &mut file_str).unwrap();

		Ok(calculate_frequencies(file_str).unwrap())
	}

 #[test]
 fn check_t_x_freq() {
		let freq = run_file().unwrap();		

		assert_eq!(*freq.get(&'t').unwrap(), 223000usize);
		assert_eq!(*freq.get(&'X').unwrap(), 333usize);
 }
}
