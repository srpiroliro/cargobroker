use std::collections::HashMap;
use std::cmp::Ordering;

use nanoid::nanoid;
use rust_decimal::Decimal;



#[derive(Debug)]
pub enum OrderType {
    Bid, // buy
    Ask // sell
}

#[derive(Debug)]
pub struct Order {
    id: String,
    size: f32,
    side: OrderType
}

impl Order {
    pub fn new(size: f32, side:OrderType) -> Order {
        let id = nanoid!(32);
        Order { id , size , side }
    }

    fn is_filled(&mut self) -> bool {
        self.size == 0.0
    }

    fn fill_order(&mut self, order: &mut Order) {
        match self.size > order.size {
            true => {
                self.size -= order.size;
                order.size = 0.0;
            },
            false => {
                order.size -= self.size;
                self.size = 0.0;
            }
        }
    }
}


#[derive(Debug)]
struct LimitOrders {
    price: Decimal, // probably not needed. used to sort limit orders.
    orders: Vec<Order> // linkedlist? doesnt need to be sorted or searched.
}

impl LimitOrders {
    fn new(price: Decimal) -> LimitOrders {
        LimitOrders { price, orders: Vec::new() }
    }

    fn add_order(&mut self, order: Order) {
        self.orders.push(order)
    }

    fn fill_market_order(&mut self, market_order: &mut Order) {
        for order in self.orders {
            match order.size >= market_order.size {
                true => {
                    order.size -= market_order.size;
                    market_order.size = 0.0;
                },

                false => {
                    market_order.size -= order.size;
                    order.size = 0.0;
                }
            }
            if market_order.is_filled() {
                break;
            }
        }
    }

    fn total_volume(&self) -> f32 {
        self.orders.iter().map(|order| order.size).sum()
    }

    fn cmp(&self, other: &LimitOrders) -> Ordering {
        self.price.cmp(&other.price)
    }
}

#[derive(Debug)]
pub struct Orderbook {
    bids: HashMap<Decimal, LimitOrders>,
    asks: HashMap<Decimal, LimitOrders>
}

impl Orderbook {
    pub fn new() -> Orderbook {
        Orderbook { bids: HashMap::new(), asks: HashMap::new() }
    }

    pub fn add_limit_order(&mut self, price:Decimal, order: Order) {
        match order.side {
            OrderType::Bid => {
                match self.bids.get_mut(&price) {
                    Some(limit_orders) => limit_orders.add_order(order),

                    None => {
                        let mut limit = LimitOrders::new(price);
                        limit.add_order(order);
                        self.bids.insert(price, limit);
                    }
                }
            },

            OrderType::Ask => {
                if !self.asks.contains_key(&price){
                    self.asks.insert(price, LimitOrders::new(price));
                }

                self.asks.get_mut(&price).unwrap().add_order(order);
            }
        }
    }

    pub fn fill_market_order(&mut self, market_order: &mut Order, side: OrderType) {
        let mut limit_orders = match side {
            OrderType::Bid => self.sorted_bids(),
            OrderType::Ask => self.sorted_asks()
        };

        for limit_order in limit_orders {
            for order in limit_order.orders {
                order.fill_order(market_order);

                if market_order.is_filled() {
                    break;
                }
            }
        }
    }


    fn sorted_asks(&self) -> Vec<LimitOrders> {
        self.asks.values().collect().sort()
    }

    fn sorted_bids(&self) -> Vec<LimitOrders> {
        self.bids.values().collect().sort()
    }
}