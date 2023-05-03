use rusty_money::{Money, iso};
use rusty_money::iso::Currency;
use chrono::naive::NaiveDate;

#[derive(Debug)]
pub struct ErstelleRechnung<'a> {
    pub(crate) rechnungs_nummer: String,
    pub(crate) rechnungs_datum: Option<NaiveDate>,
    pub(crate) betrag: Money<'a, Currency>,
}

impl ErstelleRechnung<'_> {
    pub fn new(rechnungs_nummer: String, rechnungs_datum: Option<NaiveDate>, betrag: Money<Currency>) -> ErstelleRechnung {
        ErstelleRechnung {
            rechnungs_nummer,
            rechnungs_datum,
            betrag,
        }
    }

    pub fn to_string(&self) -> String {
        // We don't want to disclose the secret
        format!("Betrag {}\nRechnung # {}", &self.rechnungs_nummer, &self.betrag)
    }

    pub fn to_string_verbose(&self) -> String {
        format!("{:?}", &self )
    }
}



