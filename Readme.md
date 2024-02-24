# rust_huff

`rust_huff` is a text file compressor implemented using the Huffman coding algorithm. The primary goal of this project is to learn and apply various programming techniques, including file handling, concurrency, data structures, and algorithms.

## Dependencies

This program has been developed using Rust version 1.74.0. Command-line argument management has been simplified thanks to the use of the crate clap in its version 4.4.18. `clap` offers a declarative interface for defining and parsing program arguments, making it easy to create a friendly and efficient user experience. 

## Project Structure

 - **main.rs**: Main entry point of the program.
 - **calc.rs**: Module containing functions related to calculating character frequencies within a file.
 - **cli.rs**: Module responsible for parsing command-line arguments.
 - **decoder.rs**: Module for decoding compressed files.
 - **encoder.rs**: Module for encoding files.
 - **header.rs**: Module for the funtions related to create and parse headers.
 - **huff.rs**: Module containing functions and data structures for the Huffman algorithm.
 - **test.rs**: Module for testing program functionalities.

## Options

- `-d, --decode`: Decode functionality.

## Compilation

```bash
# Example of compilation, executable file will be found in the target/release directory.
cargo build --release && cd target/release
```

## Usage

To use rust_huff, you can compile the program and run it from the command line.

Encoding:

```bash
# Example usage for encoding a file
.rust_huff -d file1.huff file2.txt
```

Decoding: 

```bash
# Example usage for encoding a file
./rust_huff -d file1.huff file2.txt
```
