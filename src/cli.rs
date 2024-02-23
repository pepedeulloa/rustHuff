use std::{ error::Error, fs::File, io::{ BufReader, Write }, path::*};
use clap::*;

#[derive(Parser,Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {

 /// Path of the file to read
 #[arg(id="FILE")]
 pub file: PathBuf,

 /// Path to the output file. If not specified, the output file will be named as the input file
 #[arg(id="OUTPUT")]
 pub output: Option<PathBuf>,

 /// Decode file
 #[arg(short='d', long="decode", id="Decode")]
 pub decode: bool,
}

pub fn get_args () -> Result< Cli , Box<dyn Error>>{
 let mut cli = Cli::parse();

 if !file_exists(&cli.file) {
  panic!("Error")
 }

 match cli.output {
  None => cli.output = Some(cli.file.clone()),
  Some(_) => (),
 }

 Ok(cli)
}

fn file_exists(filename: &PathBuf) -> bool {
 filename.is_file()
}

pub fn open_file (filename: &PathBuf) -> Result<BufReader<File>,Box<dyn Error>> {
 let file = File::open(&filename).expect("Error. Write a valid filename");
 let reader: BufReader<File>;

 reader = BufReader::new(file);

 Ok(reader)
}

pub fn write_encoded_file (output: &PathBuf, header: (usize, String, Vec<(Vec<u8>, usize, Vec<bool>)>), encoded_data: Vec<u8>) -> std::io::Result<()> {
 let mut file = File::create(&output.with_extension("huff"))?;
 
 file.write_all(&[header.0 as u8])?;
 file.write_all(&header.1.as_bytes())?;

 for (char, length, code) in header.2 {
  file.write_all(&char)?;
  file.write_all(&[length as u8])?;
  for item in code {
   file.write_all(&[item as u8])?;
  }
 }
 file.write_all(&encoded_data)

}

pub fn write_decoded_file (output: &PathBuf, decoded_data: String) -> std::io::Result<()> {
 let mut file = File::create(&output)?;

 file.write_all(&decoded_data.as_bytes())
}
