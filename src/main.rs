use std::io::Read;
use std::time::Instant;

use rust_huff::{cli::{Cli, get_args, open_file},freq_calc::{calculate_frequencies, sort_freq_in_huff_vector}, huff_btree::{HuffBTree, HuffBTreeMethods}};

fn main() {
    let start_time = Instant::now();

    let Cli{file} = get_args().unwrap();

    let mut reader = open_file(file).unwrap();
    let mut file_str = String::new();
    reader.read_to_string(&mut file_str).unwrap();
    println!("{}", file_str.len());
    let sorted_freq: Vec<_> = calculate_frequencies(file_str).unwrap().into_iter().collect();
    let mut sorted_vect = sort_freq_in_huff_vector(sorted_freq);

    let huff_tree = HuffBTree::run(&mut sorted_vect);

    let mut table = huff_tree.gen_table();
    table.sort_by(|a, b| b.1.cmp(&a.1));
    println!("char\t\t|\tfreq\t|\tcode\t\t\t\t|\tbits");
    for (char, freq, code, bits) in table {
        println!("{}\t\t|\t{}\t|\t{}\t\t\t\t|\t{}\n", char.escape_unicode(), freq, code, bits);
    }
    let end_time = Instant::now();
    let duration = end_time - start_time;

    println!("Tiempo transcurrido: {}s {}ms", duration.as_secs(), duration.subsec_millis());

}
