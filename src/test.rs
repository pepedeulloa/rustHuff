mod cli;
mod calc;
#[cfg(test)]
mod tests {
 use std::{collections::BTreeMap, error::Error, io::Read, path::PathBuf, str::FromStr};
 
	use crate::freq_calc::calculate_frequencies;
	
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
