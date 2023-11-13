use std::collections::HashMap;
use std::cmp::Ordering;

use nanoid::nanoid;
use rust_decimal::Decimal;



#[derive(Debug, Clone, PartialEq)]
pub enum OrderType {
    Bid, // buy
    Ask // sell
}

#[derive(Debug, Clone, PartialEq)]
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
pub struct PriceLevel {
    price: Decimal, // probably not needed. used to sort limit orders.
    orders: Vec<Order> // linkedlist? doesnt need to be sorted or searched.
}

impl PriceLevel {
    fn new(price: Decimal) -> PriceLevel {
        PriceLevel { price, orders: Vec::new() }
    }

    fn add_order(&mut self, order: Order) {
        self.orders.push(order)
    }

    fn fill_order(&mut self, target_order: &mut Order) {
        for order in self.orders.iter_mut() {
            order.fill_order(target_order);

            if target_order.is_filled() {
                break;
            }
        }
    }

    fn total_volume(&self) -> f32 {
        self.orders.iter().map(|order| order.size).sum()
    }

    fn cmp(&self, other: &PriceLevel) -> Ordering {
        self.price.cmp(&other.price)
    }
}

#[derive(Debug)]
pub struct Orderbook {
    bids: HashMap<Decimal, PriceLevel>,
    asks: HashMap<Decimal, PriceLevel>
}

impl Orderbook {
    pub fn new() -> Orderbook {
        Orderbook { bids: HashMap::new(), asks: HashMap::new() }
    }

    pub fn add_limit_order(&mut self, price: Decimal, order: Order) {
        // 2 solutions with the same result.
        match order.side {
            OrderType::Bid => {
                match self.bids.get_mut(&price) {
                    Some(limit_orders) => limit_orders.add_order(order),

                    None => {
                        let mut limit = PriceLevel::new(price);
                        limit.add_order(order);
                        self.bids.insert(price, limit);
                    }
                }
            },

            OrderType::Ask => {
                if !self.asks.contains_key(&price){
                    self.asks.insert(price, PriceLevel::new(price));
                }

                self.asks.get_mut(&price).unwrap().add_order(order);
            }
        }
    }

    pub fn fill_market_order(&mut self, market_order: &mut Order) {
        let price_levels = match market_order.side {
            OrderType::Bid => self.sorted_asks(),
            OrderType::Ask => self.sorted_bids()
        };

        for price_level in price_levels {
            price_level.fill_order(market_order);

            if market_order.is_filled() {
                println!("Market Order Filled!");
                break;
            }
        }

        // remove empty price levels
    }

    pub fn fill_limit_order(&mut self, limit_order: &mut Order, price: Decimal) {
        let price_levels = match limit_order.side {
            OrderType::Bid => &mut self.asks,
            OrderType::Ask => &mut self.bids
        };

        let price_level = match price_levels.get_mut(&price) {
            Some(price_level) => price_level,
            None => {
                self.add_limit_order(price, limit_order.clone()); 
                return
            }
        };

        price_level.fill_order(limit_order);

        if !limit_order.is_filled() {
            println!("Limit Order NOT Filled! {:?}", limit_order);

            self.add_limit_order(price, limit_order.clone());
        } else {
            println!("Limit Order Filled!");
        }
    }

    pub fn sorted_asks(&mut self) -> Vec<&mut PriceLevel> { // ascd
        let mut asks = self.asks.values_mut().collect::<Vec<&mut PriceLevel>>();
        asks.sort_by(|a,b| a.price.cmp(&b.price));

        asks
    }

    pub fn sorted_bids(&mut self) -> Vec<&mut PriceLevel> { // desc
        let mut bids = self.bids.values_mut().collect::<Vec<&mut PriceLevel>>();
        bids.sort_by(|a,b| b.price.cmp(&a.price));

        bids
    }
}