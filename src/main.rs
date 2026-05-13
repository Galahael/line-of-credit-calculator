use chrono::{Local, NaiveDate};
use clap::Parser;
use std::path::PathBuf;

use std::fs;

mod ledger;
mod parser;

#[derive(Parser)]
#[command(
    version = "0.1",
    about = "A CLI application for the purposes of recordkeeping self-lending activities of a business in the LOC format.",
    author = "Galahael"
)]
struct Args {
    // The file the program operates on
    #[arg(short = 'f', long)]
    file: Option<String>,

    // For looking at values at a specific point in time
    #[arg(short = 'a', long)]
    as_of: Option<String>,
}

pub fn default_ledger_path() -> PathBuf {
    PathBuf::from("~/share/.local/self-lending-ledger")
}

fn main() {
    let args = Args::parse();
    let file = match args.file {
        Some(file) => PathBuf::from(file),
        None => default_ledger_path(),
    };
    let date = match args.as_of {
        Some(ref s) => NaiveDate::parse_from_str(s, "%Y-%m-%d")
            .expect("invalid date format, expected YYYY-MM-DD"),
        None => Local::now().date_naive(),
    };

    let document = fs::read_to_string(&file).expect("Could not read ledger file");
    let translated_document = parser::document_translation(document);

    //todo: Vector of Draws
    let mut draw_vec: Vec<parser::Entry> = Vec::new();
    let mut repayment_full_vec: Vec<parser::Entry> = Vec::new();
    let mut repayment_vec: Vec<parser::Entry> = Vec::new();

    for entry in translated_document {
        match entry {
            parser::Entry::Draw { .. } => draw_vec.push(entry),
            parser::Entry::RepaymentFull { .. } => repayment_full_vec.push(entry),
            parser::Entry::Repayment { .. } => repayment_vec.push(entry),
        }
    }
    //todo: Vector of RepaymentFulls
    //todo: Vector of Repayments
}

// Setup:
// done: read CLI options
// todo: enter one of several program operations
// done: open the document and make a copy
// done: translate that document

// Main Operation:
// todo: parse through translated document from the beginning -- or last "REPAYMENT FULL" date
// note: if "as-of" date is specified, drop all entries after the "as-of" date for making an inquiry as if it were made that day.
// todo: for every REPAYMENT, add it to a repayment String array
// todo: run through every draw; they must be accounted for and against existing repayments, such draws may be broken up several times against a repayment history into figures of various compounding that then need summing into the final variable.
// todo: for every REPAYMENT that has been accounted for, remove it from the array.
// todo: return the sums of both interest and principal

// General Notes
// 1. It would be wise to translate the human-readable format to a format more "machine-aligned" for the simplicity of programming and String parsing.

// Optionality worth considering:
// 1. A print-out table per-tranche of outstanding and interest.
// 2. CLI write operations
// 3. Different compounding frequencies
