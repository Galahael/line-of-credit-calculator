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
                date: ledger::date_conversion(date),
                amount: dollar_to_decimal_conversion(amount),
                rate: rate_to_decimal_conversion(rate),
            },
            [date, "REPAYMENT", "FULL", amount, ..] => Entry::RepaymentFull {
                date: ledger::date_conversion(date),
                amount: ledger::dollar_to_decimal_conversion(amount),
            },
            [date, "REPAYMENT", amount, ..] => Entry::Repayment {
                date: ledger::date_conversion(date),
                amount: ledger::dollar_to_decimal_conversion(amount),
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
