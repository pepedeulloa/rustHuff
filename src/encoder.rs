use std::collections::BTreeMap;

use crate::HuffCode;
//use crate::header::bool_vec_to_string;

pub fn encode(table: &BTreeMap<char,HuffCode> ,data: String) -> Vec<u8> {
 let mut encoded_data = Vec::new();
 
 let mut byte: u8 = 0;
 let mut index: u8 = 0;

 println!("Codificando...");
 for char in data.chars() {
  let code = table.get(&char).unwrap().get_code();

  for bool in code {
   if index == 7 {

    encoded_data.push(byte);

    byte = 0;
    index = 0;
   }
   if bool {
    byte |= 1 << index;
    index += 1;
   } else {
    byte |= 0 << index;
    index += 1;
   }
  }
 }
 println!("Fin codificacion");

 println!("\nTamaño datos: {} bytes", data.len());
 println!("Tamaño datos comprimidos: {:?} bytes", encoded_data.len());
 
 let data_compression_porcentage: f32 = (1.0 - (encoded_data.len() as f32 / data.len() as f32)) * 100.0; 
 println!("% de compresion: {:.2}%\n", data_compression_porcentage);

 encoded_data
}