pub mod erstelle_rechnung {
    use rusty_money::Money;
    use rusty_money::iso::Currency;
    use chrono::naive::NaiveDate;

    #[derive(Debug)]
    pub struct Command<'a> {
        pub rechnungs_nummer: String,
        pub rechnungs_datum: Option<NaiveDate>,
        pub betrag: Money<'a, Currency>,
    }

    impl<'a> Command<'a> {
        pub fn new(rechnungs_nummer: String, rechnungs_datum: Option<NaiveDate>, betrag: Money<Currency>) -> Command {
            Command {
                rechnungs_nummer,
                rechnungs_datum,
                betrag,
            }
        }

        pub fn to_string(&self) -> String {
            format!("Betrag {}\nRechnung # {}", &self.rechnungs_nummer, &self.betrag)
        }

        pub fn to_string_verbose(&self) -> String {
            format!("{:?}", &self)
        }
    }
}