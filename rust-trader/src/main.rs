use binance::api::*;
use binance::account::*;
use binance::market::*;


fn main() {

    let config_path = r"D:\Documents\QuantitativeTrading\python-trader\config.json";

    use std::fs::File;
    use std::io::Read;
    use serde_json::Value;

    let mut file = File::open(config_path).expect("无法打开配置文件");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("无法读取配置文件内容");

    let json: Value = serde_json::from_str(&contents).expect("无法解析 JSON 数据");

    let api_key = json["api_key"].as_str().expect("无法获取 API key").to_string();
    let api_secret = json["api_secret"].as_str().expect("无法获取 secret key").to_string();

    println!("API Key: {}", api_key);
    println!("Secret Key: {}", api_secret);

    let account: Account = Binance::new(Some(api_key), Some(api_secret));
    // let market: Market = Binance::new(Some(api_key), Some(secret_key));

    match account.get_account() {
        Ok(answer) => println!("{:?}", answer.balances),
        Err(e) => println!("Error: {:?}", e),
    }

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