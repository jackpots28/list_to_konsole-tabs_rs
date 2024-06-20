extern crate anyhow;
extern crate clap;

use std::fs::{File};
use std::io::Write;
use anyhow::{Result};
use clap::Parser;


/// Convert CLI list of hostnames to Konsole tab-from-file list
#[derive(Parser, Debug)]
// #[command(version, about, long_about = None)]
#[group(multiple = true)]
pub struct Cli {

    /// Space delimited list of hostnames
    #[arg(short = 'l',
        long = "list",
        value_parser,
        value_delimiter = ',',
        required_unless_present = "input",
        num_args = 0..)]
    pub list: Vec<String>,

    // TODO - fix the following to allow either required input as stdin or file
    // #[arg(short = 'i',
    //     long = "input",
    //     required_unless_present = "list",)]
    // pub input: std::path::PathBuf,

    /// The output path with filename (default is: ${pwd}/default_output.tabs)
    #[arg(required = false,
        short = 'f',
        long = "file",
        default_value = "./default_output.tabs",)]
    pub file: std::path::PathBuf,

    // pub test_input: Box<dyn std::io::Read + 'static>


    // TODO - create arg that depends on if cli "list" is passed to instead take newline delim file
}


fn main() -> Result<()> {
    let args = Cli::parse();
    let output_path_file: std::path::PathBuf = args.file;

    let mut out_file = File::create(
        &format!("{}", output_path_file.display()))
        .expect(
            &format!("Creation of {} file failed", output_path_file.display()));

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
