use client::BybitClient;
use serde::{Deserialize, Serialize};
use servers::{FuturesUsdServer, SpotServer};

use crate::ApiCredentials;

mod client;
mod http;
mod servers;
mod types;
mod websocket;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Deserialize, Serialize)]
pub struct BybitConfig {
    pub credentials: ApiCredentials,
}

pub type BybitSpot = BybitClient<SpotServer>;

pub type BybitFuturesUsd = BybitClient<FuturesUsdServer>;
