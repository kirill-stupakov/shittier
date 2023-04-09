use std::{path::PathBuf, fs::File, io::{BufReader, BufRead, BufWriter, Write}, error::Error};
use clap::Parser;
use rand::{thread_rng, Rng};

/// Format a file so it looks like crap
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CliArguments {
    /// File path
    #[arg(short, long)]
    file: PathBuf,
}

fn format(reader: impl BufRead, mut writer: impl Write) -> Result<(), Box<dyn Error>> {
    let mut rng = thread_rng();
    for line in reader.lines() {
        let line = line?;
        let indentation = rng.gen_range(0..=64);

        let indented_line = " ".repeat(indentation) + line.trim();
        writeln!(writer, "{}", indented_line)?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>>{
    let args = CliArguments::parse();

    let file = File::open(args.file)?;
    let reader = BufReader::new(file);
    let output_file = File::create("format.txt")?;
    let writer = BufWriter::new(output_file);

    format(reader, writer)
}
