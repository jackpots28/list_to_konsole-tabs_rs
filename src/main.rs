use std::fs::{File};
use std::io::Write;
use anyhow::{Result};
use clap::Parser;

/// Convert CLI list of hostnames to Konsole tab-from-file list
#[derive(Parser, Debug)]
// #[command(version, about, long_about = None)]
#[group(required = false, multiple = false)]
pub struct Cli {
    /// The pattern to look for
    #[arg(required = false, short = 'l', long = "list", value_parser, value_delimiter = ' ', num_args = 0..)]
    pub list: Vec<String>,

    // TODO - need to implement multi arg for output /dir/file naming
    // The path to the file to read
    // #[arg(required = false, short = 'f', long = "file")]
    // pub file: std::path::PathBuf,

    // TODO - create arg that depends on if cli "list" is passed to instead take newline delim file
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let output_path_string: &str = "/tmp/output_tabs.txt";

    let mut out_file = File::create(
        &format!("{}", output_path_string))
        .expect(
            &format!("Creation of {} file failed", output_path_string));

    for item in args.list {
        println!("title: {item};; command: ssh {item}");
        write!(out_file, "title: {item};; command: ssh {item}\n").expect("Writing to output file failed");
    }

    // let some_other_test_string: &str = "TEST";
    // let test_string: String = format!("Something\n{}\n", some_other_test_string);
    //
    // std::fs::write("/tmp/test.file", test_string).expect("Unable to write to file.");
    //
    // let output_content = std::fs::read_to_string("/tmp/test.file")
    //     .with_context(|| format!("Could not read file '{}'", "/tmp/test.file"))?;

    Ok(())
}