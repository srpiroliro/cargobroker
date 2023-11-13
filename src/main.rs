mod matching_engine;
use matching_engine::orderbook::{Orderbook, Order, OrderType};

use rust_decimal_macros::dec;

fn main() {
    let mut order_book = Orderbook::new();

    order_book.add_limit_order(dec!(20.1), Order::new(2.0, OrderType::Bid));
    order_book.add_limit_order(dec!(30), Order::new(4.0, OrderType::Bid));
    order_book.add_limit_order(dec!(11), Order::new(1.0, OrderType::Bid));
    order_book.add_limit_order(dec!(11), Order::new(3.0, OrderType::Bid));
    order_book.add_limit_order(dec!(11), Order::new(20.0, OrderType::Bid));

    order_book.add_limit_order(dec!(10.322), Order::new(1.0, OrderType::Ask));
    order_book.add_limit_order(dec!(10.322), Order::new(3.0, OrderType::Ask));
    order_book.add_limit_order(dec!(11.322), Order::new(20.0, OrderType::Ask));

    println!("{:?}\n", order_book);


    println!("{:?}\n", order_book.sorted_bids());


    println!("{:?}\n", order_book.sorted_asks());


    let mut order = Order::new(30.0, OrderType::Ask);

    order_book.fill_limit_order(&mut order, dec!(11));
    println!("{:?}\n", order_book.sorted_asks());
    println!("{:?}\n", order_book.sorted_bids());
}