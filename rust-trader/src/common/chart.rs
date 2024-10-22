use plotters::prelude::*;
use chrono::{DateTime, Utc};
use binance::model::KlineSummary;

pub fn plot_candlestick_chart(klines: &[KlineSummary], symbol: &str, output_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(output_file, (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let (min_price, max_price) = klines.iter().fold((f64::MAX, f64::MIN), |acc, k| {
        (acc.0.min(k.low.parse::<f64>().unwrap()),
         acc.1.max(k.high.parse::<f64>().unwrap()))
    });

    let start_time = DateTime::<Utc>::from_timestamp(klines[0].open_time / 1000, 0).unwrap();
    let end_time = DateTime::<Utc>::from_timestamp(klines.last().unwrap().close_time / 1000, 0).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption(format!("{} 价格图", symbol), ("sans-serif", 40).into_font())
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(start_time..end_time, min_price..max_price)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(klines.iter().map(|k| {
        CandleStick::new(
            DateTime::<Utc>::from_timestamp(k.open_time / 1000, 0).unwrap(),
            k.open.parse::<f64>().unwrap(),
            k.high.parse::<f64>().unwrap(),
            k.low.parse::<f64>().unwrap(),
            k.close.parse::<f64>().unwrap(),
            &GREEN,
            &RED,
            15,
        )
    }))?;

    root.present()?;

    Ok(())
}