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
		let mut freq: BTreeMap<char, usize> = BTreeMap::new();
		let filepath = PathBuf::from_str("/Users/lucascounagobarreiro/Documents/GitHub/rustHuff/docs/lesMiserables.txt");
 	let mut reader = open_file(filepath.unwrap()).unwrap();

 	let mut file_str = String::new();

 	Read::read_to_string(&mut reader, &mut file_str).unwrap();

 	freq = calculate_frequencies(file_str).unwrap();

		Ok(freq)
	}

 #[test]
 fn check_t_x_freq() {
		let freq = run_file().unwrap();		

		assert_eq!(*freq.get(&'t').unwrap(), 223000usize);
		assert_eq!(*freq.get(&'X').unwrap(), 333usize);
 }
}
