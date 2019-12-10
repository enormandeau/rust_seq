use flate2::read;
use flate2::write;
use flate2::Compression;
use std::error::Error;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::Path;

// Classes

// Functions
/// Read normal or compressed files seamlessly
/// Uses the presence of a `.gz` extension to decide
pub fn reader(filename: &str) -> Box<dyn BufRead> {
    let path = Path::new(filename);
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why.description()),
        Ok(file) => file,
    };

    if path.extension() == Some(OsStr::new("gz")) {
        Box::new(BufReader::with_capacity(
            128 * 1024,
            read::GzDecoder::new(file),
        ))
    } else {
        Box::new(BufReader::with_capacity(128 * 1024, file))
    }
}

/// Write normal or compressed files seamlessly
/// Uses the presence of a `.gz` extension to decide
// Attempting to have a file writer too
pub fn writer(filename: &str) -> Box<dyn Write> {
    let path = Path::new(filename);
    let file = match File::create(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why.description()),
        Ok(file) => file,
    };

    if path.extension() == Some(OsStr::new("gz")) {
        // Error is here: Created file isn't gzip-compressed
        Box::new(BufWriter::with_capacity(
            128 * 1024,
            write::GzEncoder::new(file, Compression::default()),
        ))
    } else {
        Box::new(BufWriter::with_capacity(128 * 1024, file))
    }
}

/// Doing tests
fn main() -> io::Result<()> {
    /*
    // Test with uncompressed file
    let filename = "file.txt";
    println!("Testing reader with uncompressed file: '{}'", filename);
    let reader_file = reader(filename);
    for line in reader_file.lines() {
        println!("{}", line?);
    }
    println!();

    // Test with compressed file
    let filename = "file.txt.gz";
    println!("Testing reader with compressed file: '{}'", filename);
    let reader_file_gz = reader(filename);
    for line in reader_file_gz.lines() {
        println!("{}", line?);
    }
    println!();

    // Test writing to uncompressed file
    let filename = "file.output.txt";
    println!("Testing writer with compressed file: '{}'", filename);
    let mut writer_file = writer(filename);
    for _i in 1..=100 {
        writer_file.write_all(b"This is the end. Count your chickens.\n")?;
    }

    // Test writing to compressed file
    let filename = "file.output.txt.gz";
    println!("Testing writer with compressed file: '{}'", filename);
    let mut writer_file = writer(filename);
    for _i in 1..=100 {
        writer_file.write_all(b"This is the end. Count your chickens.\n")?;
    }
    */

    Ok(())
}
