use std::collections::BTreeMap;

use crate::huff::HuffCode;

pub fn get_headers(table: BTreeMap<char, HuffCode>) -> Vec<(char, String, usize)> {
 let mut headers = Vec::new();

 for (_, code) in table {
  let parsed_code = get_header(code);
  headers.push(parsed_code);
 }
 //println!("{:?}", headers);
 headers
}

pub fn get_header(code: HuffCode) -> (char, String, usize) {
 let (char, code_bool, length) = code.get_header_data();
 let code_string = bool_vec_to_string(code_bool);

 let header = (char, code_string, length);
 //println!("{:?}", header);
 header
}

pub fn bool_vec_to_string (bool_vector: Vec<bool> ) -> String {
 let mut string: String = String::new(); 

 for char in bool_vector.clone().iter() {
  if *char {
   string.push('1')
  } else {
   string.push('0')
  }
 }

 string
}
