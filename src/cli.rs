use std::{ error::Error, fs::File, io::{ BufReader, Write }, path::PathBuf};
use clap::Parser;

#[derive(Parser,Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
 /// Path of the file to read
 #[arg(id="FILE")]
 pub file: PathBuf,

 #[arg(id="OUTPUT")]
 pub output: PathBuf,
}

pub fn get_args () -> Result< Cli , Box<dyn Error>>{
 let cli = Cli::parse();

 Ok(cli)
}
pub fn open_file (filename: PathBuf) -> Result<BufReader<File>,Box<dyn Error>> {
 let file = File::open(filename)?;
 let reader: BufReader<File>;

 reader = BufReader::new(file);

 Ok(reader)
}

pub fn write_file (output: PathBuf, header: (usize, Vec<(char, usize, Vec<bool>)>), encoded_data: Vec<u8>) -> std::io::Result<()> {
 let mut file = File::create(output)?;
 println!("{}", header.0 as u8);
 file.write_all(&[header.0 as u8])?;

 for (char, length, code) in header.1 {
  file.write_all(&[char as u8])?;
  file.write_all(&[length as u8])?;
  for item in code {
   file.write_all(&[item as u8])?;
  }
 }

 file.write_all(&encoded_data)

}

