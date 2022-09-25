pub mod balances;
pub mod order_book;
use std::sync::{Arc, Mutex};
pub use order_book::{OrderBook, OrderBookResults};
use balances::Balances;
use std::collections::{HashMap, VecDeque};


pub type ClientName = String;
pub type ActionWithOrder = char;
pub type CurencyName = String;
pub type Price = i32;
pub type Amount = i32;

pub struct Order {
    pub client: ClientName,
    pub action_with_order: ActionWithOrder,
    pub currency: CurencyName,
    pub price: Price,
    pub amount: Amount,
}

pub struct SuccessfulOrder {
    pub seller: ClientName,
    pub buyer: ClientName,
    pub currency: CurencyName,
    pub price: Price,
    pub amount: Amount,
}
#[derive(Debug, PartialEq)]
pub enum OrdersQueue {
    SellQueue(VecDeque<ClientName>),
    BuyQueue(VecDeque<ClientName>),
}

pub type Clients = HashMap<ClientName, Balances>;
pub type OrderBooks = HashMap<CurencyName, Arc<Mutex<OrderBook>>>;



pub fn create_order_books() -> OrderBooks {
    let mut s = HashMap::new();
    s.insert("A".to_string(), Arc::new(Mutex::new(OrderBook::new())));
    s.insert("B".to_string(), Arc::new(Mutex::new(OrderBook::new())));
    s.insert("C".to_string(), Arc::new(Mutex::new(OrderBook::new())));
    s.insert("D".to_string(), Arc::new(Mutex::new(OrderBook::new())));
    s
}