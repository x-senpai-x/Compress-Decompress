//we want package 
//so we added dependencies in cargo.toml
//now we will import it 
extern crate flate2;//provides functionality for Gzip Compression

use flate2::write::GzEncoder;
use flate2::Compression;

use std::env::args;//we want to use command line arguments
use std::fs::File;//file operations
//use std::io::BufReader;//reading files
//use std::io::copy;//copying data betweeen files
use std::io::{BufReader,BufWriter,copy};
use std::time::Instant;//to show time taken to compress


use flate2::read::GzDecoder;

fn main(){
    if args().len() !=4{
        eprint!("Usage: compress-files <compress/decompress> <file-to-compress> <compressed-file-name>");
        return;
    }
    let operation=args().nth(1).unwrap();
    let input_path=args().nth(2).unwrap();
    let output_path=args().nth(3).unwrap();
    if operation=="compress"{
        compress_file(&input_path,&output_path);
    }
    else if operation=="decompress"{
        decompress_file(&input_path,&output_path);
    }
    else{
        eprintln!("Invalid operation");
    }

  /*   
    //check if we have 2 arguments (actually 3 )
    //the program name, the file to compress, and the name of the compressed file
    //args asseses the CLI arguments
    if args().len() != 3{
        eprintln!("Usage: compress-files <file-to-compress> <compressed-file-name>");
        return;
    }
  
    //args().nth(1).unwrap retrieves the first argument and is used as input 
    //File::open opens the file 
    //BufReader::new wraps the file in a buffer
    let mut input=BufReader::new(File::open(args().nth(1).unwrap()).unwrap());

    //File::create creates the output file
    let output=File::create(args().nth(2).unwrap()).unwrap();
    //it is empty now and we will write compressed data to it

    //GzEncoder::new is used to create a new Gzip encoder that will compress data into the output file with default compression settings.
    //nothing has been compressed yet only an instance for Gzencoder has been created and whatever will be compressed will be written to output file
    let mut encoder=GzEncoder::new(output,Compression::default());
    let start=Instant::now();

    //data from input file is copied and provided to encoder
    copy(&mut input,&mut encoder).unwrap();
    //copy(&mut input, &mut encoder).unwrap() reads data from the BufReader and writes it to the GzEncoder. This effectively compresses the data and writes it to the output file in a streaming manner, without loading the entire file into memory.

    //now output file is in encoder
    let output=encoder.finish().unwrap();
    let duration=start.elapsed();
    println!("Time taken to compress: {:?}",duration);
    println!("Original file size: {} bytes",input.get_ref().metadata().unwrap().len());
    println!("Target file size: {} bytes",output.metadata().unwrap().len());
*/

}
fn compress_file(input_path : &str,output_path : &str){

    let mut input=BufReader::new(File::open(input_path).unwrap());
    let output=File::create(output_path).unwrap();
    let mut encoder=GzEncoder::new(output,Compression::default());
    let start=Instant::now();
    copy(&mut input,&mut encoder).unwrap();
    let output=encoder.finish().unwrap();
    let duration=start.elapsed();
    println!("Time taken to compress: {:?}",duration);
    println!("Original file size: {} bytes",input.get_ref().metadata().unwrap().len());
    println!("Target file size: {} bytes",output.metadata().unwrap().len());
}
fn decompress_file(input_path : &str,output_path : &str){
    let mut input=BufReader::new(File::open(input_path).unwrap());
    let output:File=File::create(output_path).unwrap();
    let mut decoder=GzDecoder::new(input);
    let mut output=BufWriter::new(output);
    let start=Instant::now();
    copy(&mut decoder,&mut output).unwrap();
    let duration=start.elapsed();
    println!("Time taken to decompress: {:?}", duration);

    let mut input=BufReader::new(File::open(input_path).unwrap());

    println!("Compressed file size: {} bytes", input.get_ref().metadata().unwrap().len());
    println!("Target file size: {} bytes",output.get_ref().metadata().unwrap().len());

}
