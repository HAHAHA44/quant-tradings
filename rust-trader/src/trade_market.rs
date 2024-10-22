use binance::{config::Config, market::Market, model::KlineSummaries};

use crate::common::chart::plot_candlestick_chart;

pub fn get_klines(market: &Market, symbol: &str, interval: &str, limit: u16, start_time: Option<i64>) -> Result<KlineSummaries, binance::errors::Error> {
    market.get_klines(symbol, interval, Some(limit.into()), start_time.map(|t| t as u64), None)
}

pub fn get_klines_and_plot_candlestick_chart(market: &Market, symbol: &str, interval: &str, limit: u16, start_time: Option<i64>, output_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let klines = get_klines(market, symbol, interval, limit, start_time)?;
    // Extract Vec<Kline> from KlineSummaries
    let kline_vec = match klines {
        KlineSummaries::AllKlineSummaries(vec) => vec,
    };
    plot_candlestick_chart(&kline_vec, symbol, output_file)
}

