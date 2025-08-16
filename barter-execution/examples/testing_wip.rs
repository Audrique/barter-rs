use std::error::Error;
use barter_execution::client::binance::config::BinanceSpotConfig;
fn main() -> Result<(), Box<dyn Error>> {
    let config = BinanceSpotConfig::load()?;
    println!("the config: {:?}", &config);
    Ok(())
}