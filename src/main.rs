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

use std::time::Instant;

fn main() {
	let start_time = Instant::now();

	// Parsing cli args
	let Cli{file, output} = get_args().unwrap();

	// Read the file and parse to a string 
	let start_read_time = Instant::now();
	let mut reader = open_file(file).unwrap();
	let mut file_str = String::new();

	reader.read_to_string(&mut file_str).unwrap();
	let end_read_time = Instant::now();

	// Calculate the frequencies
	let start_gen_huffcode_time = Instant::now();
	let frequencies: Vec<_> = calculate_frequencies(file_str.clone()).unwrap();
	let mut huff_vector = HuffVector::gen_huff_vector(frequencies);

	// Create the Huffman tree with the frequencies.
	let huff_tree = HuffBTree::gen_tree(&mut huff_vector);

	// Genenate the table with the codes.
	let table = huff_tree.gen_table();
	let end_gen_huffcode_time = Instant::now();

	// Generate the data to write
	let start_encoding_time = Instant::now();
	let header: (usize, Vec<(Vec<u8>, usize, Vec<bool>)>) = get_headers(table.clone());
	let encoded_data = encode(&table, file_str);
	let end_encoding_time = Instant::now();
	
	// Write the file
	let start_write_time = Instant::now();
	write_file(output.clone(), header, encoded_data).unwrap();
	let end_write_time = Instant::now();

	// Benchmarking
	let end_time = Instant::now();
	let duration = end_time - start_time;

	let read_duration = end_read_time - start_read_time;
	let gen_huffcode_duration = end_gen_huffcode_time - start_gen_huffcode_time;
	let encoding_duration = end_encoding_time - start_encoding_time;
	let write_duration = end_write_time - start_write_time;

	println!("Tempo de lectura: {}s {:3}ms {:3}ns", read_duration.as_secs(), read_duration.subsec_millis(), read_duration.subsec_nanos());
	println!("Tempo de xeneración do código: {}s {:3}ms {:3}ns", gen_huffcode_duration.as_secs(), gen_huffcode_duration.subsec_millis(), gen_huffcode_duration.subsec_nanos());
	println!("Tempo de codificacion: {}s {:3}ms {:3}ns", encoding_duration.as_secs(), encoding_duration.subsec_millis(), encoding_duration.subsec_nanos());
	println!("Tempo de escritura: {}s {:3}ms {:3}ns", write_duration.as_secs(), write_duration.subsec_millis(), write_duration.subsec_nanos());
	println!("Tempo transcorrido: {}s {:3}ms {:3}ns", duration.as_secs(), duration.subsec_millis(), duration.subsec_nanos());

	let mut encoded_reader = open_file(output).unwrap();

	parse_headers(&mut encoded_reader);

	let decoded_text = decode(&mut encoded_reader);

}

