use std::collections::HashMap;


#[derive(Debug)]
enum OrderType {
    Bid, // buy
    Ask // sell
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Price {
    integral: u32, // int part of price
    fractional: u32, // decimal part of price
    scalar: u32, // 10^(-scalar) is the precision of the price
}
impl Price {
    fn new(price: f32) -> Price {
        let scalar = 10u32.pow(4);
        let integral = price as u32;
        let fractional = (price * scalar as f32) as u32;

        Price { integral, fractional, scalar}
    }
}


#[derive(Debug)]
struct Order {
    size: f32,
    side: OrderType
}

impl Order {
    fn new(size: f32, side:OrderType) -> Order {
        Order { size , side }
    }
}


#[derive(Debug)]
struct LimitOrders {
    // price: Price,  not necessary due to HashMap
    orders: Vec<Order>
}

impl LimitOrders {
    fn new() -> LimitOrders {
        LimitOrders { orders:Vec::new() }
    }
    
    // fn new(price: Price) -> LimitOrders {
    //     LimitOrders { price, orders:Vec::new() }
    // }

    fn add_order(&mut self, order: Order) {
        self.orders.push(order)
    }
}

#[derive(Debug)]
struct OrderBook {
    bids: HashMap<Price, LimitOrders>,
    asks: HashMap<Price, LimitOrders>
}

impl OrderBook {
    fn new() -> OrderBook {
        OrderBook { bids: HashMap::new(), asks: HashMap::new() }
    }

    fn add_order(&mut self, price:f32, order: Order) {
        let price = Price::new(price);

        match order.side {
            OrderType::Bid => {
                if !self.bids.contains_key(&price){
                    self.bids.insert(price.clone(), LimitOrders::new());
                }
                
                self.bids.get_mut(&price).unwrap().add_order(order);

            },
            OrderType::Ask => {
                if !self.asks.contains_key(&price){
                    self.asks.insert(price.clone(), LimitOrders::new());
                }
                
                self.asks.get_mut(&price).unwrap().add_order(order);
            }
        }
    }
}


fn main() {
    let price = Price::new(102.322);
    println!("{:?}", price);

    let order = Order::new(2.0, OrderType::Bid);

    let mut level_orders = LimitOrders::new();
    level_orders.add_order(order);

    println!("{:?}", level_orders);


    println!("\n"); 


    let mut order_book = OrderBook::new();

    order_book.add_order(102.322, Order::new(2.0, OrderType::Bid));
    order_book.add_order(110.322, Order::new(4.0, OrderType::Bid));
    order_book.add_order(110.322, Order::new(1.0, OrderType::Bid));

    order_book.add_order(110.322, Order::new(1.0, OrderType::Ask));


    println!("{:?}", order_book);
}