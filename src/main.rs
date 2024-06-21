extern crate anyhow;
extern crate clap;

use std::io::prelude::*;
use std::fs::File;
use anyhow::Result;
use clap::Parser;


/// Convert CLI list or newline file of hostnames to Konsole tab-from-file list
#[derive(Parser, Debug)]
// #[command(version, about, long_about = None)]
#[group(multiple = true)]
pub struct Cli {

    #[clap(flatten)]
    group: Required,

    /// The output path with filename
    #[arg(required = false,
        short = 'o',
        long = "output-file",
        default_value = "./default_output.tabs",)]
    pub file: std::path::PathBuf,

}

// Argument group for mutually exclusive flags
#[derive(Debug, clap::Args)]
#[group(required = true, multiple = false)]
pub struct Required {

    /// Space delimited list of hostnames
    #[arg(short = 'l',
        long = "list",
        value_parser,
        value_delimiter = ' ',
        num_args = 0..)]
    pub list: Option<Vec<String>>,

    /// Input file as newline delimited text file
    #[arg(short = 'i',
        long = "input-file",)]
    pub input: Option<std::path::PathBuf>,
}

// File parser for newline ingestion list file
fn read_lines_to_vec(filename: impl AsRef<std::path::Path>) -> Vec<String> {
    let ingestion_file = std::fs::File::open(filename).expect("File does not exist.");
    let buffer = std::io::BufReader::new(ingestion_file);
    buffer.lines().map(|line| line.expect("Could not parse line.")).collect()
}


fn main() -> Result<()> {
    let args:Cli = Cli::parse();
    let output_path_file: std::path::PathBuf = args.file;
    let input_path_file: Option<std::path::PathBuf> = args.group.input;
    let input_cli_stdin: Option<Vec<String>> = args.group.list;


    let mut out_file = File::create(
        &format!("{}", output_path_file.display()))
        .expect(
            &format!("Creation of {} file failed", output_path_file.display()));


    if input_path_file.is_none() {
        for item in input_cli_stdin.unwrap() {
            println!("title: {item};; command: ssh {item}");
            write!(out_file, "title: {item};; command: ssh {item}\n").expect(&format!("Writing to output file {:?} failed.", out_file));
        }
    }
    else {
        for item in read_lines_to_vec(input_path_file.unwrap().into_os_string()){
            println!("title: {item};; command: ssh {item}");
            write!(out_file, "title: {item};; command: ssh {item}\n").expect(&format!("Writing to output file {:?} failed.", out_file))
        }
    }

    Ok(())
}
