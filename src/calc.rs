use std::{collections::BTreeMap, error::Error};
use crate::huff::*;

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