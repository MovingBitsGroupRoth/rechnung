#[cfg(test)]
use rusty_money::{Money, iso};
use chrono::{Datelike, naive::NaiveDate};

use crate::commands::rechnung_commands::erstelle_rechnung::Command as erstelle_rechnung_command;

#[test]
fn rechnung_erstellen_command() {
    let datum = NaiveDate::from_ymd_opt(2023, 5, 3);

    // let re = ErstelleRechnung {
    //     rechnungs_nummer: String::from("RE-12"),
    //     rechnungs_datum: datum,
    //     betrag: Money::from_str("4009,09", iso::EUR).unwrap()
    // };

    let re = erstelle_rechnung_command::new(
        String::from("RE-12"),
        datum,
        Money::from_str("4009,09", iso::EUR).unwrap()
    );

    assert_eq!(re.rechnungs_nummer, "RE-12");
    assert_eq!(re.betrag.to_string(), "€4.009,09");

    // assert_eq!(re.datum.to_string(), "34");

    let from_ymd_opt = NaiveDate::from_ymd_opt;

    assert!(from_ymd_opt(
            re.rechnungs_datum.unwrap().year(),
            re.rechnungs_datum.unwrap().month(),
            re.rechnungs_datum.unwrap().day())
        .is_some());

    // TODO: Move to app somewhere
    println!("{}",re.to_string());
    println!("{}",re.to_string_verbose());
}
