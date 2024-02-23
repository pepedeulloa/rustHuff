use std::{fs::File, io::{BufReader, Read}};

pub fn decode_file(reader: &mut BufReader<File>, table: Vec<(char, usize, Vec<bool>)>) -> String {
 let mut text = String::new();
 let mut decoded_text = String::new();
 let mut code = Vec::new();
 let mut decoded = false;

 let _ = reader.read_to_string(&mut text);

 println!("Decodificando...");

 for byte in text.as_bytes() {
  for i in 0..7 {
   if decoded {
   code = Vec::new();
   decoded = false;
  }
   let mask = 1 << i;
   let to_bool = (mask & byte) > 0;
   code.push(to_bool);
   let is_code = table.iter().find(|element| element.2 == code);

   match is_code {
    None => {
     continue
    },
    Some((char, _, _)) => {
     decoded_text.push(*char);
     decoded = true;
    },
   }
   
  }
 }
 println!("Fin da decodificación...");
 decoded_text
}

