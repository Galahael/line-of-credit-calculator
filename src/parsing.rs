use chrono::NaiveDate;
use rust_decimal::Decimal;

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
