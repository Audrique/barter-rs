use std::error::Error;

use barter_execution::client::binance::config::BinanceSpotConfig;

fn main() -> Result<(), Box<dyn Error>> {
    let path = "C:/Users/Lenovo/RustroverProjects/barter-rs/barter-execution/src/client/binance/config";
    let config = BinanceSpotConfig::load_from_dir(path)?;
    println!("the config: {:?}", &config);
    Ok(())
}