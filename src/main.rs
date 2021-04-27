//! Command-line interface for Crypto Candlesticks.

mod database;
mod exchanges;
mod get_data;
mod symbols;
mod text_console;
use chrono::{TimeZone, Utc};
use clap::{App, Arg};
use colorful::Colorful;
use exchanges::bitfinex::Bitfinex;
use get_data::get_data;
use std::{io::Error, str::FromStr, thread, time};
use symbols::{intervals::INTERVALS, list_of_currency::LIST_OF_CURRENCY};
use time::Duration;

/// Info message -- --help
fn info() -> &'static str {
    "Download cryptocurrency candlestick data from Bitfinex.
    If the data is obtained successfully, it will be converted to a .csv and a sqlite3 database."
}

/**
Cryptocurrency symbol to download (ie. BTC, ETH, LTC).
```text
Full list can be obtained here => https://api.bitfinex.com/v1/symbols
```
*/
fn symbol() -> Arg<'static> {
    Arg::new("symbol")
        .short('s')
        .long("symbol")
        .value_name("BTC")
        .about("Cryptocurrency symbol to download (ie. BTC, ETH, LTC)")
        .takes_value(true)
        .required(true)
        .display_order(1)
        .default_value("BTC")
}

/**
Cryptocurrency base trading pair.
```text
"USD", "UST", "EUR", "CNHT", "GBP", "JPY", "DAI", "BTC", "EOS", "ETH", "XCH", "USTF0"
```
*/
fn base_currency() -> Arg<'static> {
    Arg::new("base_currency")
        .short('b')
        .long("base_currency")
        .value_name("USDT")
        .about("Cryptocurrency base trading pair")
        .takes_value(true)
        .required(true)
        .display_order(2)
        .default_value("USD")
}

/**
Interval that will be used to download the data.
```text
"1m, 5, 15m, 30m, 1h, 3h, 6h, 12h, 1D, 7D, 14D, 1M"
```
*/
fn interval() -> Arg<'static> {
    Arg::new("interval")
        .short('i')
        .long("interval")
        .value_name("1m, 5, 15m, 30m, 1h, 3h, 6h, 12h, 1D, 7D, 14D, 1M")
        .about("Interval that will be used to download the data")
        .takes_value(true)
        .required(true)
        .display_order(3)
        .default_value("1D")
}

/**
Date to start downloading the data (ie. YYYY-MM-DD).
```text
2018-01-01
```
*/
fn start_date() -> Arg<'static> {
    Arg::new("start_date")
        .short('d')
        .long("start_date")
        .value_name("YYYY-MM-DD")
        .about("Date to start downloading the data (ie. YYYY-MM-DD)")
        .takes_value(true)
        .required(true)
        .display_order(4)
        .default_value("2020-11-01")
}

/**
Date up to the data will be downloaded (ie. YYYY-MM-DD).
```text
2021-01-01
```
*/
fn end_date() -> Arg<'static> {
    Arg::new("end_date")
        .short('e')
        .long("end_date")
        .value_name("YYYY-MM-DD")
        .about("Date up to the data will be downloaded (ie. YYYY-MM-DD)")
        .takes_value(true)
        .required(true)
        .display_order(5)
        .default_value("2021-01-01")
}

/// After -- --help message.
fn repo_info() -> &'static str {
    "Question? Improvements? Feel free to open a PR or issue at: https://github.com/Corfucinas/crypto-candlesticks-rust/issues"
}

/**
Validates the quote currency before making the request.
```text
symbol: &str: A valid asset listed on the exchange
```
 */
fn check_symbol(symbol: &str) -> bool {
    match Bitfinex::new().get_symbols() {
        Some(all_symbols) => all_symbols.contains(&symbol.to_lowercase()),
        None => false,
    }
}

/**
Validates the base currency before making the request.
```text
base_currency: &str: base currency submitted to the stdin
```
*/
fn check_base_currency(base_currency: &str) -> bool {
    LIST_OF_CURRENCY
        .iter()
        .any(|base_currency_list| base_currency_list == &base_currency.to_uppercase())
}

/**
Validates the interval before making the request.
```text
interval: &str: interval submitted to the stdin
```
*/
fn check_interval(interval: &str) -> bool {
    INTERVALS
        .iter()
        .any(|interval_list| interval_list == &interval)
}

