use std::{collections::BTreeMap, fs::File, io::{BufReader, Read}};

use crate::huff::HuffCode;

pub fn get_headers(table: BTreeMap<char, HuffCode>) -> (usize,Vec<(Vec<u8>, usize, Vec<bool>)>) {
 let mut headers = Vec::new();

 for (_, code) in table {
  let parsed_code = get_header(code);
  headers.push(parsed_code);
 }

 println!("elementos na cabeceira: {:?}", headers.len());
 (headers.len(),headers)
}

pub fn get_header(code: HuffCode) -> (Vec<u8>, usize, Vec<bool>) {
 let (char, length, code_bool) = code.get_header_data();

 let mut char_u8 = Vec::with_capacity(4);

 let char_u32 = char as u32;

 for byte in char_u32.to_be_bytes() {
  char_u8.push(byte)
 }

 let header = (char_u8, length, code_bool);
 println!("{:?}", (char, length));
 header
}

pub fn parse_headers(reader: &mut BufReader<File>) -> (usize, Vec<(char, usize, Vec<bool>)>) {
 let mut table: (usize, Vec<(char, usize, Vec<bool>)>)  = (0, Vec::new());
 let mut u8_vector: Vec<u8> = Vec::new();
 let mut header_length = [0u8; 1];
 let mut char_buff = [0u8; 4];
 let mut code_length = [0u8; 1];
 let mut code_element = [0u8; 1];

 let _ = reader.read_exact(&mut header_length);

 table.0 = header_length[0] as usize;

 let mut count = header_length[0];

 while count > 0 {
  u8_vector.clear();
  let _ = reader.read_exact(&mut char_buff);
  let _ = reader.read_exact(&mut code_length);

  for _ in 0..code_length[0] {
   let _ = reader.read_exact(&mut code_element);
   u8_vector.push(code_element[0]);
  }

  let code_bool_vector = parse_code(u8_vector.clone());

  
  table.1.push((parse_char(char_buff) , code_length[0] as usize, code_bool_vector));

  count -= 1;
 }

 println!("{:?}", table.1);

 table
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

pub fn parse_char(vec: [u8;4]) -> char {

 let char_u32: u32 = u32::from_be_bytes(vec).try_into().unwrap();

 std::char::from_u32(char_u32).unwrap()

}