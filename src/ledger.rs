use chrono::NaiveDate;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use crate::parsing::Entry;

fn apply_interest(principal: &Decimal, interval_interest_rate: &Decimal) -> Decimal {
    principal * interval_interest_rate
}

fn determine_interval_interest(date: &NaiveDate, interest_rate: &Decimal) -> Decimal {
    if date.leap_year() {
        interest_rate / dec!(100) / dec!(365)
    } else {
        interest_rate / dec!(100) / dec!(366)
    }
}

pub fn compute_tranche(
    as_of_date: NaiveDate,
    ledger: &mut Vec<Entry>,
    tranche_index: usize,
) -> Decimal {
    let mut running_date = ledger[tranche_index].date();
    let mut interest = dec!(0);

    while ledger[tranche_index].amount() != dec!(0) && running_date <= as_of_date {
        loop {
            let repayment_index = ledger
                .iter()
                .enumerate()
                .skip(tranche_index + 1)
                .find(|(_, repayment)| {
                    matches!(repayment, Entry::Repayment(_))
                        && repayment.amount() != dec!(0)
                        && repayment.date() == running_date
                })
                .map(|(i, _)| i);

            match repayment_index {
                None => break,
                Some(repayment_index) => {
                    let repayment_amount = ledger[repayment_index].amount();
                    let tranche_amount = ledger[tranche_index].amount();

                    // If the repayment is smaller than the tranche principal, subtract that amount from the tranche and delete the repayment from the table:
                    if repayment_amount < tranche_amount {
                        ledger[tranche_index].set_amount(tranche_amount - repayment_amount);
                        ledger.remove(repayment_index);
                    // If the repayment is equal to or larger than the tranche, zero the amount in the tranche, and modify the repayment amount accordingly:
                    } else {
                        ledger[tranche_index].set_amount(dec!(0));
                        ledger[repayment_index].set_amount(repayment_amount - tranche_amount);
                        break;
                    }
                }
            }
        }

        if ledger[tranche_index].amount() == dec!(0) {
            println!("   Tranche paid off.");
            break;
        }

        let daily_rate =
            determine_interval_interest(&running_date, &ledger[tranche_index].rate().unwrap());
        interest += apply_interest(&ledger[tranche_index].amount(), &daily_rate);

        println!(
            "{:?}, ${}, {}",
            &ledger[tranche_index].amount().round_dp(2),
            interest.round_dp(2),
            running_date
        );

        running_date += chrono::Duration::days(1);
    }

    interest
}

// Bugs:
// 1. compute_tranche cannot seem to account for payments made in the same day.
//
// Error Handling:
// 1. Should not allow repayments that result in negative principal.
