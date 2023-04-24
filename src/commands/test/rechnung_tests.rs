#[cfg(test)]
mod command_tests {
    use rusty_money::{Money, iso};

    use crate::commands::rechnung_commands::RechnungErstellen;

    #[test]
    fn rechnung_erstellen_command() {
        let re = RechnungErstellen {
            rechnungs_nummer: "RE-12",
            betrag: Money::from_str("4009,09", iso::EUR).unwrap()
        };

        assert_eq!(re.rechnungs_nummer, "RE-12");
        assert_eq!(re.betrag.to_string(), "€4.009,09");
    }
}