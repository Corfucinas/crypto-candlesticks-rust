//! Print a table to the console.

use crate::exchanges::bitfinex::CandleData;
use chrono::{TimeZone, Utc};
use comfy_table::{
    presets::ASCII_NO_BORDERS, presets::UTF8_FULL, Attribute, Cell, CellAlignment, Color,
    ContentArrangement, Table,
};

/// Table format to be displayed while data is downloaded.
pub fn setup_table() -> Table {
    let mut table: Table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_header(vec![
            Cell::new("Open")
                .add_attribute(Attribute::Bold)
                .fg(Color::Green)
                .set_alignment(CellAlignment::Center),
            Cell::new("High")
                .add_attribute(Attribute::Bold)
                .fg(Color::Green)
                .set_alignment(CellAlignment::Center),
            Cell::new("Low")
                .add_attribute(Attribute::Bold)
                .fg(Color::Green)
                .set_alignment(CellAlignment::Center),
            Cell::new("Close")
                .add_attribute(Attribute::Bold)
                .fg(Color::Green)
                .set_alignment(CellAlignment::Center),
            Cell::new("Volume")
                .add_attribute(Attribute::Bold)
                .fg(Color::Green)
                .set_alignment(CellAlignment::Center),
            Cell::new("Timestamp")
                .add_attribute(Attribute::Bold)
                .fg(Color::Green)
                .set_alignment(CellAlignment::Center),
            Cell::new("Ticker")
                .add_attribute(Attribute::Bold)
                .fg(Color::Green)
                .set_alignment(CellAlignment::Center),
            Cell::new("Time")
                .add_attribute(Attribute::Bold)
                .fg(Color::Green)
                .set_alignment(CellAlignment::Center),
        ])
        .set_content_arrangement(ContentArrangement::DynamicFullWidth)
        .apply_modifier(ASCII_NO_BORDERS)
        .trim_fmt();
    table
}

/// Live data from the candles.
pub fn write_to_column(
    ticker: &str,
    interval: &str,
    data_downloaded: Vec<CandleData>,
    mut table: Table,
) {
    data_downloaded
        .into_iter()
        .for_each(|candle_data: CandleData| {
            candle_data.0.into_iter().for_each(|single_candle_info| {
                let datetime: String = Utc
                    .timestamp_millis(
                        single_candle_info[0]
                            .to_string()
                            .parse()
                            .expect("Datetime could not be converted from timestamp"),
                    )
                    .to_string();
                table.add_row(vec![
                    Cell::new(single_candle_info[2]).set_alignment(CellAlignment::Center),
                    Cell::new(single_candle_info[1]).set_alignment(CellAlignment::Center),
                    Cell::new(single_candle_info[3]).set_alignment(CellAlignment::Center),
                    Cell::new(single_candle_info[4]).set_alignment(CellAlignment::Center),
                    Cell::new(single_candle_info[5]).set_alignment(CellAlignment::Center),
                    Cell::new(ticker).set_alignment(CellAlignment::Center),
                    Cell::new(interval).set_alignment(CellAlignment::Center),
                    Cell::new(datetime).set_alignment(CellAlignment::Center),
                ]);
            });
        });
    println!("\n");
    println!("\n");
    println!("\n");
    println!("\n");
    println!("\n");
    println!("\n");
    println!("\n");
    println!("\n");
    println!("\n");
    println!("\n");
    println!("\n");
    println!("\n");
    println!("\n");
    println!("\n");
    println!("\n");
    println!("\n");
    println!("\n");
    println!("\n");
    println!("\n");
    println!("\n");
    println!("\n");
    println!("\n");
    println!("\n");
    println!("\n");
    println!("\n");
    println!("{}", table);
}
