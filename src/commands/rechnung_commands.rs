use rusty_money::{Money, iso};

use rusty_money::iso::Currency;

pub struct RechnungErstellen {
    pub(crate) rechnungs_nummer: i32,
    pub(crate) betrag: Money<'static, Currency>
}
