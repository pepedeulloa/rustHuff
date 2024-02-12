use std::{collections::BTreeMap, error::Error};

pub fn calculate_frequencies (file: String ) -> Result<Vec<(char,usize)>, Box<dyn Error>> {
	let mut frequencies_map: BTreeMap<char, usize> = BTreeMap::new();
	for character in file.chars() {
		*frequencies_map.entry(character).or_insert(0) += 1;
	}

	Ok(frequencies_map.into_iter().collect())
}
