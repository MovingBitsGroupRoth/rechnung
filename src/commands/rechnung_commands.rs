use rusty_money::{Money, iso};
use rusty_money::iso::Currency;
use chrono::naive::NaiveDate;

pub struct ErstelleRechnung<'a> {
    pub(crate) rechnungs_nummer: String,
    pub(crate) rechnungs_datum: Option<NaiveDate>,
    pub(crate) betrag: Money<'a, Currency>
}

pub fn build_erstelle_rechnung(rechnungs_nummer: String, rechnungs_datum: Option<NaiveDate>, betrag: Money<Currency>) -> ErstelleRechnung {
    ErstelleRechnung {
        rechnungs_nummer,
        rechnungs_datum,
        betrag,
    }
}
