use crate::primitives::{
    Amount, Price, OrdersQueue,
    Order, SuccessfulOrder
};
use std::collections::{HashMap, VecDeque};
#[derive(Debug)]
pub struct OrderBook {
    pub orders: HashMap<(Price, Amount), OrdersQueue>,
}

pub enum OrderBookResults {
    AddedInOrderBook,
    SuccessfulOrder(SuccessfulOrder),
}

impl OrderBook {

    pub fn new() -> OrderBook {
        OrderBook{ orders: HashMap::new()}
    }
    
    pub fn do_order(&mut self, order: Order) -> OrderBookResults {
        match self.orders.get_mut(&(order.price, order.amount)) {
            Some(queue) => {
                match queue {
                    OrdersQueue::BuyQueue(queue) if order.action_with_order == 's' => {
                        let back_name = queue.pop_back().expect("I know, that if queue exist it isn't empty");
                        if queue.len() == 0 {
                            self.orders.remove(&(order.price, order.amount));
                        }
                        OrderBookResults::SuccessfulOrder(SuccessfulOrder { 
                            seller: order.client, 
                            buyer: back_name, 
                            currency: order.currency, 
                            price: order.price, 
                            amount: order.amount
                        })
                    },
                    OrdersQueue::SellQueue(queue) if order.action_with_order == 'b' => {
                        let back_name = queue.pop_back().expect("I know, that if queue exist it isn't empty");
                        if queue.len() == 0 {
                            self.orders.remove(&(order.price, order.amount));
                        }
                        OrderBookResults::SuccessfulOrder(SuccessfulOrder { 
                            seller: back_name, 
                            buyer: order.client, 
                            currency: order.currency, 
                            price: order.price, 
                            amount: order.amount
                        })
                    },
                    OrdersQueue::BuyQueue(queue) if order.action_with_order == 'b' => {
                        queue.push_front(order.client);
                        OrderBookResults::AddedInOrderBook
                    },
                    OrdersQueue::SellQueue(queue) if order.action_with_order == 's' => {
                        queue.push_front(order.client);
                        OrderBookResults::AddedInOrderBook
                    },
                    _ => {panic!("Doesn't expect this parametr")},
                }
            },
            None => {
                match order.action_with_order {
                    's' => {
                        self.orders.insert((order.price, order.amount), OrdersQueue::SellQueue(VecDeque::from([order.client])));
                    },
                    'b' => {
                        self.orders.insert((order.price, order.amount), OrdersQueue::BuyQueue(VecDeque::from([order.client])));
                    },
                    _ => {panic!("Doesn't expect this parametr")},
                }
                OrderBookResults::AddedInOrderBook
            },
        }
    }
}