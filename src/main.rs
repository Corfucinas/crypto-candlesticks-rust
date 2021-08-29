//! Command-line interface for Crypto Candlesticks.

mod database;
mod exchanges;
mod get_data;
mod symbols;
mod text_console;
use chrono::{NaiveDate, Utc};
use clap::{App, Arg};
use colorful::Colorful;
use core::panic;
use exchanges::bitfinex::Bitfinex;
use get_data::get_data;
use std::{error::Error, process, thread, time};
use symbols::{intervals::INTERVALS, list_of_currency::LIST_OF_CURRENCY};
use time::Duration;

/// Info message -- --help.
fn info<'a>() -> &'a str {
    "Download cryptocurrency candlestick data from Bitfinex.
    If the data is obtained successfully, it will be converted to a .csv and a sqlite3 database."
}

/**
Cryptocurrency symbol to download (ie. BTC, ETH, LTC).
```text
Full list can be obtained here => https://api.bitfinex.com/v1/symbols
```
*/
fn symbol<'a>() -> Arg<'a> {
    Arg::new("symbol")
        .short('s')
        .long("symbol")
        .value_name("BTC")
        .about("Cryptocurrency symbol to download (ie. BTC, ETH, LTC)")
        .takes_value(true)
        .display_order(1)
        .default_value("BTC")
        .env("symbol")
}

/**
Cryptocurrency base trading pair.
```text
"USD", "UST", "EUR", "CNHT", "GBP", "JPY", "DAI", "BTC", "EOS", "ETH", "XCH", "USTF0"
```
*/
fn base_currency<'a>() -> Arg<'a> {
    Arg::new("base_currency")
        .short('b')
        .long("base_currency")
        .value_name("USDT")
        .about("Cryptocurrency base trading pair")
        .takes_value(true)
        .display_order(2)
        .default_value("USD")
        .env("base_currency")
}

/**
Interval that will be used to download the data.
```text
"1m, 5, 15m, 30m, 1h, 3h, 6h, 12h, 1D, 7D, 14D, 1M"
```
*/
fn interval<'a>() -> Arg<'a> {
    Arg::new("interval")
        .short('i')
        .long("interval")
        .value_name("1m, 5, 15m, 30m, 1h, 3h, 6h, 12h, 1D, 7D, 14D, 1M")
        .about("Interval that will be used to download the data")
        .takes_value(true)
        .display_order(3)
        .default_value("1D")
        .env("interval")
}

/**
Date to start downloading the data (ie. YYYY-MM-DD).
```text
2018-01-01
```
*/
fn start_date<'a>() -> Arg<'a> {
    Arg::new("start_date")
        .short('d')
        .long("start_date")
        .value_name("YYYY-MM-DD")
        .about("Date to start downloading the data (ie. YYYY-MM-DD)")
        .takes_value(true)
        .display_order(4)
        .default_value("2020-11-01")
        .env("start_date")
}

/**
Date up to the data will be downloaded (ie. YYYY-MM-DD).
```text
2021-01-01
```
*/
fn end_date<'a>() -> Arg<'a> {
    Arg::new("end_date")
        .short('e')
        .long("end_date")
        .value_name("YYYY-MM-DD")
        .about("Date up to the data will be downloaded (ie. YYYY-MM-DD)")
        .takes_value(true)
        .display_order(5)
        .default_value("2021-01-01")
        .env("end_date")
}

