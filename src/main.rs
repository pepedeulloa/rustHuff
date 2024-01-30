use std::io::Read;
use rust_huff::cli::{Cli, get_args, open_file};
use rust_huff::freq_calc::calculate_frequencies;
fn main() {
    let Cli{file} = get_args().unwrap();

    let mut reader = open_file(file).unwrap();
    let mut file_str = String::new();
    reader.read_to_string(&mut file_str).unwrap();

    let mut sorted_freq: Vec<_> = calculate_frequencies(file_str).unwrap().into_iter().collect();
    sorted_freq.sort_by(|a, b| a.1.cmp(&b.1));

    println!("{:?}", sorted_freq)
}
