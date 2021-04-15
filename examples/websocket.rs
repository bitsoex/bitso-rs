extern crate bitsors;
use bitsors::websocket::*;

#[tokio::main]
async fn main() {
    let mut socket = BitsoWebSocket::new().await.unwrap();
    //subscribe to the BTC-MXN orders channel
    socket
        .subscribe(Subscription::Orders, Books::BtcMxn)
        .await
        .unwrap();
    // You can iterate over the Books and Subscription channels
    for book in Books::iter() {
        for subs in Subscription::iter() {
            socket.subscribe(subs, book).await.unwrap();
        }
    }
    loop {
        match socket.read().await.unwrap() {
            Response::Orders(r) => println!("{:?}", r),
            Response::Trades(r) => println!("{:?}", r),
            Response::DiffOrders(r) => println!("{:?}", r),
        }
    }
}
