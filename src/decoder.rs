use std::{error::Error, fs::File, io::{BufReader, Read}, sync::Arc, thread, time::Instant};

//use num_cpus::get;

/* pub fn decode_file_mt(reader: &mut BufReader<File>, table: Vec<(char, usize, Vec<bool>)>) {
 let chunks = num_cpus::get();


} */

pub fn decode_file(reader: &mut BufReader<File>, table: Arc<Vec<(char, usize, Vec<bool>)>>) -> String {
 let mut data_chunks = Vec::new();
 let mut decoded_text = String::new();

 let start_chunking_time = Instant::now();
 let _ = get_chunks(&mut data_chunks, reader);
 let end_chunking_time = Instant::now();
 let chunking_time = end_chunking_time - start_chunking_time;
 println!("chunking_time: {}ms", chunking_time.as_millis());

 let mut handles = Vec::new();

 for mut chunk in data_chunks {
  let table_shared_clone = Arc::clone(&table);
  let handle = thread::spawn(move||{
   decode_chunk(chunk.remove(0), &mut chunk, &table_shared_clone)
  });
  handles.push(handle);
 }
 
 for t in handles {
  let result = t.join().unwrap();
  decoded_text.push_str(result.as_str())
 }

 decoded_text
}


fn decode_chunk(padding_bits: u8, chunk: &mut Vec<u8>, table: &Vec<(char, usize, Vec<bool>)>) -> String {
 let mut chunk_text = String::new();
 let mut code = Vec::new();
 let mut decoded = false;
 let len = chunk.len() - 1; 
 let padding_bits = padding_bits;

 for (index, byte) in chunk.into_iter().enumerate() {
  for i in 0..8u8 {
   if decoded {
   code = Vec::new();
   decoded = false;
  }
   let mask = 1 << i;
   let to_bool = (mask & *byte) > 0;
   code.push(to_bool);
   let is_code = table.iter().find(|element| element.2 == code);
   if len == index && padding_bits == i  {
    break;
   }
   match is_code {
    None => {
     continue
    },
    Some((char, _, _)) => {
     chunk_text.push(*char);
     decoded = true;
    },
   }
  }
 }
 chunk_text
}


// Read and divide data in chunks, first byte of each chunk is 
// its padding bits at last byte of the chunk
fn get_chunks(data_chunks: &mut Vec<Vec<u8>>, data: &mut BufReader<File>) -> Result<(), Box<dyn Error>>{
 let mut chunks = [0u8;1];
 let _ = data.read_exact(&mut chunks);

 for _ in 0..chunks[0] {
  let mut len = [0u8; 8];

  let _ = data.read_exact(&mut len);

  let len = usize::from_be_bytes(len) + 1;
  let mut chunk = vec![0; len];
  
  let _ = data.read_exact( &mut chunk);

  data_chunks.push(chunk);
 }

 Ok(())
}
