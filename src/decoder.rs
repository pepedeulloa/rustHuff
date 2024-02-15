use std::{fs::File, io::{BufReader, Read}};

pub fn decode(reader: &mut BufReader<File>, table: Vec<(char, usize, Vec<bool>)>) -> String {
 let mut text = String::new();
 let mut decoded_text = String::new();
 let mut code = Vec::new();
 let mut decoded = false;

 let _ = reader.read_to_string(&mut text);

 println!("Decoding...");

 for byte in text.as_bytes() {
  for i in 0..7 {
   if decoded {
    println!("novo char");
   code = Vec::new();
   decoded = false;
  }
   println!("{i}");
   let mask = 1 << i;
   let to_bool = (mask & byte) > 0;
   code.push(to_bool);
   let is_code = table.iter().find(|element| element.2 == code);
   
   match is_code {
    None => {
     println!("code: {:?}", code);
     continue
    },
    Some((char, _, _)) => {
     println!("code: {:?}", code);
     println!("is code for {}", char);
     decoded_text.push(*char);
     decoded = true;
    },
   }
   
  }
 }
 decoded_text
}

