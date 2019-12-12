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
//use bio::io::fastq;

/// Fasta sequence
#[derive(Clone, Hash, Debug)]
struct Fasta {
    name: String,
    sequence: String,
}

impl std::fmt::Display for Fasta {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.sequence.len() > 30 {
            write!(f, "{} {}", &self.name, &self.sequence[0..31])
        } else {
            write!(f, "{} {}", &self.name, &self.sequence)
        }
    }
}

impl Fasta {
    pub fn write_to_file(&self, mut output_file: Box<dyn Write>) {
        output_file.write_all(&self.name.as_bytes()).unwrap();
        output_file.write_all("\n".as_bytes()).unwrap();
        output_file.write_all(&self.sequence.as_bytes()).unwrap();
        output_file.write_all("\n".as_bytes()).unwrap();
    }
}

/// Fastq sequence
#[derive(Clone, Hash, Debug)]
struct Fastq {
    name: String,
    sequence: String,
    name2: String,
    quality: String,
}

impl std::fmt::Display for Fastq {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            &self.name,
            &self.sequence[0..31],
            &self.quality[0..31]
        )
    }
}

impl Fastq {
    pub fn write_to_file(&self, mut output_file: Box<dyn Write>) {
        output_file.write_all(&self.name.as_bytes()).unwrap();
        output_file.write_all("\n".as_bytes()).unwrap();
        output_file.write_all(&self.sequence.as_bytes()).unwrap();
        output_file.write_all("\n".as_bytes()).unwrap();
        output_file.write_all(&self.name2.as_bytes()).unwrap();
        output_file.write_all("\n".as_bytes()).unwrap();
        output_file.write_all(&self.quality.as_bytes()).unwrap();
        output_file.write_all("\n".as_bytes()).unwrap();
    }
}


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
    // Test Fasta
    let fasta = Fasta {
        name: ">sequence_1".to_string(),
        sequence: "ACTG".repeat(10).to_string(),
    };
    println!("Fasta sequence: {}", fasta);

    let filename = "output.fasta";
    let outfasta = writer(filename);
    let outfastagz = writer(&(filename.to_owned() + ".gz"));
    fasta.write_to_file(outfasta);
    fasta.write_to_file(outfastagz);

    // Test Fastq
    let fastq = Fastq {
        name: "@sequence_1".to_string(),
        sequence: "ACTG".repeat(10).to_string(),
        name2: "+".to_string(),
        quality: "!ABC".repeat(10).to_string(),
    };
    println!("Fastq sequence: {}", fastq);

    let filename = "output.fastq";
    let outfastq = writer(filename);
    let outfastqgz = writer(&(filename.to_owned() + ".gz"));
    fastq.write_to_file(outfastq);
    fastq.write_to_file(outfastqgz);

    // Testing Rust-Bio
    let input_filename = "test.fasta.gz";
    let output_filename = "test.out.fasta.gz";
    let infile = reader(input_filename);
    let outfile = writer(input_filename);

    let sequences = fasta::Reader::new(infile);

    for seq in sequences.records() {
        println!("{:?}", seq.unwrap());
        //println!("{}", seq.unwrap().id());
    }

    Ok(())
}
