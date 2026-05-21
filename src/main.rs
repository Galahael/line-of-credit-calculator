mod ledger;
mod parsing;

use chrono::{Local, NaiveDate};
use clap::Parser;
use rust_decimal_macros::dec;
use std::path::PathBuf;

use std::fs;

use crate::ledger::compute_tranche;
use crate::parsing::{translate_document, trim_document, Entry};

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
    let as_of_date = match args.as_of {
        Some(ref s) => NaiveDate::parse_from_str(s, "%Y-%m-%d")
            .expect("invalid date format, expected YYYY-MM-DD"),
        None => Local::now().date_naive(),
    };

    let document = fs::read_to_string(&file).expect("Could not read ledger file");
    let trimmed_document = trim_document(document);
    let mut ledger = translate_document(trimmed_document);

    // Filters out any date after the "as-of" if it was provided:
    ledger.retain(|entry| match entry {
        Entry::Draw(draw) => draw.date < as_of_date,
        Entry::Repayment(repayment) => repayment.date < as_of_date,
        Entry::RepaymentFull(repaymentfull) => repaymentfull.date < as_of_date,
    });

    // Finds the last full repayment entry and removes every entry before it:
    if let Some(index) = ledger
        .iter()
        .rposition(|entry| matches!(entry, Entry::RepaymentFull(_)))
    {
        ledger.drain(..=index);
    }

    let mut total_interest = dec!(0);
    let mut total_principal = dec!(0);

    // println!("{:?}", ledger);
    let mut index = 0;

    loop {
        if ledger.len() > index {
            total_interest += compute_tranche(as_of_date, &mut ledger, index);
        } else {
            break;
        }
        index += 1;
    }

    for tranche in ledger {
        if matches!(tranche, Entry::Draw(_)) {
            total_principal += tranche.amount();
        } else {
            eprintln!(
                "A lingering REPAYMENT has not been accounted for. This is that repayment: {:?}",
                tranche
            );
        }
    }

    println!();
    println!("Outstanding Principal: ${}", total_principal.round_dp(2));
    println!("Acquired Interest: ${}", total_interest.round_dp(2));
    println!("     - As of: {}", as_of_date);
}

// Setup:
// todo: enter one of several program operations

// Main Operation:
// 1. Program states from a match table as indicated by the user.

// General Notes

// Optionality worth considering:
// 1. A print-out table per-tranche of outstanding and interest.
// 2. CLI write operations
// 3. Different compounding frequencies

// Error Handling:
// 1. Need to make sure the ledger is in order.
