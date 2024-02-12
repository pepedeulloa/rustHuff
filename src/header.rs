use std::{collections::BTreeMap, fs::File, io::{BufReader, Read}};

use crate::huff::HuffCode;

pub fn get_headers(table: BTreeMap<char, HuffCode>) -> (usize,Vec<(char, usize, Vec<bool>)>) {
 let mut headers = Vec::new();

 for (_, code) in table {
  let parsed_code = get_header(code);
  headers.push(parsed_code);
 }

 println!("elementos na cabeceira: {:?}", headers.len());
 (headers.len(),headers)
}

pub fn get_header(code: HuffCode) -> (char, usize, Vec<bool>) {
 let (char, length, code_bool) = code.get_header_data();
 //let code_string = bool_vec_to_string(code_bool);

 let header = (char, length, code_bool);
 println!("{:?}", header);
 header
}

pub fn parse_headers(mut reader: BufReader<File>) /* -> (usize, Vec<(char, Vec<bool>, usize)>) */ {
 let mut table: (usize, Vec<(char, usize, Vec<bool>)>)  = (0, Vec::new());
 let mut u8_vector: Vec<u8> = Vec::new();
 let mut header_length = [0u8; 1];
 let mut char = [0u8;1];
 let mut code_length = [0u8;1];
 let mut code_element = [0u8;1];
 let mut result; 

 result = reader.read_exact(&mut header_length);

 println!("elementos na cabeceira: {}",header_length[0]);

 table.0 = header_length[0] as usize;

 let mut count = header_length[0];

 while count > 0 {
  u8_vector.clear();
  result = reader.read_exact(&mut char);
  result = reader.read_exact(&mut code_length);

  for _ in 0..code_length[0] {
   result = reader.read_exact(&mut code_element);
   u8_vector.push(code_element[0]);
  }

  let code_bool_vector = parse_code(u8_vector.clone());

  table.1.push((char[0] as char, code_length[0] as usize, code_bool_vector));

  count -= 1;
 }

 for item in table.1 {
  println!("{:?}", item)
 }
}

pub fn parse_code(code_to_parse: Vec<u8>) -> Vec<bool> {
 let mut code = Vec::new();

 for byte in code_to_parse {
  if byte == 1 {
   code.push(true);
  } else if byte == 0 {
   code.push(false);
  } else {
   println!("error, valor: {}", byte);
  }
 }

 code
}

