use flate2::read;
use flate2::write;
use flate2::Compression;
use std::error::Error;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::Path;

extern crate bio;
use bio::io::fasta;
use bio::io::fastq;

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
        Box::new(BufWriter::with_capacity(
            128 * 1024,
            write::GzEncoder::new(file, Compression::default()),
        ))
    } else {
        Box::new(BufWriter::with_capacity(128 * 1024, file))
    }
}

// Reading Fasta
pub fn fasta_reader(filename: &str) -> fasta::Reader<Box<dyn io::BufRead>> {
    println!("### Read from Fasta file with Rust-Bio using fasta_reader");
    let infile = reader(filename);
    let sequences = fasta::Reader::new(infile);
    sequences
}

// Reading Fastq
pub fn fastq_reader(filename: &str) -> fastq::Reader<Box<dyn io::BufRead>> {
    println!("### Read from Fasta file with Rust-Bio using fasta_reader");
    let infile = reader(filename);
    let sequences = fastq::Reader::new(infile);
    sequences
}

// Writing Fasta
pub fn fasta_writer(filename: &str) -> fasta::Reader<Box<dyn io::BufRead>> {
    println!("### Read from Fasta file with Rust-Bio using fasta_reader");
    let infile = reader(filename);
    let sequences = fasta::Reader::new(infile);
    sequences
}

// Writing Fastq
pub fn fastq_writer(filename: &str) -> fasta::Reader<Box<dyn io::BufRead>> {
    println!("### Read from Fasta file with Rust-Bio using fasta_reader");
    let infile = reader(filename);
    let sequences = fasta::Reader::new(infile);
    sequences
}

/// Doing tests
fn main() -> io::Result<()> {
    // Rust-Bio
    // Fasta
    // Input
    println!("### Read from Fasta file with Rust-Bio");
    let input_filename = "input.fasta.gz";
    let sequences = fasta_reader(input_filename);

    // Output
    let output_filename = "output_rust-bio.fasta.gz";
    let outfile = writer(output_filename);
    let mut outwriter = fasta::Writer::new(outfile);

    for seq in sequences.records() {
        let s = seq.unwrap().clone();
        println!("{:?}", s.id());
        println!("{:?}", s);
        outwriter.write_record(&s)?;
    }
    println!();

    // Rust-Bio
    // Fastq
    // Input
    println!("### Read from Fastq file with Rust-Bio");
    let input_filename = "input.fastq.gz";
    let sequences = fastq_reader(input_filename);

    // Output
    let output_filename = "output_rust-bio.fastq.gz";
    let outfile = writer(output_filename);
    let mut outwriter = fastq::Writer::new(outfile);

    for seq in sequences.records() {
        let s = seq.unwrap().clone();
        println!("{:?}", s.id());
        println!("{:?}", s);
        outwriter.write_record(&s)?;
    }
    println!();

    Ok(())
}
