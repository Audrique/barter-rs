use std::error::Error;
use barter_execution::client::binance::BinanceSpotConfig;
fn main() -> Result<(), Box<dyn Error>> {
    let config = BinanceSpotConfig::new()?;
    println!("the config: {:?}", &config);
    Ok(())
}