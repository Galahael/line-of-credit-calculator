use chrono::NaiveDate;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use crate::parser::Entry;

pub enum YearType {
    RegularYear,
    LeapYear,
}

pub fn tranche_calculator(
    tranche: &mut Entry,
    repayments: &mut Vec<Entry>,
    as_of_date: NaiveDate,
) -> (Decimal, Decimal) {
    let mut total_interest = dec!(0);
    let mut principal = dec!(0);
    let mut interest_rate = dec!(0);
    let mut tranche_date = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();

    if let Entry::Draw { date, amount, rate } = tranche {
        principal = amount;
        interest_rate = rate;
        tranche_date = date;
    }

    let mut days = (as_of_date - tranche_date).num_days();

    while principal != dec!(0) || days != 0 {
        let interval_interest_rate = determine_interval_interest(&interest_rate);
        total_interest += apply_interest(&principal, &interval_interest_rate);

        //todo: continue to add interest up until first repayment date, subtract that repayment from the principal.
        // - If the repayment has consumed the principal, end the loop, and record the new repayment value; if it is zero, strike the repayment from the vec.
        // - If the repayment has not consumed the principal, strike the repayment from the vec and keep iterating on the new principal amount.

        days -= 1;
    }
    (principal, total_interest)
}

// pub fn days_in_year(year: i32) -> i32 {
//     if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
//         366
//     } else {
//         365
//     }
// }

pub fn apply_interest(principal: &Decimal, interval_interest_rate: &Decimal) -> Decimal {
    // let year = 2026; //todo: Determine which year we're in
    principal * interval_interest_rate
}

pub fn determine_interval_interest(
    // date: NaiveDate,
    // days_out: u32,
    interest_rate: &Decimal,
) -> Decimal {
    // let year = 2000;
    // interest_rate / dec!(100) / dec!(days_in_year(year))
    interest_rate / dec!(100) / dec!(365)
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
