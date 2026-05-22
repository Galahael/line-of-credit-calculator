use crate::parsing::{
    date_conversion, dollar_to_decimal_conversion, rate_to_decimal_conversion, Draw, Entry,
    Repayment, RepaymentFull,
};
pub fn translate_document(document: String) -> Vec<Entry> {
    let mut entry_vec: Vec<Entry> = Vec::new();

    let document = trim_document(document);

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
        entry_vec.push(tranche)
    }

    entry_vec
}

fn trim_document(document: String) -> Vec<Vec<String>> {
    let mut trimmed_document = Vec::new();

    for line in document.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let tokens: Vec<String> = line.split_whitespace().map(|t| t.to_string()).collect();
        trimmed_document.push(tokens);
    }
    trimmed_document
}
