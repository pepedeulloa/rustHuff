use std::{error::Error, fs::File, io::{BufReader, Read}};

//use num_cpus::get;

/* pub fn decode_file_mt(reader: &mut BufReader<File>, table: Vec<(char, usize, Vec<bool>)>) {
 let chunks = num_cpus::get();


} */

pub fn decode_file(reader: &mut BufReader<File>, table: Vec<(char, usize, Vec<bool>)>) -> String {
 let mut data_chunks = Vec::new();
 let mut decoded_text = String::new();

 let _ = get_chunks(&mut data_chunks, reader);

 println!("Decodificando");

 println!("{:?}", table);

 for mut chunk in data_chunks {
  println!("NOVO CHUNK");
  
  let chunk_string = decode_chunk(chunk.remove(0), &mut chunk, &table);
  decoded_text.push_str( chunk_string.as_str());
 }
 
 println!("Fin da decodificación...");
 decoded_text
}


fn decode_chunk(padding_bits: u8, chunk: &mut Vec<u8>, table: &Vec<(char, usize, Vec<bool>)>) -> String {
 let mut chunk_text = String::new();
 let mut code = Vec::new();
 let mut decoded = false;
 let len = chunk.len() - 1;

 println!("chunk len: {len}\tpadding bits: {padding_bits}");

 for (index, byte) in chunk.into_iter().enumerate() {
  for i in 0..8u8 {
   if decoded {
   code = Vec::new();
   decoded = false;
  }
   let mask = 1 << i;
   let to_bool = (mask & *byte) > 0;
   //println!("mask: {:08b}\nbyte: {:08b}", mask, byte);
   code.push(to_bool);
   //println!("{:?}", code);
   let is_code = table.iter().find(|element| element.2 == code);

   if len == index && padding_bits == i  {
    println!("FIN, padding {} == {}", padding_bits, i);
    break;
   }
   
   match is_code {
    None => {
     //println!("NON");
     continue
    },
    Some((char, _, _)) => {
     //println!("{} == {} && {} == {}", len, index, padding_bits, i);
     
     chunk_text.push(*char);
     //println!("añadido {}, actual text:{}", char, chunk_text);
     decoded = true;
    },
   }
  }
 }
 println!("CHUNK: {}", chunk_text);
 chunk_text
}


// Read and divide data in chunks, first byte of each chunk is 
// its padding bits at last byte of the chunk
fn get_chunks(data_chunks: &mut Vec<Vec<u8>>, data: &mut BufReader<File>) -> Result<(), Box<dyn Error>>{
 let mut chunks = [0u8;1];
 let _ = data.read_exact(&mut chunks);

 println!("num chunks: {}", chunks[0]);

 for _ in 0..chunks[0] {
  let mut len = [0u8; 8];

  let _ = data.read_exact(&mut len);

  let len = usize::from_be_bytes(len) + 1;
  println!("len of chunk: {len}");
  let mut chunk = vec![0; len];
  
  let _ = data.read_exact( &mut chunk);

  data_chunks.push(chunk);
 }

 Ok(())
}
