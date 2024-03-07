use std::{collections::BTreeMap, sync::Arc, thread};
use num_cpus;

use crate::HuffCode;

pub fn encode_mt (table: Arc<BTreeMap<char, HuffCode>>, data: String) -> Vec<u8>  {
 
 let chunks = num_cpus::get();

 let chunk_size = (data.len() / chunks) + 1;
 
 let mut data_chunks: Vec<String> = Vec::new();
 
 recursive_chunking(&mut data_chunks, &data, chunk_size);
 
 //println!("chunk {}", data_chunks[1]);
 
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
 
 /* 
  header of chunk, [ len | padding_index ] | data
  
  len: length of the data in the chunk.
  padding_index: index where begins the padding bits at last byte in the chunk.  
 */
 encoded_data.push(chunks as u8);
 for t in handles {
  let mut result = t.join().unwrap();
  encoded_data.append(&mut result.1);
  encoded_data.push(result.2);
  encoded_data.append(&mut result.3);
 }

 // println!("encoded data: {:?}",encoded_data);

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

pub fn encode(table: &BTreeMap<char, HuffCode>, data: String, thread_id: u8) -> (u8, Vec<u8>, u8, Vec<u8>) {
 let mut encoded_data = Vec::new();
 
 let mut encoder_buffer: u8 = 0;
 let mut index: u8 = 0;
 println!("#{thread_id} chunk: {data}");
 //println!("#{} Codificando...",thread_id);
 for char in data.chars() {
  let code = table.get(&char).unwrap().get_code();

  for bool in code {
   if index == 8 {
    //println!("push -> {:08b}",encoder_buffer);
    encoded_data.push(encoder_buffer);
    encoder_buffer = 0;
    index = 0;
   }
   //println!("#{} index: {} mask:{:08b}, byte: {:08b}, value: {}", thread_id, index, 1 << index, encoder_buffer, bool);
   if bool {
    encoder_buffer |= 1 << index;
    index += 1;
   } else {
    encoder_buffer |= 0 << index;
    index += 1;
   }
  }
 }
 
 if index != 8 {
  encoded_data.push(encoder_buffer);
 }

 //println!("\t\tultimo byte de #{}: {:08b} == {:08b}. Remata no indice: {}", thread_id, encoder_buffer, encoded_data.last().unwrap(), index);
 println!("Fin codificacion");
 println!("\nCHUNK: {}, Tamaño datos comprimidos: {:?} bytes", thread_id, encoded_data.len());
 
 //et data_compression_porcentage: f32 = (1.0 - (encoded_data.len() as f32 / data.len() as f32)) * 100.0; 
 //println!("% de compresion: {:.2}%\n", data_compression_porcentage);

 let len = encoded_data.len().to_be_bytes().into_iter().collect();

 (thread_id, len, index, encoded_data)
}