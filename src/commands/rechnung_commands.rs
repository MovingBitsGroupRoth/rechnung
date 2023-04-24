use rusty_money::{Money, iso};

use rusty_money::iso::Currency;

pub struct RechnungErstellen<'a> {
    pub(crate) rechnungs_nummer: &'a str,
    pub(crate) betrag: Money<'a, Currency>
}
