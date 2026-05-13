use chrono::{Local, NaiveDate};
use clap::Parser;
use std::path::PathBuf;

use rust_decimal_macros::dec;

// use std::fmt;
use std::fs;

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
    let translated_document = document_translation(document);

    //todo: Vector of Draws
    let mut draw_vec: Vec<Entry> = Vec::new();
    let mut repayment_full_vec: Vec<Entry> = Vec::new();
    let mut repayment_vec: Vec<Entry> = Vec::new();

    for entry in translated_document {
        match entry {
            Entry::Draw { .. } => draw_vec.push(entry),
            Entry::RepaymentFull { .. } => repayment_full_vec.push(entry),
            Entry::Repayment { .. } => repayment_vec.push(entry),
        }
    }
    //todo: Vector of RepaymentFulls
    //todo: Vector of Repayments
}

pub mod parser {
    use chrono::NaiveDate;
    use rust_decimal::Decimal;

    pub enum Entry {
        Draw {
            date: NaiveDate,
            amount: Decimal,
            rate: Decimal,
        },
        RepaymentFull {
            date: NaiveDate,
            amount: Decimal,
        },
        Repayment {
            date: NaiveDate,
            amount: Decimal,
        },
    }

    pub fn document_translation(document: String) -> Vec<Entry> {
        let mut translated_document: Vec<Entry> = Vec::new();

        for line in document.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let tokens: Vec<&str> = line.split_whitespace().collect();

            let entry = match tokens.as_slice() {
                [date, "DRAW", amount, "@", rate, ..] => Entry::Draw {
                    date: date_conversion(date),
                    amount: dollar_to_decimal_conversion(amount),
                    rate: rate_to_decimal_conversion(rate),
                },
                [date, "REPAYMENT", "FULL", amount, ..] => Entry::RepaymentFull {
                    date: date_conversion(date),
                    amount: dollar_to_decimal_conversion(amount),
                },
                [date, "REPAYMENT", amount, ..] => Entry::Repayment {
                    date: date_conversion(date),
                    amount: dollar_to_decimal_conversion(amount),
                },
                _ => {
                    eprintln!("unrecognized entry: {}", line);
                    continue;
                }
            };

            translated_document.push(entry);
        }
        translated_document
    }
}

pub mod ledger {
    use chrono::NaiveDate;
    use rust_decimal::Decimal;
    use rust_decimal_macros::dec;
    use std::path::PathBuf;

    pub enum YearType {
        RegularYear,
        LeapYear,
    }

    pub fn tranche_calculator(
        tranche: String,
        repayments: &mut Vec<String>,
        date: NaiveDate,
    ) -> (Decimal, Decimal) {
        apply_interest(principal, interest_rate)
    }

    pub fn days_in_year(year: i32) -> i32 {
        if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
            366
        } else {
            365
        }
    }

    pub fn apply_interest(principal: &Decimal, interest_rate: Decimal) -> Decimal {
        let year = 2000; //todo: Determine which year we're in
        principal * interest_rate
    }

    pub fn interest_interval_calculation(
        date: NaiveDate,
        days_out: u32,
        interest_rate: Decimal,
    ) -> Decimal {
        let year = 2000;
        interest_rate / dec!(100) / dec!(days_in_year(year))
    }

    pub fn dollar_to_decimal_conversion(amount: &str) -> Decimal {
        // let amount: &str;
        amount
            .trim_start_matches('$')
            .replace('_', "")
            .parse()
            .expect("invalid amount")
    }

    pub fn rate_to_decimal_conversion(rate: &str) -> Decimal {
        rate.trim_end_matches('%').parse().expect("invalid rate")
    }

    pub fn date_conversion(date: &str) -> NaiveDate {
        NaiveDate::parse_from_str(date, "%Y-%m-%d").expect("invalid date")
    }
    pub fn default_ledger_path() -> PathBuf {
        PathBuf::from("~/share/.local/self-lending-ledger")
    }
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
