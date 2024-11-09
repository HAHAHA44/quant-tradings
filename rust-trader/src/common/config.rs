pub const CSV_HEADERS: &str = "timestamp,asset,free_balance,locked_balance";
pub const ACTIVE_SYMBOLS: &[&str] = &[
    "PEPE", "BTC", "ETH", "ARB", // "SOL",
    "SHIB", "DOGE", // "OP",
    "ORDI",
];

pub const BALANCE_HEADERS: &str = "timestamp,asset,total_amount,usdt_price,usdt_value";
pub const VALUE_HEADERS: &str = "timestamp,total_usdt_value,asset_count";
pub const BALANCE_FILE: &str = "balance_details.csv";
pub const VALUE_FILE: &str = "total_value.csv";
    