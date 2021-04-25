//! Test suite for crypto-candlesticks-rust
// #[path = "../src/crypto_candlesticks_rust/get_data.rs"]
// mod get_data;

use chrono::{TimeZone, Utc};

#[test]
fn test_main_succeeds_1m() {
    let start_date: i64 = Utc
        .ymd(2020, 11, 1)
        .and_hms_milli(0, 0, 1, 0)
        .timestamp_millis();
    let end_date: i64 = Utc
        .ymd(2021, 1, 1)
        .and_hms_milli(0, 0, 1, 0)
        .timestamp_millis();
    assert_eq!(true, get_data("BTC", "USD", "1m", start_date, end_date));
}

#[test]
fn test_main_succeeds_5m() {
    let start_date: i64 = Utc
        .ymd(2020, 11, 1)
        .and_hms_milli(0, 0, 1, 0)
        .timestamp_millis();
    let end_date: i64 = Utc
        .ymd(2021, 1, 1)
        .and_hms_milli(0, 0, 1, 0)
        .timestamp_millis();
    assert_eq!(true, get_data("ETH", "USD", "5m", start_date, end_date));
}

#[test]
fn test_main_succeeds_15m() {
    let start_date: i64 = Utc
        .ymd(2020, 11, 1)
        .and_hms_milli(0, 0, 1, 0)
        .timestamp_millis();
    let end_date: i64 = Utc
        .ymd(2021, 1, 1)
        .and_hms_milli(0, 0, 1, 0)
        .timestamp_millis();
    assert_eq!(true, get_data("BTC", "USD", "1m", start_date, end_date));
}

#[test]
fn test_main_succeeds_30m() {
    let start_date: i64 = Utc
        .ymd(2020, 11, 1)
        .and_hms_milli(0, 0, 1, 0)
        .timestamp_millis();
    let end_date: i64 = Utc
        .ymd(2021, 1, 1)
        .and_hms_milli(0, 0, 1, 0)
        .timestamp_millis();
    assert_eq!(true, get_data("BTC", "USD", "1m", start_date, end_date));
}

#[test]
fn test_main_succeeds_1h() {
    let start_date: i64 = Utc
        .ymd(2020, 11, 1)
        .and_hms_milli(0, 0, 1, 0)
        .timestamp_millis();
    let end_date: i64 = Utc
        .ymd(2021, 1, 1)
        .and_hms_milli(0, 0, 1, 0)
        .timestamp_millis();
    assert_eq!(true, get_data("BTC", "USD", "1m", start_date, end_date));
}

#[test]
fn test_main_succeeds_3h() {
    let start_date: i64 = Utc
        .ymd(2020, 11, 1)
        .and_hms_milli(0, 0, 1, 0)
        .timestamp_millis();
    let end_date: i64 = Utc
        .ymd(2021, 1, 1)
        .and_hms_milli(0, 0, 1, 0)
        .timestamp_millis();
    assert_eq!(true, get_data("BTC", "USD", "1m", start_date, end_date));
}

#[test]
fn test_main_succeeds_6h() {
    let start_date: i64 = Utc
        .ymd(2020, 11, 1)
        .and_hms_milli(0, 0, 1, 0)
        .timestamp_millis();
    let end_date: i64 = Utc
        .ymd(2021, 1, 1)
        .and_hms_milli(0, 0, 1, 0)
        .timestamp_millis();
    assert_eq!(true, get_data("BTC", "USD", "1m", start_date, end_date));
}

#[test]
fn test_main_succeeds_12h() {
    let start_date: i64 = Utc
        .ymd(2020, 11, 1)
        .and_hms_milli(0, 0, 1, 0)
        .timestamp_millis();
    let end_date: i64 = Utc
        .ymd(2021, 1, 1)
        .and_hms_milli(0, 0, 1, 0)
        .timestamp_millis();
    assert_eq!(true, get_data("BTC", "USD", "1m", start_date, end_date));
}

#[test]
fn test_main_succeeds_1d() {
    let start_date: i64 = Utc
        .ymd(2020, 11, 1)
        .and_hms_milli(0, 0, 1, 0)
        .timestamp_millis();
    let end_date: i64 = Utc
        .ymd(2021, 1, 1)
        .and_hms_milli(0, 0, 1, 0)
        .timestamp_millis();
    assert_eq!(true, get_data("BTC", "USD", "1m", start_date, end_date));
}

#[test]
fn test_main_succeeds_7d() {
    let start_date: i64 = Utc
        .ymd(2020, 11, 1)
        .and_hms_milli(0, 0, 1, 0)
        .timestamp_millis();
    let end_date: i64 = Utc
        .ymd(2021, 1, 1)
        .and_hms_milli(0, 0, 1, 0)
        .timestamp_millis();
    assert_eq!(true, get_data("BTC", "USD", "1m", start_date, end_date));
}

#[test]
fn test_main_succeeds_14d() {
    let start_date: i64 = Utc
        .ymd(2020, 11, 1)
        .and_hms_milli(0, 0, 1, 0)
        .timestamp_millis();
    let end_date: i64 = Utc
        .ymd(2021, 1, 1)
        .and_hms_milli(0, 0, 1, 0)
        .timestamp_millis();
    assert_eq!(true, get_data("BTC", "USD", "1m", start_date, end_date));
}

#[test]
fn test_main_succeeds_fails() {
    let start_date: i64 = Utc
        .ymd(2020, 11, 1)
        .and_hms_milli(0, 0, 1, 0)
        .timestamp_millis();
    let end_date: i64 = Utc
        .ymd(2021, 1, 1)
        .and_hms_milli(0, 0, 1, 0)
        .timestamp_millis();
    assert_ne!(
        true,
        get_data(
            "BTCssssssssssss",
            "USdfasdfsdafD",
            "103m",
            start_date,
            end_date
        )
    );
}
