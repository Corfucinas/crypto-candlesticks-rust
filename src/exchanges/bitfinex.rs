//! Main stuct for the Bitfinex exchange.

use colorful::Colorful;
use core::fmt;
use fmt::{Display, Formatter, Result};
use reqwest::{blocking, StatusCode};
use rusqlite::ToSql;
use serde::Deserialize;
use simple_excel_writer::{CellValue, ToCellValue};
use std::{thread, time};

#[derive(Deserialize, Debug, Clone)]
#[serde(transparent)]
pub struct CandleData(pub Vec<[FloatOrInt; 6]>);

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(untagged)]
pub enum FloatOrInt {
    Int(i64),
    Float(f64),
}

impl Display for FloatOrInt {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            FloatOrInt::Int(int) => write!(f, "{}", int),
            FloatOrInt::Float(float) => write!(f, "{}", float),
        }
    }
}

impl ToCellValue for FloatOrInt {
    fn to_cell_value(&self) -> CellValue {
        match self {
            FloatOrInt::Int(int) => CellValue::String(int.to_string()),
            FloatOrInt::Float(float) => CellValue::String(float.to_string()),
        }
    }
}

impl ToSql for FloatOrInt {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        match self {
            FloatOrInt::Int(int) => ToSql::to_sql(int),
            FloatOrInt::Float(float) => ToSql::to_sql(float),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Bitfinex<'a> {
    api_v1: &'a str,
    api_v2: &'a str,
}

impl<'a> Bitfinex<'a> {
    pub fn new() -> Self {
        Self {
            api_v1: "https://api.bitfinex.com/v1",
            api_v2: "https://api.bitfinex.com/v2",
        }
    }

    /**
    Downloads the candlestick data for the given period.
    ```text
    Args:
    start_time: i64: Time in ms on which the data will start.
    ticker: &str: Cryptocurrency pair.
    end_time: i64: Time in ms on which the data will finish.
    time_interval: &str: Interval of the data.

    Returns: Option<CandleData>: Returns a list of candle data which can be parsed.
    ```
    */
    pub fn get_candles(
        self,
        tickers: &str,
        time_interval: &str,
        start_time: i64,
        end_time: i64,
    ) -> Option<CandleData> {
        let url: String = format!(
            "{}/candles/trade:{}:t{}/hist?limit={}&start={}&end={}&sort=-1",
            self.api_v2,
            time_interval,
            tickers.to_uppercase(),
            10000, /* max allowed by Bitfinex */
            start_time,
            end_time
        );
        let candle_data_request = blocking::get(&url);
        if candle_data_request.is_err() {
            println!("{}", candle_data_request.unwrap_err());
            self.retry_candles(&url)
        } else if candle_data_request.is_ok()
            && candle_data_request.as_ref().unwrap().status() == StatusCode::OK
        {
            Some(candle_data_request.unwrap().json().unwrap())
        } else {
            None
        }
    }
    /**
    Calls the exchange and gets all current tickers.
    ```text
    Returns: Option<String>: All available tickers.
    ```
    */
    pub fn get_symbols(self) -> Option<String> {
        let url: String = format!("{}{}", self.api_v1, "/symbols");
        let symbols_request = blocking::get(&url);
        if symbols_request.is_err() {
            self.retry_symbol(&url)
        } else if symbols_request.is_ok()
            && symbols_request.as_ref().unwrap().status() == StatusCode::OK
        {
            Some(symbols_request.unwrap().text().unwrap())
        } else {
            None
        }
    }

    /// Will retry to download the data in case of an interruption.
    fn retry_symbol(self, url: &str) -> Option<String> {
        let mut counter: i8 = 0;
        loop {
            counter += 1;
            let retry_symbol_request = blocking::get(url);
            thread::sleep(time::Duration::from_secs(1));
            if counter >= 15 {
                println!("{}", "Cannot connect to Bitfinex, please try again".red())
            } else if retry_symbol_request.is_ok()
                && retry_symbol_request
                    .as_ref()
                    .expect("Error downloading the data, try again")
                    .status()
                    == StatusCode::OK
            {
                retry_symbol_request
                    .expect("Error downloading the data, try again")
                    .json::<String>()
                    .expect("Error parsing the data, try again");
            }
        }
    }
    /// Will retry to download the data in case of an interruption.
    fn retry_candles(self, url: &str) -> Option<CandleData> {
        let mut counter: i8 = 0;
        loop {
            counter += 1;
            let retry_candles_request = blocking::get(url);
            thread::sleep(time::Duration::from_secs(1));
            if counter >= 15 {
                println!("{}", "Cannot connect to Bitfinex, please try again".red())
            } else if retry_candles_request.is_ok()
                && retry_candles_request
                    .as_ref()
                    .expect("Error downloading the data, try again")
                    .status()
                    == StatusCode::OK
            {
                retry_candles_request
                    .expect("Error downloading the data, try again")
                    .json::<CandleData>()
                    .expect("Error downloading the data, try again");
            }
        }
    }
}
