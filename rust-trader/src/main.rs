mod common;
mod trade_market;

use binance::account::*;
use binance::api::*;
use binance::config::Config;
use binance::general::General;
use binance::market;
use binance::market::*;
use binance::model::Balance;
use trade_market::get_klines_and_plot_candlestick_chart;
use binance::websockets::*;
use std::sync::atomic::{AtomicBool};

const CSV_HEADERS: &str = "timestamp,asset,free_balance,locked_balance";
const ACTIVE_SYMBOLS: &[&str] = &[
    "PEPE", "BTC", "ETH", "BNB", "USDT", "ARB", "SOL", "SHIB", "DOGE", "OP", "ORDI",
];
// Implement a basic quantitative trading algorithm for PEPEUSDT
fn basic_pepe_trading_strategy(market: &Market, account: &Account) {
    println!("Starting basic quantitative trading strategy for PEPEUSDT");

    // Get current price of PEPEUSDT
    match market.get_price("PEPEUSDT") {
        Ok(price_info) => {
            let current_price = price_info.price;
            println!("Current PEPEUSDT price: {}", current_price);

            // Get account balance
            match account.get_account() {
                Ok(account_info) => {
                    let usdt_balance = account_info
                        .balances
                        .iter()
                        .find(|balance| balance.asset == "USDT")
                        .map(|balance| balance.free.parse::<f64>().unwrap_or(0.0))
                        .unwrap_or(0.0);

                    println!("Available USDT balance: {}", usdt_balance);

                    // Simple trading logic: Buy PEPE if USDT balance is greater than 10
                    if usdt_balance > 10.0 {
                        let quantity = (10.0 / current_price).floor(); // Buy quantity, rounded down
                        match account.market_buy("PEPEUSDT", quantity) {
                            Ok(buy_result) => println!("Buy successful: {:?}", buy_result),
                            Err(e) => println!("Buy failed: {:?}", e),
                        }
                    } else {
                        println!("Insufficient USDT balance, unable to buy");
                    }
                }
                Err(e) => println!("Failed to get account information: {:?}", e),
            }
        }
        Err(e) => println!("Failed to get PEPEUSDT price: {:?}", e),
    }
}

fn read_api_key_and_secret(config_path: &str) -> (String, String) {
    use serde_json::Value;
    use std::fs::File;
    use std::io::Read;

    let mut file = File::open(config_path).expect("无法打开配置文件");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("无法读取配置文件内容");

    let json: Value = serde_json::from_str(&contents).expect("无法解析 JSON 数据");

    let api_key = json["api_key"]
        .as_str()
        .expect("无法获取 API key")
        .to_string();
    let api_secret = json["api_secret"]
        .as_str()
        .expect("无法获取 secret key")
        .to_string();

    (api_key, api_secret)
}

fn write_balance_to_csv(balance: &Balance) {
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

fn print_all_balances(account: &Account) {
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

fn is_active_symbol(symbol: &str) -> bool {
    ACTIVE_SYMBOLS.contains(&symbol)
}


fn websocket_ticker() {
    let keep_running = AtomicBool::new(true); // Used to control the event loop
    let agg_trade = format!("!ticker@arr"); // All Symbols
    let mut web_socket = WebSockets::new(|event: WebsocketEvent| {
	match event {
        // 24hr rolling window ticker statistics for all symbols that changed in an array.
	    WebsocketEvent::DayTickerAll(ticker_events) => {
	        for tick_event in ticker_events {
		    if tick_event.symbol == "BTCUSDT" {
			let btcusdt: f32 = tick_event.average_price.parse().unwrap();
			let btcusdt_close: f32 = tick_event.current_close.parse().unwrap();
			println!("{} - {}", btcusdt, btcusdt_close);
		    }
		}
	    },
	    _ => (),
        };

        Ok(())
    });

    web_socket.connect(&agg_trade).unwrap(); // check error
    if let Err(e) = web_socket.event_loop(&keep_running) {
	match e {
	    err => {
	        println!("Error: {:?}", err);
	    }
	}
     }
}

fn main() {
    let (api_key, api_secret) = read_api_key_and_secret("./test-secret.json");

    let config = Config::default().set_rest_api_endpoint("https://testnet.binance.vision");

    let account: Account =
        Binance::new_with_config(Some(api_key.clone()), Some(api_secret.clone()), &config);

    print_all_balances(&account);
    // let market: Market = Binance::new(Some(api_key), Some(secret_key));

    let market: Market =
        Binance::new_with_config(Some(api_key.clone()), Some(api_secret.clone()), &config);

    let general: General =
        Binance::new_with_config(Some(api_key.clone()), Some(api_secret.clone()), &config);

    websocket_ticker();

    // Call this strategy in the main function
    // let market: Market = Binance::new(Some(api_key.clone()), Some(api_secret.clone()));
    // basic_pepe_trading_strategy(&market, &account);

    // match account.get_account() {
    //     Ok(answer) => println!("{:?}", answer.balances),
    //     Err(e) => println!("Error: {:?}", e),
    // }

    // // Order book at default depth
    // match market.get_depth("BNBETH") {
    //     Ok(answer) => println!("{:?}", answer),
    //     Err(e) => println!("Error: {}", e),
    // }

    // // Order book at depth 500
    // match market.get_custom_depth("BNBETH", 500) {
    //     Ok(answer) => println!("{:?}", answer),
    //     Err(e) => println!("Error: {}", e),
    // }

    // // Latest price for ALL symbols
    // match market.get_all_prices() {
    //     Ok(answer) => println!("{:?}", answer),
    //     Err(e) => println!("Error: {:?}", e),
    // }

    // // Latest price for ONE symbol
    // match market.get_price("BNBETH") {
    //     Ok(answer) => println!("{:?}", answer),
    //     Err(e) => println!("Error: {:?}", e),
    // }

    // // Current average price for ONE symbol
    // match market.get_average_price("BNBETH") {
    //     Ok(answer) => println!("{:?}", answer),
    //     Err(e) => println!("Error: {:?}", e),
    // }

    // // Best price/qty on the order book for ALL symbols
    // match market.get_all_book_tickers() {
    //     Ok(answer) => println!("{:?}", answer),
    //     Err(e) => println!("Error: {:?}", e),
    // }

    // // Best price/qty on the order book for ONE symbol
    // match market.get_book_ticker("BNBETH") {
    //     Ok(answer) => println!(
    //         "Bid Price: {}, Ask Price: {}",
    //         answer.bid_price, answer.ask_price
    //     ),
    //     Err(e) => println!("Error: {:?}", e),
    // }

    // // 24hr ticker price change statistics
    // match market.get_24h_price_stats("BNBETH") {
    //     Ok(answer) => println!(
    //         "Open Price: {}, Higher Price: {}, Lower Price: {:?}",
    //         answer.open_price, answer.high_price, answer.low_price
    //     ),
    //     Err(e) => println!("Error: {:?}", e),
    // }

    // // last 10 5min klines (candlesticks) for a symbol:
    // match market.get_klines("BNBETH", "5m", 10, None, None) {
    //     Ok(klines) => {
    //         match klines {
    //             binance::model::KlineSummaries::AllKlineSummaries(klines) => {
    //                 let kline: KlineSummary = klines[0].clone(); // You need to iterate over the klines
    //                 println!(
    //                     "Open: {}, High: {}, Low: {}",
    //                     kline.open, kline.high, kline.low
    //                 )
    //             }
    //         }
    //     },
    //     Err(e) => println!("Error: {}", e),
    // }
}
