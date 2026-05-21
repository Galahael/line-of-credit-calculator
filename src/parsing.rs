use chrono::NaiveDate;
use rust_decimal::Decimal;

pub fn trim_document(document: String) -> Vec<Vec<String>> {
    let mut translated_document = Vec::new();

    for line in document.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let tokens: Vec<String> = line.split_whitespace().map(|t| t.to_string()).collect();
        translated_document.push(tokens);
    }
    translated_document
}

#[derive(Debug)]
pub enum Entry {
    Draw(Draw),
    Repayment(Repayment),
    RepaymentFull(RepaymentFull),
}
impl Entry {
    // Get Functions:
    pub fn date(&self) -> NaiveDate {
        match self {
            Entry::Draw(entry) => entry.date,
            Entry::Repayment(entry) => entry.date,
            Entry::RepaymentFull(entry) => entry.date,
        }
    }

    pub fn amount(&self) -> Decimal {
        match self {
            Entry::Draw(entry) => entry.amount,
            Entry::Repayment(entry) => entry.amount,
            Entry::RepaymentFull(entry) => entry.amount,
        }
    }

    pub fn rate(&self) -> Option<Decimal> {
        match self {
            Entry::Draw(entry) => Some(entry.rate),
            _ => None,
        }
    }

    // Set Function:
    pub fn set_amount(&mut self, new_amount: Decimal) {
        match self {
            Entry::Draw(entry) => entry.amount = new_amount,
            Entry::Repayment(entry) => entry.amount = new_amount,
            Entry::RepaymentFull(entry) => entry.amount = new_amount,
        }
    }
}

#[derive(Debug)]
pub struct Draw {
    pub date: NaiveDate,
    pub amount: Decimal,
    pub rate: Decimal,
}
#[derive(Debug)]
pub struct Repayment {
    pub date: NaiveDate,
    pub amount: Decimal,
}
#[derive(Debug)]
pub struct RepaymentFull {
    pub date: NaiveDate,
    pub amount: Decimal,
}

pub fn translate_document(document: Vec<Vec<String>>) -> Vec<Entry> {
    let mut tranche_vec: Vec<Entry> = Vec::new();

    for line in document {
        let tranche = match line.as_slice() {
            [date, draw, amount, at, rate, ..] if draw == "DRAW" && at == "@" => {
                Entry::Draw(Draw {
                    date: date_conversion(date),
                    amount: dollar_to_decimal_conversion(amount),
                    rate: rate_to_decimal_conversion(rate),
                })
            }
            [date, repayment, amount, ..] if repayment == "REPAYMENT" && amount != "FULL" => {
                Entry::Repayment(Repayment {
                    date: date_conversion(date),
                    amount: dollar_to_decimal_conversion(amount),
                })
            }
            [date, repayment, full, amount, ..] if repayment == "REPAYMENT" && full == "FULL" => {
                Entry::RepaymentFull(RepaymentFull {
                    date: date_conversion(date),
                    amount: dollar_to_decimal_conversion(amount),
                })
            }
            _ => continue,
        };
        tranche_vec.push(tranche)
    }

    tranche_vec
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