/// After -- --help message.
fn repo_info<'a>() -> &'a str {
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

fn check_and_transform_dates(start_date: &str, end_date: &str) -> (i64, i64) {
    let message: String = format!(
        "Data could not be downloaded ❌, please make sure your dates
    are in the following format YYYY-MM-DD
    (ie. 2020-01-01), your dates are Start Date: {}, End Date: {}",
        &start_date, &end_date,
    );

    let earliest: NaiveDate = NaiveDate::from_ymd(2016, 1, 1);
    let today: NaiveDate = Utc::today().naive_utc();

    let parse_date = |date: &str| -> NaiveDate {
        let date: NaiveDate = NaiveDate::parse_from_str(date, "%F").unwrap_or_else(|_| {
            eprintln!("{}", &message);
            process::exit(1);
        });
        if date < earliest {
            earliest
        } else if date > today {
            today
        } else {
            date
        }
    };

    (
        parse_date(start_date).and_hms(0, 0, 1).timestamp() * 1000,
        parse_date(end_date).and_hms(0, 0, 2).timestamp() * 1000,
    )
}
/// Reads the arguments from stdin.
pub fn main() -> Result<(), Box<dyn Error>> {
    verify_arguments_from_app_instance(
        App::new("crypto-candlesticks-rust")
            .author("Pedro Torres")
            .version("0.1.2")
            .after_help(repo_info())
            .about(info())
            .arg(symbol())
            .arg(base_currency())
            .arg(interval())
            .arg(start_date())
            .arg(end_date())
            .get_matches(),
    );
    Ok(())
}

fn verify_arguments_from_app_instance(app_instance: clap::ArgMatches) {
    if let (Some(symbol), Some(base_currency), Some(interval), Some(start_date), Some(end_date)) = (
        app_instance.value_of("symbol"),
        app_instance.value_of("base_currency"),
        app_instance.value_of("interval"),
        app_instance.value_of("start_date"),
        app_instance.value_of("end_date"),
    ) {
        check_values_exist_on_the_exchange(symbol, base_currency, interval);
        let (parsed_start_date, parsed_end_date): (i64, i64) =
            check_and_transform_dates(start_date, end_date);
        check_default_arguments(
            symbol,
            base_currency,
            interval,
            parsed_start_date,
            parsed_end_date,
        );
        get_data(
            symbol,
            base_currency,
            interval,
            parsed_start_date,
            parsed_end_date,
        )
    } else {
        const EXIT_HELP_MESSAGE: [&str; 2] = [
            "Run with '-- --help' for the arguments",
            "Error: Please make sure your inputs are correct.",
        ];
        EXIT_HELP_MESSAGE
            .iter()
            .for_each(|help_message: &&str| println!("{}", help_message.yellow()));
        process::exit(1);
    };
}

fn check_values_exist_on_the_exchange(symbol: &str, base_currency: &str, interval: &str) {
    let message: String = format!(
        "\n Data could not be downloaded ❌, please make sure your inputs are correct.\n Symbol: {}\n, Base Currency: {}\n, Interval: {}\n",
        &symbol, &base_currency, &interval,

    );
    if !check_symbol(symbol) || !check_base_currency(base_currency) || !check_interval(interval) {
        panic!("{}", &message.red());
    }
}

fn check_default_arguments(
    symbol: &str,
    base_currency: &str,
    interval: &str,
    parsed_start_date: i64,
    parsed_end_date: i64,
) {
    if (symbol == "BTC")
        & (base_currency == "USD")
        & (interval == "1D")
        & (parsed_start_date == 1604188801000)
        & (parsed_end_date == 1609459202000)
    {
        const DEFAULT_MESSAGE_WARNING: &str =
            "USING DEFAULT VALUES: run --help to know what arguments you can pass";
        println!("{}", DEFAULT_MESSAGE_WARNING.yellow());
        for second in 1..=3 {
            println!("{}", format!("{}{}", second, "..."));
            thread::sleep(Duration::from_secs(1));
        }
        println!("Starting!");
    }
}

#[cfg(test)]
mod tests {
    use super::main as entry_point;
    use std::env;
    #[test]
    fn main_1m() {
        env::set_var("symbol", "btc");
        env::set_var("base_currency", "usd");
        env::set_var("interval", "1m");
        env::set_var("start_date", "2021-01-01");
        env::set_var("end_date", "2021-02-01");
        entry_point().unwrap();
    }

    #[test]
    fn main_5m() {
        env::set_var("symbol", "eth");
        env::set_var("base_currency", "usd");
        env::set_var("interval", "5m");
        env::set_var("start_date", "2021-01-01");
        env::set_var("end_date", "2021-02-01");
        entry_point().unwrap();
    }

    #[test]
    fn main_15m() {
        env::set_var("symbol", "ltc");
        env::set_var("base_currency", "usd");
        env::set_var("interval", "15m");
        env::set_var("start_date", "2021-01-01");
        env::set_var("end_date", "2021-02-01");
        entry_point().unwrap();
    }

    #[test]
    fn main_30m() {
        env::set_var("symbol", "btc");
        env::set_var("base_currency", "usd");
        env::set_var("interval", "30m");
        env::set_var("start_date", "2021-01-01");
        env::set_var("end_date", "2021-02-01");
        entry_point().unwrap();
    }

    #[test]
    #[should_panic]
    fn fail_symbol() {
        env::set_var("symbol", "notbtc");
        env::set_var("base_currency", "usd");
        env::set_var("interval", "30m");
        env::set_var("start_date", "2021-01-01");
        env::set_var("end_date", "2021-02-01");
        entry_point().unwrap_err();
    }

    #[test]
    #[should_panic]
    fn fail_base_currency() {
        env::set_var("symbol", "btc");
        env::set_var("base_currency", "USDR");
        env::set_var("interval", "30m");
        env::set_var("start_date", "2021-01-01");
        env::set_var("end_date", "2021-02-01");
        entry_point().unwrap();
    }

    #[test]
    #[should_panic]
    fn fail_interval() {
        env::set_var("symbol", "btc");
        env::set_var("base_currency", "usd");
        env::set_var("interval", "10D");
        env::set_var("start_date", "2021-01-01");
        env::set_var("end_date", "2021-02-01");
        entry_point().unwrap();
    }

    #[test]
    #[should_panic]
    fn fail_start_date() {
        env::set_var("symbol", "btc");
        env::set_var("base_currency", "usd");
        env::set_var("interval", "10D");
        env::set_var("start_date", "2021-10-01");
        env::set_var("end_date", "2021-02-300");
        entry_point().unwrap();
    }

    #[test]
    #[should_panic]
    fn fail_end_date() {
        env::set_var("symbol", "btc");
        env::set_var("base_currency", "usd");
        env::set_var("interval", "10D");
        env::set_var("start_date", "2021-20-01");
        env::set_var("end_date", "1980-02-01");
        entry_point().unwrap();
    }
}
