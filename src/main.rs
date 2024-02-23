mod cli;
use crate::cli::*;

mod huff;
use crate::huff::*;

mod calc;
use crate::calc::*;

mod header;
use crate::header::*;

mod encoder; 
use crate::encoder::*;

mod decoder;
use crate::decoder::*;

use std::io::Read;

fn main() {
	// Parsing cli args
	let Cli{file, output, decode} = get_args().unwrap();

	if !decode {
		// Read the file and parse to a string 
	let mut reader = open_file(&file).unwrap();
	let mut file_str = String::new();

	reader.read_to_string(&mut file_str).unwrap();

	// Calculate the frequencies
	let frequencies: Vec<_> = calculate_frequencies(file_str.clone()).unwrap();
	let mut huff_vector = HuffVector::gen_huff_vector(frequencies);

	// Create the Huffman tree with the frequencies.
	let huff_tree = HuffBTree::gen_tree(&mut huff_vector);

	// Genenate the table with the codes.
	let table = huff_tree.gen_table();

	// Generate the data to write
	let header: (usize, String, Vec<(Vec<u8>, usize, Vec<bool>)>) = get_headers(file.extension().unwrap().to_str().unwrap() ,table.clone());
	let encoded_data = encode(&table, file_str);
	
	// Write the file
	write_encoded_file(&output.unwrap(), header, encoded_data).unwrap();

	} else {
		let mut encoded_reader = open_file(&file).unwrap();

		let (_, extension, table) = parse_headers(&mut encoded_reader);

		println!("{} {:?}", extension, table);
		
		let decoded_text = decode_file(&mut encoded_reader, table);

		write_decoded_file(&output.unwrap().with_extension(extension), decoded_text).unwrap();
	}
}