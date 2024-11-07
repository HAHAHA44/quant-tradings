```
DayTickerEvent {
    // 事件类型，固定为 "24hrTicker"
    event_type: "24hrTicker",
    
    // 事件发生的时间戳（毫秒）
    event_time: 1730305854020,
    
    // 交易对名称
    symbol: "ARBUSDT",
    
    // 24小时价格变化（绝对值）
    price_change: "0.02090000",
    
    // 24小时价格变化百分比
    price_change_percent: "3.849",
    
    // 24小时加权平均价格
    average_price: "0.55228457",
    
    // 前一个收盘价
    prev_close: "0.54310000",
    
    // 最新价格（当前收盘价）
    current_close: "0.56390000",
    
    // 最新成交数量
    current_close_qty: "20.80000000",
    
    // 当前最高买价
    best_bid: "0.56390000",
    
    // 当前最高买价对应的数量
    best_bid_qty: "895.20000000",
    
    // 当前最低卖价
    best_ask: "0.56400000",
    
    // 当前最低卖价对应的数量
    best_ask_qty: "12633.80000000",
    
    // 24小时内的开盘价
    open: "0.54300000",
    
    // 24小时内的最高价
    high: "0.57000000",
    
    // 24小时内的最低价
    low: "0.53730000",
    
    // 24小时内的交易量（以基础货币计，如 ARB）
    volume: "87248777.30000000",
    
    // 24小时内的交易额（以计价货币计，如 USDT）
    quote_volume: "48186153.71317000",
    
    // 24小时统计的开始时间（毫秒）
    open_time: 1730219454020,
    
    // 24小时统计的结束时间（毫秒）
    close_time: 1730305854020,
    
    // 24小时内第一笔交易ID
    first_trade_id: 119258937,
    
    // 24小时内最后一笔交易ID
    last_trade_id: 119405080,
    
    // 24小时内的总成交笔数
    num_trades: 146144 
}
```


```
KlineEvent {
    event_type: "kline",
    event_time: 1730305854020,
    symbol: "BTCUSDT",
    kline: Kline {
        start_time: 1730305800000,    // K线开始时间
        end_time: 1730305859999,      // K线结束时间
        symbol: "BTCUSDT",
        interval: "1m",               // K线间隔
        first_trade_id: 123456,       // 这根K线期间第一笔成交ID
        last_trade_id: 123460,        // 这根K线期间最后一笔成交ID
        open: "50000.00",             // 开盘价
        close: "50100.00",            // 收盘价
        high: "50200.00",             // 最高价
        low: "49900.00",              // 最低价
        volume: "100.5",              // 成交量(基础货币)
        trade_count: 100,             // 成交笔数
        is_closed: false,             // 该K线是否已经完结
        quote_volume: "5050000.00",   // 成交额(计价货币)
        buy_volume: "60.5",           // 主动买入成交量
        quote_buy_volume: "3030000.00" // 主动买入成交额
    }
}
```