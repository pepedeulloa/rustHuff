use std::{fs::File, io::{BufReader, Read}};

pub fn decode(reader: &mut BufReader<File>) -> String {
 let mut text = String::new();

 reader.read_to_string(&mut text);

 for byte in text.as_bytes() {

  print!("{:08b}", byte);

 }

 text
}

