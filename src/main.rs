mod matching_engine;
use matching_engine::Orderbook::{Orderbook, Order, OrderType};

use rust_decimal_macros::dec;

fn main() {
    let mut order_book = Orderbook::new();

    order_book.add_limit_order(dec!(20.1), Order::new(2.0, OrderType::Bid));
    order_book.add_limit_order(dec!(30), Order::new(4.0, OrderType::Bid));
    order_book.add_limit_order(dec!(11), Order::new(1.0, OrderType::Bid));

    order_book.add_limit_order(dec!(10.322), Order::new(1.0, OrderType::Ask));

    println!("{:?}", order_book);
}