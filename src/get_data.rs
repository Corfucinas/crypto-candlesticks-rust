//! The Crypto candlesticks engine.
use crate::{
    bitfinex::connector::{Bitfinex, CandleData},
    database::insert_candlesticks,
    text_console::{setup_table, write_to_column},
};
use chrono::{TimeZone, Utc};
use colorful::Colorful;
use simple_excel_writer::{blank, row, CellValue, Row, Sheet, SheetWriter, Workbook};
use std::{thread, time};
use time::Duration;

/// Avoid getting rate limited by Bitfinex.
const RATE_LIMIT: f32 = 0.5;
/// Slice of data to be downloaded.
const STEP_SIZE: i64 = 86400000;

/**
Calls the exchange for the data and extends it into a list.
```text
Args:
    ticker: &str: Ticker to download the data.
    mut start_time: i64: Time in ms on which the data will start.
    end_time: i64: Time in ms on which the data will finish.
    interval: &str: Period downloaded.
    step_size: i64: The size step for each call. Defaults to _STEP_SIZE.

Returns:
    Vec<CandleData>: A vector of FloatOrInt.
```
    */
pub fn get_candles(
    ticker: &str,
    mut start_time: i64,
    end_time: i64,
    interval: &str,
    step_size: i64,
) -> Vec<CandleData> {
    let mut candle_data: Vec<CandleData> = Vec::new();
    let message: String = format!("Downloading {} data for {} interval...", ticker, interval);
    let bitfinex: Bitfinex = Bitfinex::new();
    println!("{}", message.yellow());
    while start_time <= end_time {
        let period: i64 = start_time + step_size;
        let candlestick: Option<CandleData> =
            bitfinex.get_candles(ticker, interval, start_time, period);
        if candlestick.is_none() {
            let panic_message: &str =
                "Data could not be downloaded ❌, please verify your connection and try again";
            panic!("{}", panic_message.red());
        }
        write_to_column(ticker, interval, candle_data.clone(), setup_table());
        candle_data.push(candlestick.expect("Could not append to datalist"));
        start_time = period;
        thread::sleep(Duration::from_secs_f32(RATE_LIMIT));
    }
    candle_data
}

/**
Converts the data to an excel sheet.
```text
Args:
   filename: String: Filename of the file.
   interval: &str: Time interval of the data.
    parsed_data: Vec<CandleData>: A Vector of CandleData struct.
    ticker: &str: Quote + base asset.
```
    */
fn write_excel(filename: String, interval: &str, parsed_data: Vec<CandleData>, ticker: &str) {
    let mut workbook: Workbook = Workbook::create(
        &(filename + "-" + &Utc::now().format("%Y-%m-%d %H-%M-%S").to_string() + ".xlsx"),
    );
    let mut worksheet: Sheet = workbook.create_sheet("Crypto-candlesticks");
    workbook
        .write_sheet(
            &mut worksheet,
            |sheet_writer: &mut SheetWriter| -> Result<(), std::io::Error> {
                sheet_writer.append_row(row![
                    "open",
                    "high",
                    "low",
                    "close",
                    "volume",
                    "interval",
                    "ticker",
                    "timestamp"
                ])?;
                parsed_data.into_iter().for_each(|candlestick| {
                    candlestick.0.into_iter().for_each(|candle_data| {
                        let (timestamp, open, close, high, low, volume) = (
                            &candle_data[0],
                            &candle_data[2],
                            &candle_data[1],
                            &candle_data[3],
                            &candle_data[4],
                            &candle_data[5],
                        );
                        let datetime: String = Utc
                            .timestamp_millis(
                                timestamp
                                    .to_string()
                                    .parse()
                                    .expect("Datetime could not be converted from timestamp"),
                            )
                            .to_string();
                        sheet_writer
                            .append_row(row![
                                *open, *close, *high, *low, *volume, interval, ticker, datetime
                            ])
                            .expect("Writing to excel failed");
                    });
                });
                sheet_writer.append_row(row![blank!(1), blank!(1), blank!(1)])
            },
        )
        .expect("Writing to excel failed");
    workbook.close().expect("Writing to excel failed");
}

/// Function for handling the OHLC response and conversion.
pub fn get_data(symbol: &str, base_currency: &str, interval: &str, time_start: i64, time_end: i64) {
    println!("{}", "Welcome, let's download your data".green());
    let ticker: String = format!("{}{}", symbol, base_currency);
    let candle_stick_data: Vec<CandleData> =
        get_candles(&ticker, time_start, time_end, interval, STEP_SIZE);
    let output: String = ticker.clone() + "-" + interval;
    println!("{}", "Data download completed! 🚀".green());
    println!("{}", "Processing data...".yellow());
    insert_candlesticks(&output, &candle_stick_data, &ticker, interval);
    write_excel(output, interval, candle_stick_data, &ticker);
    println!("{}", "Writing to database completed! 🚀🚀".green());
    println!("{}", "Writing to Excel...".yellow());
    println!("{}", "Writing to Excel completed! 🚀🚀🚀".green());
    println!("----------------------");
    println!("{}", buy_me_a_coffee());
}

/**
Why not?
```text
Message to be displayed at the end.

Returns:
   &str: ETH address.
```
*/
fn buy_me_a_coffee() -> &'static str {
    "Thank you for using crypto-candlesticks
Consider supporting your developers
ETH: 0x06Acb31587a96808158BdEd07e53668d8ce94cFE
"
}