/// Converts the dates to miliseconds from the stdin.
fn check_and_transform_dates(start_date: &str, end_date: &str) -> (i64, i64) {
    let message: String = format!(
        "Data could not be downloaded ❌, please make sure your dates
    are in the following format YYYY-MM-DD
    (ie. 2020-01-01), your dates are Start Date: {}, End Date: {}",
        &start_date, &end_date,
    );
    let start_date_parsed: i64 = Utc
        .ymd(
            FromStr::from_str(start_date.split('-').collect::<Vec<&str>>()[0]).expect(&message),
            FromStr::from_str(start_date.split('-').collect::<Vec<&str>>()[1]).expect(&message),
            FromStr::from_str(start_date.split('-').collect::<Vec<&str>>()[2]).expect(&message),
        )
        .and_hms_milli(0, 0, 1, 0)
        .timestamp_millis()
        .clamp(
            Utc.ymd(2016, 1, 1)
                .and_hms_milli(0, 0, 0, 0)
                .timestamp_millis(),
            Utc::now().timestamp_millis(),
        );
    let end_date_parsed: i64 = Utc
        .ymd(
            FromStr::from_str(end_date.split('-').collect::<Vec<&str>>()[0]).expect(&message),
            FromStr::from_str(end_date.split('-').collect::<Vec<&str>>()[1]).expect(&message),
            FromStr::from_str(end_date.split('-').collect::<Vec<&str>>()[2]).expect(&message),
        )
        .and_hms_milli(0, 0, 2, 0)
        .timestamp_millis()
        .clamp(
            Utc.ymd(2016, 1, 1)
                .and_hms_milli(0, 0, 0, 0)
                .timestamp_millis(),
            Utc::now().timestamp_millis(),
        );

    (start_date_parsed, end_date_parsed)
}
/// Reads the arguments from stdin.
pub fn main() -> Result<(), Error> {
    let app_instance: clap::ArgMatches = App::new("crypto-candlesticks-rust")
        .author("Pedro Torres")
        .version("0.1.0")
        .after_help(repo_info())
        .about(info())
        .arg(symbol())
        .arg(base_currency())
        .arg(interval())
        .arg(start_date())
        .arg(end_date())
        .get_matches();

    match (
        app_instance.value_of("symbol"),
        app_instance.value_of("base_currency"),
        app_instance.value_of("interval"),
        app_instance.value_of("start_date"),
        app_instance.value_of("end_date"),
    ) {
        (Some(symbol), Some(base_currency), Some(interval), Some(start_date), Some(end_date)) => {
            if !check_symbol(symbol) {
                let message: String = format!(
                    "Data could not be downloaded ❌, check if {} is listed on Bitfinex",
                    symbol
                );
                panic!("{}", message.red());
            }
            if !check_base_currency(base_currency) {
                let message: String = format!("Data could not be downloaded ❌, check '{}' is listed on Bitfinex as a base pair. The available base currencies are {:#?}",
                base_currency,
                LIST_OF_CURRENCY);
                panic!("{}", message.red());
            }
            if !check_interval(interval) {
                let message: String = format!("Data could not be downloaded ❌, the following intervals are available {:#?}, you have selected {}",
                INTERVALS,
                interval);
                panic!("{}", message.red());
            }
            let (parsed_start_date, parsed_end_date) =
                check_and_transform_dates(start_date, end_date);
            if (symbol == "BTC")
                & (base_currency == "USD")
                & (interval == "1D")
                & (parsed_start_date == 1604188801000)
                & (parsed_end_date == 1609459202000)
            {
                let default_message_warning: &str =
                    "USING DEFAULT VALUES: run --help to know what arguments you can pass";
                println!("{}", default_message_warning.yellow());
                thread::sleep(Duration::from_secs(1));
                println!("3...");
                thread::sleep(Duration::from_secs(1));
                println!("2...");
                thread::sleep(Duration::from_secs(1));
                println!("1...");
            }
            get_data(
                symbol,
                base_currency,
                interval,
                parsed_start_date,
                parsed_end_date,
            )
        }
        _ => {
            let error_message: &str = "Error: Please make sure your inputs are correct.";
            let help_message: &str = "Run with '-- --help' for the arguments";
            println!("{}", error_message.red());
            println!("{}", help_message);
        }
    };
    Ok(())
}

#[test]
fn check_main() {
    use self::main as entry_point;
    entry_point().unwrap();
}
