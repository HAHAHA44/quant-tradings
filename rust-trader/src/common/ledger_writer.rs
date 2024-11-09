use binance::account::Account;
use binance::market::Market;
use binance::model::Balance;

use crate::common::config::*;
use crate::common::symbol::*;

pub fn record_balance_to_ledger(account: &Account, market: &Market) {
    use chrono::Local;
    use std::fs::OpenOptions;
    use std::io::Write;

    match account.get_account() {
        Ok(account_info) => {
            let now = Local::now();
            let timestamp = now.format("%Y-%m-%d %H:%M:%S").to_string();
            
            let mut total_usdt_value = 0.0;
            let mut asset_count = 0;
            
            // Open balance file
            let mut balance_file = OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open(BALANCE_FILE)
                .expect("Cant open balance file");

            // Write balance file header
            if balance_file.metadata().unwrap().len() == 0 {
                writeln!(balance_file, "{}", BALANCE_HEADERS).expect("Cant write balance file header");
            }

            // Record detailed information for each asset
            for balance in account_info.balances {
                if is_active_symbol(&balance.asset) {
                    let free_amount = balance.free.parse::<f64>().unwrap_or(0.0);
                    let locked_amount = balance.locked.parse::<f64>().unwrap_or(0.0);
                    let total_amount = free_amount + locked_amount;

                    if total_amount > 0.0 {
                        let (usdt_price, usdt_value) = if balance.asset == "USDT" {
                            (1.0, total_amount)
                        } else {
                            let symbol = format!("{}USDT", balance.asset);
                            match market.get_price(&symbol) {
                                Ok(price_info) => (price_info.price, total_amount * price_info.price),
                                Err(_) => (0.0, 0.0),
                            }
                        };

                        // Write balance data
                        writeln!(
                            balance_file,
                            "{},{},{:.8},{:.4},{:.4}",
                            timestamp, balance.asset, total_amount, usdt_price, usdt_value
                        ).expect("Cant write balance data");

                        total_usdt_value += usdt_value;
                        asset_count += 1;
                    }
                }
            }

            // Open total value file
            let mut value_file = OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open(VALUE_FILE)
                .expect("Cant open or create total value file");

            // Write total value file header
            if value_file.metadata().unwrap().len() == 0 {
                writeln!(value_file, "{}", VALUE_HEADERS).expect("Cant write total value file header");
            }

            // Write total value data
            writeln!(
                value_file,
                "{},{:.4},{}",
                timestamp, total_usdt_value, asset_count
            ).expect("Cant write total value data");

            println!("Ledger updated - Total value: {:.2} USDT, Asset count: {}", total_usdt_value, asset_count);
        }
        Err(e) => println!("Failed to get account information: {:?}", e),
    }
}

pub fn write_balance_to_csv(balance: &Balance) {
    use chrono::Local;
    use std::fs::OpenOptions;
    use std::io::Write;

    let now = Local::now();
    let timestamp = now.format("%Y-%m-%d %H:%M:%S").to_string();

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("balances.csv")
        .expect("无法打开或创建 CSV 文件");

    if file.metadata().unwrap().len() == 0 {
        writeln!(file, "{}", CSV_HEADERS).expect("无法写入 CSV 文件头");
    }

    writeln!(
        file,
        "{},{},{},{}",
        timestamp, balance.asset, balance.free, balance.locked
    )
    .expect("无法写入余额数据到 CSV 文件");
}

pub fn print_and_write_active_balances(account: &Account) {
    match account.get_account() {
        Ok(account_info) => {
            println!("Current Account Balances:");
            println!("Asset\t\tFree Balance\t\tLocked Balance");
            println!("----------------------------------------");
            for balance in account_info.balances {
                if is_active_symbol(&balance.asset)
                    && (balance.free.parse::<f64>().unwrap_or(0.0) > 0.0
                        || balance.locked.parse::<f64>().unwrap_or(0.0) > 0.0)
                {
                    println!(
                        "{}\t\t{}\t\t{}",
                        balance.asset, balance.free, balance.locked
                    );

                    write_balance_to_csv(&balance);
                }
            }
        }
        Err(e) => println!("Failed to get account information: {:?}", e),
    }
}
