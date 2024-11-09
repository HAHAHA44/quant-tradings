use crate::common::config::*;

pub fn get_default_pair_symbol() -> Vec<String> {
    ACTIVE_SYMBOLS
        .iter()
        .map(|symbol| format!("{}{}", symbol.to_lowercase(), "usdt"))
        .collect::<Vec<String>>()
}

pub fn is_active_symbol(symbol: &str) -> bool {
    ACTIVE_SYMBOLS.contains(&symbol)
}
