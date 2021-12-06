use super::model::websocket::*;
use std::str::FromStr;

pub use strum::{EnumCount, IntoEnumIterator};

use futures_util::{SinkExt, StreamExt};
use strum_macros::{AsRefStr, Display, EnumCount, EnumIter, EnumString};
use tokio::net::TcpStream;
use tokio_tungstenite::{
    tungstenite::{error::Result, Error, Message},
    MaybeTlsStream, WebSocketStream,
};

/// Bitso WebSocket object.
///
/// For more info see: <https://bitso.com/api_info?#websocket-api>
///
/// Check all the possible options in [`Books`] and [`Subscription`]
///
/// # Examples
/// ```no_run
/// use bitsors::websocket::*;
/// # #[tokio::main]
/// # async fn main() {
/// let mut socket = BitsoWebSocket::new().await.unwrap();
///
/// // You can subscribe to a specific orders channel
/// socket.subscribe(Subscription::Orders, Books::BtcMxn).await.unwrap();
///                                                         
/// // You can also iterate over all the Books and Subscription channels
/// for book in Books::iter() {
///     for subs in Subscription::iter() {
///         socket.subscribe(subs, book).await.unwrap();
///     }
/// }
///
/// loop {
///     match socket.read().await.unwrap() {
///         Response::Orders(r) => println!("{:?}", r),
///         Response::Trades(r) => println!("{:?}", r),
///         Response::DiffOrders(r) => println!("{:?}", r),
///     }
/// }
/// # }
/// ```

pub struct BitsoWebSocket {
    socket: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl BitsoWebSocket {
    /// Creates a new WebSocket connection.
    pub async fn new() -> Result<Self> {
        let (socket, _) = tokio_tungstenite::connect_async("wss://ws.bitso.com").await?;
        Ok(BitsoWebSocket { socket })
    }

    /// Closes an existing WebSocket connection.
    pub async fn close(&mut self) -> Result<()> {
        self.socket.close(None).await
    }

    /// Creates a subscription request to a given channel.
    pub async fn subscribe(
        &mut self,
        subscription_type: Subscription,
        book: Books,
    ) -> Result<String> {
        let request = format!(
            r#"{{"action":"subscribe","book":"{}","type":"{}"}}"#,
            book.as_ref(),
            subscription_type.as_ref()
        );
        self.socket.send(Message::Text(request)).await?;
        self.socket
            .next()
            .await
            .ok_or(Error::AlreadyClosed)??
            .into_text()
    }

    /// Reads the response from the WebSocket connection.
    pub async fn read(&mut self) -> Result<Response> {
        let mut response = self.socket.next().await.ok_or(Error::AlreadyClosed)??;
        let mut sub_type = response
            .to_text()?
            .trim_start_matches(r#"{"type":""#)
            .split('"')
            .next()
            .unwrap_or("");
        let mut subscription_type = Subscription::from_str(sub_type);

        // ignore keep alive and subscribe messages
        while subscription_type.is_err() {
            response = self.socket.next().await.ok_or(Error::AlreadyClosed)??;
            sub_type = response
                .to_text()?
                .trim_start_matches(r#"{"type":""#)
                .split('"')
                .next()
                .unwrap_or("");
            subscription_type = Subscription::from_str(sub_type);
        }

        let data = response.to_text()?;

        let response = match subscription_type.unwrap() {
            Subscription::Trades => Response::Trades(serde_json::from_str(data).unwrap()),
            Subscription::DiffOrders => Response::DiffOrders(serde_json::from_str(data).unwrap()),
            Subscription::Orders => Response::Orders(serde_json::from_str(data).unwrap()),
        };

        Ok(response)
    }
}

/// Represents the possible subscription responses in the Bitso WebSocket API.
#[derive(Debug, Clone, PartialEq, Display, AsRefStr, EnumCount)]
pub enum Response {
    ///[Trades Channel](https://bitso.com/api_info?#trades-channel)
    Trades(Trades),

    ///[Diff-Orders](https://bitso.com/api_info?#diff-orders)
    DiffOrders(DiffOrders),

    ///[Orders](https://bitso.com/api_info?#orders)
    Orders(Orders),
}

/// Represents the three subscription channels in the Bitso WebSocket API.
#[derive(Debug, Copy, Clone, PartialEq, Display, AsRefStr, EnumString, EnumIter)]
pub enum Subscription {
    ///[Trades Channel](https://bitso.com/api_info?#trades-channel)
    #[strum(serialize = "trades")]
    Trades,

    ///[Diff-Orders](https://bitso.com/api_info?#diff-orders)
    #[strum(serialize = "diff-orders")]
    DiffOrders,

    ///[Orders](https://bitso.com/api_info?#orders)
    #[strum(serialize = "orders")]
    Orders,
}

/// Represents all the possible books available in Bitso.
///
/// For more info, see: <https://bitso.com/api_info#available-books>
#[derive(Debug, Copy, Clone, PartialEq, Display, AsRefStr, EnumCount, EnumIter, EnumString)]
#[non_exhaustive]
pub enum Books {
    #[strum(serialize = "btc_mxn")]
    BtcMxn,
    #[strum(serialize = "eth_btc")]
    EthBtc,
    #[strum(serialize = "eth_ars")]
    EthArs,
    #[strum(serialize = "eth_mxn")]
    EthMxn,
    #[strum(serialize = "xrp_btc")]
    XrpBtc,
    #[strum(serialize = "xrp_mxn")]
    XrpMxn,
    #[strum(serialize = "ltc_btc")]
    LtcBtc,
    #[strum(serialize = "ltc_mxn")]
    LtcMxn,
    #[strum(serialize = "bch_btc")]
    BchBtc,
    #[strum(serialize = "bch_mxn")]
    BchMxn,
    #[strum(serialize = "tusd_btc")]
    TusdBtc,
    #[strum(serialize = "tusd_mxn")]
    TusdMxn,
    #[strum(serialize = "mana_btc")]
    ManaBtc,
    #[strum(serialize = "mana_mxn")]
    ManaMxn,
    #[strum(serialize = "bat_btc")]
    BatBtc,
    #[strum(serialize = "bat_mxn")]
    BatMxn,
    #[strum(serialize = "btc_ars")]
    BtcArs,
    #[strum(serialize = "btc_dai")]
    BtcDai,
    #[strum(serialize = "dai_mxn")]
    DaiMxn,
    #[strum(serialize = "btc_usd")]
    BtcUsd,
    #[strum(serialize = "xrp_usd")]
    XrpUsd,
    #[strum(serialize = "eth_usd")]
    EthUsd,
    #[strum(serialize = "dai_ars")]
    DaiArs,
    #[strum(serialize = "btc_brl")]
    BtcBrl,
    #[strum(serialize = "eth_brl")]
    EthBrl,
    #[strum(serialize = "btc_usdt")]
    BtcUsdt,
    #[strum(serialize = "mana_usd")]
    ManaUsd,
    #[strum(serialize = "ltc_usd")]
    LtcUsd,
    #[strum(serialize = "usd_mxn")]
    UsdMxn,
    #[strum(serialize = "usd_ars")]
    UsdArs,
    #[strum(serialize = "usd_brl")]
    UsdBrl,
    #[strum(serialize = "comp_usd")]
    CompUsd,
    #[strum(serialize = "link_usd")]
    LinkUsd,
    #[strum(serialize = "uni_usd")]
    UniUsd,
    #[strum(serialize = "aave_usd")]
    AaveUsd,
    #[strum(serialize = "chz_usd")]
    ChzUsd,
    #[strum(serialize = "axs_usd")]
    AxsUsd,
}
