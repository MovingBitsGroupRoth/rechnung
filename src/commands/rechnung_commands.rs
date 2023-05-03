use rusty_money::{Money, iso};
use rusty_money::iso::Currency;
use chrono::naive::NaiveDate;

pub struct ErstelleRechnung<'a> {
    pub(crate) rechnungs_nummer: String,
    pub(crate) rechnungs_datum: Option<NaiveDate>,
    pub(crate) betrag: Money<'a, Currency>
}
