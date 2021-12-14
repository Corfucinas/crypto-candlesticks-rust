//! Test quote currency is listed.

#[path = "../src/exchanges/mod.rs"]
mod exchanges;

#[path = "../src/symbols/list_of_currency.rs"]
mod list_of_currency;

use exchanges::bitfinex::Bitfinex;
use list_of_currency::LIST_OF_CURRENCY;

#[test]
fn test_quote_currency_succeeds() {
    let all_symbols: String = Bitfinex::new()
        .get_symbols()
        .expect("Could not call Bitfinex.");
    LIST_OF_CURRENCY.iter().for_each(|i| {
        assert_eq!(
            true,
            all_symbols.contains(&i.to_lowercase()),
            "Base asset not listed in Bitfinex."
        )
    });
}

#[test]
fn test_quote_currency_fails() {
    let all_symbols: String = Bitfinex::new()
        .get_symbols()
        .expect("Could not call Bitfinex.");
    assert_ne!(
        true,
        all_symbols.contains("this is not a currency!"),
        "The assets have change in Bitfinex, review."
    )
}
