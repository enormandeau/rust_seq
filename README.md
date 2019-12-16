# Rust-Seq

Provide fasta/fastq read/write from standard/compressed files

## Example

```Rust
use std::env;

extern crate rust_seq;
use rust_seq::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Parse user input
    let input_filename = &args[1];
    let output_filename = &args[2];

    // Open reader an writer
    let sequences = fastq_reader(&input_filename);
    let mut outwriter = fastq_writer(&output_filename);

    // Iterate over sequences
    for seq in sequences.records() {
        let s = seq.unwrap().clone();
        outwriter.write_record(&s).unwrap();
    }
}
```
