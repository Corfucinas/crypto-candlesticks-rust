//! Sqlite database class.
use colorful::Colorful;
use rusqlite::{params, Connection};
use std::process;

use crate::exchanges::bitfinex::CandleData;

#[derive(Debug)]
pub struct SqlDatabase {
    data_base_file: String,
    conn: Connection,
}

impl SqlDatabase {
    pub fn new(data_base_file: String) -> Self {
        Self {
            data_base_file: data_base_file.to_string(),
            conn: Connection::open(data_base_file + ".sqlite").unwrap_or_else(|_| {
                eprintln!("{}", "Could not write data to the database".red());
                process::exit(1);
            }),
        }
    }
}

/// Sqlite table schema.
fn create_schema<'a>() -> &'a str {
    "CREATE TABLE IF NOT EXISTS Candlestick(
    ID INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    Timestamp REAL,
    Open REAL,
    Close REAL,
    High REAL,
    Low REAL,
    Volume REAL,
    Ticker TEXT,
    Interval TEXT)"
}

/**
Writes the candlestick data into a SQL table.
```text
Args:
    file_name: &str: Filename for the file
    candlestick_info: &[CandleData]: List containing the candlestick information.
    ticker: &str: Time interval of the candle.
    interval: &str: Time period downloaded.
```
    */
pub fn insert_candlesticks(candlestick_info: &[CandleData], ticker: &str, interval: &str) {
    let connection: Connection = SqlDatabase::new(ticker.to_string() + "-" + interval).conn;
    connection
        .execute(&create_schema(), [])
        .unwrap_or_else(|_| {
            eprintln!("{}", "Could not write schema to the database".red());
            process::exit(1);
        });

    candlestick_info.iter().for_each(|candlestick| {
        candlestick.0.clone().into_iter().for_each(|candle_data| {
            connection
                .execute(
                    "INSERT INTO Candlestick (Timestamp, Open, Close, High, Low, Volume, Ticker, Interval)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                    params![
                        candle_data[0],
                        candle_data[2],
                        candle_data[1],
                        candle_data[3],
                        candle_data[4],
                        candle_data[5],
                        ticker,
                        interval,
                    ],
                )
                .unwrap_or_else(|_| {
                    eprintln!("{}", "Could not insert data to the table".red());
                    process::exit(1);
                });
        });
    });
}
