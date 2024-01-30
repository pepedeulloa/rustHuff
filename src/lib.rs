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
    use std::io::Error;

	#[derive(Debug)]
	enum HuffNode{
	Branch {
		weight: usize,
		left: Box<HuffNode>,
		right: Box<HuffNode>,
	},
	Leaf {
		char: char,
		weight: usize
	}
}

impl HuffNode {

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

	struct HuffBTree {
		root: Option<Box<HuffNode>>
	}

	impl HuffBTree {
		fn new(min: (char,usize), max: (char,usize)) -> HuffBTree {

			let left = HuffNode::create_leaf(min.0, min.1);
			let right = HuffNode::create_leaf(max.0, max.1);

			let branch = HuffNode::create_branch(Box::new(left), Box::new(right));

			HuffBTree { root: Some(Box::new(branch))}
		}

		fn add_leaf(&mut self, leaf: (char, usize)) {
			let new_leaf = HuffNode::create_leaf(leaf.0, leaf.1);

			if let Some(old_root) = self.root.take() {
				let left:Box<HuffNode>;
				let right:Box<HuffNode>;

				if old_root.get_weight() < new_leaf.get_weight() {
					left = old_root;
					right = Box::new(new_leaf);
				} else {
					left = Box::new(new_leaf);
					right = old_root;
				}

				let new_branch = HuffNode::create_branch(left, right);
				self.root = Some(Box::new(new_branch));
			}
		}

		fn is_lower_eq(&mut self, next: usize) -> bool {
			self.root.as_ref().map_or(false, |root| root.get_weight() <= next)
		}
	}
}

pub mod freq_calc {
 use std::{collections::BTreeMap, error::Error};

 pub fn calculate_frequencies (file: String ) -> Result<BTreeMap<char, usize>, Box<dyn Error>> {
		let mut frequencies_map: BTreeMap<char, usize> = BTreeMap::new();
		for character in file.chars() {
			*frequencies_map.entry(character).or_insert(0) += 1;
		}
		Ok(frequencies_map)
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
