use std::{collections::BTreeMap, sync::Arc, thread};
use num_cpus;

use crate::HuffCode;

pub fn encode_mt (table: Arc<BTreeMap<char, HuffCode>>, data: String) -> Vec<u8>  {
 
 let chunks = num_cpus::get();

 let chunk_size = (data.len() / chunks) + 1;
 
 let mut data_chunks: Vec<String> = Vec::new();
 
 recursive_chunking(&mut data_chunks, &data, chunk_size);
 
 println!("chunk {}", data_chunks[1]);
 
 let mut handles = Vec::new();
 
 let mut thread_id = 0;
 for chunk in data_chunks {
  let table_shared_clone = Arc::clone(&table);
  let handle = thread::spawn(move||{
   encode(&table_shared_clone, chunk, thread_id)
  });
  handles.push(handle);
  thread_id += 1;
 }

 let mut encoded_data = Vec::new();
 
 for t in handles {
  let mut result = t.join().unwrap();
  encoded_data.append(&mut result.1);
 }

 println!("len: {}",encoded_data.len());

 encoded_data

}

fn recursive_chunking(data_chunks: &mut Vec<String>, data: &str, chunk_size: usize){
 let (chunk, data) = data.split_at(chunk_size);
 data_chunks.push(chunk.to_string());
 if data.len() <= chunk_size {
  data_chunks.push(data.to_string());
  return;
 }
 recursive_chunking(data_chunks, data, chunk_size);
}

pub fn encode(table: &BTreeMap<char, HuffCode>, data: String, thread_id: u8) -> (u8, Vec<u8>) {
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
 println!("\nCHUNK: {}", thread_id);
 println!("\nTamaño datos: {} bytes", data.len());
 println!("Tamaño datos comprimidos: {:?} bytes", encoded_data.len());
 
 let data_compression_porcentage: f32 = (1.0 - (encoded_data.len() as f32 / data.len() as f32)) * 100.0; 
 println!("% de compresion: {:.2}%\n", data_compression_porcentage);

 (thread_id, encoded_data)
}