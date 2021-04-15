use bitsors::websocket::*;

#[cfg(test)]
#[derive(serde_derive::Deserialize)]
pub struct BitsoBooks {
    pub payload: Vec<Book>,
}

#[cfg(test)]
#[derive(serde_derive::Deserialize)]
pub struct Book {
    #[serde(rename = "book")]
    pub name: String,
}

/// Test that enum Books has the same number of
/// variants as the current available books in Bitso.
/// Also check that the names match.
#[tokio::test]
async fn all_books_and_proper_name() {
    let current_books = reqwest::get("https://api.bitso.com/v3/available_books/")
        .await
        .unwrap()
        .json::<BitsoBooks>()
        .await
        .unwrap();

    assert_eq!(current_books.payload.len(), Books::COUNT);

    let mut current_books: Vec<&str> = current_books
        .payload
        .iter()
        .map(|b| b.name.as_str())
        .collect();
    current_books.sort_unstable();

    let mut enum_books: Vec<String> = Books::iter().map(|b| b.to_string()).collect();
    enum_books.sort_unstable();

    assert_eq!(current_books, enum_books);
}

/// Test WebSocket connection
#[tokio::test]
async fn test_websocket_new() {
    let _socket = BitsoWebSocket::new().await;
}

/// Test that we can close the WebSocket connection
#[tokio::test]
async fn test_websocket_close() {
    let mut socket = BitsoWebSocket::new().await.unwrap();
    socket.close().await.unwrap();
}

/// Test that we can subscribe to a channel
/// with a given WebSocket connection.
#[tokio::test]
async fn test_websocket_subscribe() {
    let mut socket = BitsoWebSocket::new().await.unwrap();
    socket
        .subscribe(Subscription::Orders, Books::BtcMxn)
        .await
        .unwrap();
}

/// Test that we can read from a given WebSocket connection.
#[tokio::test]
async fn test_websocket_read() {
    let mut socket = BitsoWebSocket::new().await.unwrap();
    socket
        .subscribe(Subscription::Orders, Books::BtcMxn)
        .await
        .unwrap();
    match socket.read().await.unwrap() {
        Response::Orders(r) => {
            println!("{:?}", r);
            assert_eq!(r.type_field, "orders");
            assert_eq!(r.book, "btc_mxn");
        }
        _ => panic!("Did not get a Response::Order"),
    }
}
