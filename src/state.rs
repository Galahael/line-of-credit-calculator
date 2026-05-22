use arguments::Args;
use clap::Subcommand;

#[derive(Subcommand)]
pub enum ProgramState {
    Report,
    Write,
    Export,
}

pub fn run(args: Args) -> Result<(), anyhow::Error> {
    match args.command {
        ProgramState::Report => report::run_report(&args),
        ProgramState::Write => write::run_write(&args),
        ProgramState::Export => export::run_export(&args),
    }
}

pub mod arguments {
    use chrono::{Local, NaiveDate};
    use clap::Parser;
    use std::fs;
    use std::path::PathBuf;

    use crate::state::ProgramState;

    #[derive(Parser)]
    #[command(
        version = "0.1",
        about = "A CLI application for the purposes of recordkeeping self-lending activities of a business in the LOC format.",
        author = "Galahael"
    )]
    pub struct Args {
        // The file the program operates on
        #[arg(short = 'f', long)]
        pub file: Option<String>,

        // For looking at values at a specific point in time
        #[arg(short = 'a', long)]
        pub as_of: Option<String>,

        #[command(subcommand)]
        pub command: ProgramState,
    }

    pub fn args_file(args: &Args) -> String {
        let file = match &args.file {
            Some(file) => PathBuf::from(file),
            None => default_ledger_path(),
        };
        fs::read_to_string(&file).expect("Could not read ledger file")
    }

    pub fn args_date(args: &Args) -> NaiveDate {
        match &args.as_of {
            Some(string_date) => NaiveDate::parse_from_str(string_date, "%Y-%m-%d")
                .expect("invalid date format, expected YYYY-MM-DD"),
            None => Local::now().date_naive(),
        }
    }

    fn default_ledger_path() -> PathBuf {
        PathBuf::from("~/share/.local/self-lending-ledger")
    }
}

pub mod report {
    use anyhow::Ok;
    use rust_decimal_macros::dec;

    use crate::ledger::compute_tranche;
    use crate::parsing::Entry;
    use crate::state::{
        arguments::{args_date, args_file},
        Args,
    };
    use crate::translation::translate_document;

    pub fn run_report(args: &Args) -> Result<(), anyhow::Error> {
        let document = args_file(&args);
        let as_of_date = args_date(&args);

        let mut ledger = translate_document(document);

        let mut total_interest = dec!(0);
        let mut paid_interest = dec!(0);
        let mut total_principal = dec!(0);

        let mut index = 0;

        loop {
            if ledger.len() > index {
                let (a, b) = compute_tranche(as_of_date, &mut ledger, index);
                total_interest += a;
                paid_interest += b;
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

        println!("\nPrincipal: ${}", total_principal.round_dp(2));
        println!("Current Interest: ${}", total_interest.round_dp(2));
        println!("Interest Paid: ${}", paid_interest.round_dp(2));
        println!(
            "\nOutstanding Balance: ${}",
            (total_principal + total_interest).round_dp(2)
        );
        println!("     - As of: {}\n", as_of_date);

        Ok(())
    }
}

pub mod write {
    use super::Args;

    pub fn run_write(args: &Args) -> Result<(), anyhow::Error> {
        Ok(())
    }
}

pub mod export {
    use super::Args;

    pub fn run_export(args: &Args) -> Result<(), anyhow::Error> {
        Ok(())
    }
}
