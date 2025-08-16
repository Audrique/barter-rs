use crate::{
    balance::AssetBalance,
    client::ExecutionClient,
    error::UnindexedClientError,
    order::{state::Cancelled, state::Open, Order, request::OrderRequestCancel, request::OrderRequestOpen, request::UnindexedOrderResponseCancel},
    trade::Trade,
    ApiCredentials, InstrumentAccountSnapshot, UnindexedAccountEvent, UnindexedAccountSnapshot,
};
use barter_instrument::{
    asset::{name::AssetNameExchange, QuoteAsset},
    exchange::ExchangeId,
    instrument::name::InstrumentNameExchange,
};
use barter_integration::protocol::http::{
    private::{encoder::HexEncoder, RequestSigner},
    rest::client::RestClient,
};
use chrono::{DateTime, Utc};
use hmac::{Hmac, Mac};
use http::{
    parser::BinanceSpotHttpParser,
    signer::{BinanceSigner, BinanceSpotSigner},
};
use itertools::Itertools;
use tracing::warn;
use crate::client::binance::config::BinanceSpotConfig;
use crate::error::UnindexedOrderError;

pub mod config;
mod http;
mod model;
mod websocket;

#[derive(Debug, Clone)]
pub struct BinanceSpot {
    rest_client: RestClient<'static, BinanceSpotSigner, BinanceSpotHttpParser>,
    config: BinanceSpotConfig
}

impl ExecutionClient for BinanceSpot {
    const EXCHANGE: ExchangeId = ExchangeId::BinanceSpot;
    type Config = BinanceSpotConfig;
    type AccountStream = futures::stream::Empty<UnindexedAccountEvent>;

    fn new(config: Self::Config) -> Self {
        let hmac = Hmac::new_from_slice(config.api_secret.as_bytes())
            .expect("ApiCredentials secret invalid length");

        Self {
            rest_client: RestClient::new(
                config.http_base_url.clone(),
                RequestSigner::new(BinanceSigner::new(config.api_key.clone()), hmac, HexEncoder),
                BinanceSpotHttpParser,
            ),
            config
        }
    }

    async fn account_snapshot(
        &self,
        _: &[AssetNameExchange],
        instruments: &[InstrumentNameExchange],
    ) -> Result<UnindexedAccountSnapshot, UnindexedClientError> {
        let balances = self.fetch_balances().await?;

        let orders_by_instrument = self
            .fetch_open_orders()
            .await?
            .into_iter()
            .sorted_by(|a, b| a.key.instrument.cmp(&b.key.instrument))
            .chunk_by(|order| order.key.instrument.clone());

        let instruments = orders_by_instrument
            .into_iter()
            .fold(Vec::with_capacity(instruments.len()), |mut snapshots, (instrument, orders)| {
                if !instruments.contains(&instrument) {
                    warn!(
                        exchange = %Self::EXCHANGE,
                        %instrument,
                        "BinanceSpot | AccountSnapshot | received open_orders for untracked instrument - filtering"
                    );
                    return snapshots
                }

                snapshots.push(InstrumentAccountSnapshot {
                    instrument,
                    orders: orders
                        .into_iter()
                        .map(Order::from)
                        .collect(),
                });

                snapshots
            });

        Ok(UnindexedAccountSnapshot {
            exchange: Self::EXCHANGE,
            balances,
            instruments,
        })
    }

    async fn account_stream(
        &self,
        assets: &[AssetNameExchange],
        instruments: &[InstrumentNameExchange],
    ) -> Result<Self::AccountStream, UnindexedClientError> {
        todo!()
    }

    async fn cancel_order(
        &self,
        request: OrderRequestCancel<ExchangeId, &InstrumentNameExchange>,
    ) -> Option<UnindexedOrderResponseCancel> {
        todo!()
    }

    async fn open_order(
        &self,
        request: OrderRequestOpen<ExchangeId, &InstrumentNameExchange>,
    ) -> Option<
            Order<ExchangeId, InstrumentNameExchange, Result<Open, UnindexedOrderError>>,
        > {
        todo!()
    }

    async fn fetch_balances(
        &self,
    ) -> Result<Vec<AssetBalance<AssetNameExchange>>, UnindexedClientError> {
        todo!()
    }

    async fn fetch_open_orders(
        &self,
    ) -> Result<Vec<Order<ExchangeId, InstrumentNameExchange, Open>>, UnindexedClientError> {
        todo!()
    }

    async fn fetch_trades(
        &self,
        time_since: DateTime<Utc>,
    ) -> Result<Vec<Trade<QuoteAsset, InstrumentNameExchange>>, UnindexedClientError> {
        todo!()
    }
}
