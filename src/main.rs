use clap::Parser;

mod ledger;
mod parsing;
mod state;
mod translation;

use state::{arguments::Args, run};

fn main() {
    let args = Args::parse();
    if let Err(error) = run(args) {
        eprintln!("error: {}", error);
        std::process::exit(1);
    }
}

// Setup:
// todo: enter one of several program operations

// Main Operation:
// 1. Program states from a match table as indicated by the user.

// Program States:
// 1. Compute principal and interest from ledger.
// 2. Print ledger
// 3. Make new entry

// General Notes

// Optionality worth considering:
// 1. A print-out table per-tranche of outstanding and interest.
// 2. CLI write operations
// 3. Different compounding frequencies.
// 4. As-of dates preclude and ignore full repayments made on that same day.
// 5. Human readable numbers on the output.

// Error Handling:
// 1. Need to make sure the ledger is in order.
