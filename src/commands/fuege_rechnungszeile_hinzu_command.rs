use rusty_money::Money;
use rusty_money::iso::Currency;
use chrono::naive::NaiveDate;

#[derive(Debug)]
pub struct FuegeRechnungszeileHinzu<'a> {
    pub artikel: String,
    pub betrag: Money<'a, Currency>,
}

impl FuegeRechnungszeileHinzu<'_> {
    pub fn new(artikel: String, betrag: Money<Currency>) -> FuegeRechnungszeileHinzu {
        FuegeRechnungszeileHinzu {
            artikel,
            betrag,
        }
    }

    pub fn to_string(&self) -> String {
        format!("Betrag {}\nRechnung # {}", &self.artikel, &self.betrag)
    }

    pub fn to_string_verbose(&self) -> String {
        format!("{:?}", &self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rusty_money::{Money, iso};
    use chrono::{Datelike, naive::NaiveDate};

    #[test]
    fn fuege_rechnungszeile_hinzu_command_test() {
        let zeile = FuegeRechnungszeileHinzu::new(
            String::from("artikel1"),
            Money::from_str("101", iso::EUR).unwrap()
        );

        // Move to Rechnung-Aggregate
        let validiere_artikelname_laenge_groesser_2 = |artikel_name: &String| artikel_name.len() > 2; // Encapsulate in Validator-Enum

        if(!validiere_artikelname_laenge_groesser_2(&zeile.artikel)){
            // Soften this :D ... panic is way too harsh here -> implement Meldungssammler
            panic!("Artikelname muss mind. 2 Zeichen lang sein")
        }

        let mut zeilen: Vec<FuegeRechnungszeileHinzu> = vec![];
        // Rechnung-Aggregate END

        zeilen.push(zeile);

        let artikel = &zeilen[0].artikel;

        assert_eq!("artikel1", artikel);
    }
}