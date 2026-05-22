use chrono::NaiveDate;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use crate::parsing::Entry;

fn apply_interest(date: &NaiveDate, interest_rate: &Decimal, principal: &Decimal) -> Decimal {
    let interval_interest_rate = match date.leap_year() {
        true => interest_rate / dec!(100) / dec!(366),
        false => interest_rate / dec!(100) / dec!(365),
    };

    principal * interval_interest_rate
}

pub fn compute_tranche(
    as_of_date: NaiveDate,
    ledger: &mut Vec<Entry>,
    tranche_index: usize,
) -> (Decimal, Decimal) {
    let mut running_date = ledger[tranche_index].date();
    let mut interest = dec!(0);
    let mut paid_interest = dec!(0);

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
                    let mut repayment_amount = ledger[repayment_index].amount();
                    let tranche_amount = ledger[tranche_index].amount();

                    // Interest is paid off first:
                    if repayment_amount < interest {
                        paid_interest += repayment_amount;
                        interest -= repayment_amount;
                        ledger.remove(repayment_index);
                        break;
                    } else {
                        ledger[repayment_index].set_amount(repayment_amount - interest);
                        repayment_amount = ledger[repayment_index].amount();
                        paid_interest += interest;
                        interest = dec!(0);
                    }
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

        interest += apply_interest(
            &running_date,
            &ledger[tranche_index].rate().unwrap(),
            &(ledger[tranche_index].amount() + interest),
        );

        println!(
            "{:?}, ${}, {}",
            &ledger[tranche_index].amount().round_dp(2),
            interest.round_dp(2),
            running_date,
        );

        running_date += chrono::Duration::days(1);
    }

    (interest, paid_interest)
}

// Bugs:
// 1. compute_tranche cannot seem to account for payments made in the same day.
//
// Error Handling:
// 1. Should not allow repayments that result in negative principal.

// Todo:
// 1. Include a running figure for interest that has been paid off.
