use std::{collections::BTreeMap, io::Read};
use rust_huff::cli::{Cli, get_args, open_file};
use rust_huff::freq_calc::calculate_frequencies;
fn main() {
    let Cli{file} = get_args().unwrap();
    let mut freq: BTreeMap<char,usize> = BTreeMap::new();

    let mut reader = open_file(file).unwrap();

    let mut file_str = String::new();

    reader.read_to_string(&mut file_str).unwrap();
    freq = calculate_frequencies(file_str).unwrap();

    let mut sorted_freq: Vec<_> = freq.into_iter().collect();
    sorted_freq.sort_by(|a, b| b.1.cmp(&a.1));

    println!("{:?}", sorted_freq)
}
