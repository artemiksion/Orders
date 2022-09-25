use crate::primitives::OrderBookResults;
use crate::primitives::OrdersQueue;
use crate::primitives::balances::Balances;
use crate::Clients;
use crate::create_order_books;
use crate::Order;
use crate::transactions::transaction;
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use std::sync::Mutex;
//TODO
#[test]
fn test_buy_order() {
    //C1 sell currency A and C2 buy it
    let order_books = create_order_books();//Create order books

    let mut client_list: Clients = HashMap::new();    

    let mut balances = HashMap::new();
    balances.insert("$".to_string(), 1000);
    balances.insert("A".to_string(), 100);
    client_list.insert("C1".to_string(), Balances::new(balances));

    let mut balances = HashMap::new();
    balances.insert("$".to_string(), 1000);
    balances.insert("A".to_string(), 100);
    client_list.insert("C2".to_string(), Balances::new(balances));

    order_books[&"A".to_string()].lock().unwrap().do_order(Order {
        client: "C1".to_string(),
        action_with_order: 's',
        currency: "A".to_string(),
        price: 10,
        amount: 5,
    });

    if let OrderBookResults::SuccessfulOrder(successful_order) = order_books[&"A".to_string()].lock().unwrap().do_order(Order {
        client: "C2".to_string(),
        action_with_order: 'b',
        currency: "A".to_string(),
        price: 10,
        amount: 5,
    }) {
        transaction(&mut client_list, successful_order);
    } else {panic!("Not expected behavior")}


    assert_eq!(*client_list.get("C1").unwrap().0.get("$").unwrap(), 1050);
    assert_eq!(*client_list.get("C1").unwrap().0.get("A").unwrap(), 95);

    assert_eq!(*client_list.get("C2").unwrap().0.get("$").unwrap(), 950);
    assert_eq!(*client_list.get("C2").unwrap().0.get("A").unwrap(), 105);

}


#[test]
fn test_buy_order_with_many_clients() {
    //C1 sell currency A and C2 buy it
    let order_books = create_order_books();//Create order books

    let mut client_list: Clients = HashMap::new();    

    let mut balances = HashMap::new();
    balances.insert("$".to_string(), 1000);
    balances.insert("A".to_string(), 100);
    client_list.insert("C1".to_string(), Balances::new(balances));

    let mut balances = HashMap::new();
    balances.insert("$".to_string(), 1000);
    balances.insert("A".to_string(), 100);
    client_list.insert("C2".to_string(), Balances::new(balances));

    let mut balances = HashMap::new();
    balances.insert("$".to_string(), 1000);
    balances.insert("A".to_string(), 100);
    client_list.insert("C3".to_string(), Balances::new(balances));

    let mut balances = HashMap::new();
    balances.insert("$".to_string(), 1000);
    balances.insert("A".to_string(), 100);
    client_list.insert("C4".to_string(), Balances::new(balances));

    let mut balances = HashMap::new();
    balances.insert("$".to_string(), 1000);
    balances.insert("A".to_string(), 100);
    client_list.insert("C5".to_string(), Balances::new(balances));

    order_books[&"A".to_string()].lock().unwrap().do_order(Order {
        client: "C1".to_string(),
        action_with_order: 's',
        currency: "A".to_string(),
        price: 10,
        amount: 5,
    });

    order_books[&"A".to_string()].lock().unwrap().do_order(Order {
        client: "C2".to_string(),
        action_with_order: 's',
        currency: "A".to_string(),
        price: 10,
        amount: 5,
    });
    
    order_books[&"A".to_string()].lock().unwrap().do_order(Order {
        client: "C3".to_string(),
        action_with_order: 's',
        currency: "A".to_string(),
        price: 10,
        amount: 5,
    });

    order_books[&"A".to_string()].lock().unwrap().do_order(Order {
        client: "C4".to_string(),
        action_with_order: 's',
        currency: "A".to_string(),
        price: 10,
        amount: 5,
    });

    if let OrderBookResults::SuccessfulOrder(successful_order) = order_books[&"A".to_string()].lock().unwrap().do_order(Order {
        client: "C5".to_string(),
        action_with_order: 'b',
        currency: "A".to_string(),
        price: 10,
        amount: 5,
    }) {
        transaction(&mut client_list, successful_order);
    } else {panic!("Not expected behavior")}


    assert_eq!(*client_list.get("C1").unwrap().0.get("$").unwrap(), 1050);
    assert_eq!(*client_list.get("C1").unwrap().0.get("A").unwrap(), 95);

    assert_eq!(*client_list.get("C2").unwrap().0.get("$").unwrap(), 1000);
    assert_eq!(*client_list.get("C2").unwrap().0.get("A").unwrap(), 100);

    assert_eq!(*client_list.get("C3").unwrap().0.get("$").unwrap(), 1000);
    assert_eq!(*client_list.get("C3").unwrap().0.get("A").unwrap(), 100);

    assert_eq!(*client_list.get("C4").unwrap().0.get("$").unwrap(), 1000);
    assert_eq!(*client_list.get("C4").unwrap().0.get("A").unwrap(), 100);

    assert_eq!(*client_list.get("C5").unwrap().0.get("$").unwrap(), 950);
    assert_eq!(*client_list.get("C5").unwrap().0.get("A").unwrap(), 105);
    
    //
    let a = OrdersQueue::SellQueue(VecDeque::from(["C4".to_string(), "C3".to_string(), "C2".to_string()]));
    let b = Arc::new(Mutex::new(HashMap::from([((10, 5), a)])));
    let c = HashMap::from([("A".to_string(), b)]);
   
    assert_eq!(order_books.get("A").unwrap().lock().unwrap().orders.get(&(10, 5)).unwrap(), c.get("A").unwrap().lock().unwrap().get(&(10, 5)).unwrap());
}


#[test]
fn test_sell_order() {
    //C1 sell currency A and C2 buy it
    let order_books = create_order_books();//Create order books

    let mut client_list: Clients = HashMap::new();    

    let mut balances = HashMap::new();
    balances.insert("$".to_string(), 1000);
    balances.insert("A".to_string(), 100);
    client_list.insert("C1".to_string(), Balances::new(balances));

    let mut balances = HashMap::new();
    balances.insert("$".to_string(), 1000);
    balances.insert("A".to_string(), 100);
    client_list.insert("C2".to_string(), Balances::new(balances));

    order_books[&"A".to_string()].lock().unwrap().do_order(Order {
        client: "C1".to_string(),
        action_with_order: 'b',
        currency: "A".to_string(),
        price: 10,
        amount: 5,
    });

    if let OrderBookResults::SuccessfulOrder(successful_order) = order_books[&"A".to_string()].lock().unwrap().do_order(Order {
        client: "C2".to_string(),
        action_with_order: 's',
        currency: "A".to_string(),
        price: 10,
        amount: 5,
    }) {
        transaction(&mut client_list, successful_order);
    } else {panic!("Not expected behavior")}


    assert_eq!(*client_list.get("C1").unwrap().0.get("$").unwrap(), 950);
    assert_eq!(*client_list.get("C1").unwrap().0.get("A").unwrap(), 105);

    assert_eq!(*client_list.get("C2").unwrap().0.get("$").unwrap(), 1050);
    assert_eq!(*client_list.get("C2").unwrap().0.get("A").unwrap(), 95);
}


#[test]
fn test_sell_order_with_many_clients() {
    //C1 sell currency A and C2 buy it
    let order_books = create_order_books();//Create order books

    let mut client_list: Clients = HashMap::new();    

    let mut balances = HashMap::new();
    balances.insert("$".to_string(), 1000);
    balances.insert("A".to_string(), 100);
    client_list.insert("C1".to_string(), Balances::new(balances));

    let mut balances = HashMap::new();
    balances.insert("$".to_string(), 1000);
    balances.insert("A".to_string(), 100);
    client_list.insert("C2".to_string(), Balances::new(balances));

    let mut balances = HashMap::new();
    balances.insert("$".to_string(), 1000);
    balances.insert("A".to_string(), 100);
    client_list.insert("C3".to_string(), Balances::new(balances));

    let mut balances = HashMap::new();
    balances.insert("$".to_string(), 1000);
    balances.insert("A".to_string(), 100);
    client_list.insert("C4".to_string(), Balances::new(balances));

    let mut balances = HashMap::new();
    balances.insert("$".to_string(), 1000);
    balances.insert("A".to_string(), 100);
    client_list.insert("C5".to_string(), Balances::new(balances));

    order_books[&"A".to_string()].lock().unwrap().do_order(Order {
        client: "C1".to_string(),
        action_with_order: 'b',
        currency: "A".to_string(),
        price: 10,
        amount: 5,
    });

    order_books[&"A".to_string()].lock().unwrap().do_order(Order {
        client: "C2".to_string(),
        action_with_order: 'b',
        currency: "A".to_string(),
        price: 10,
        amount: 5,
    });
    
    order_books[&"A".to_string()].lock().unwrap().do_order(Order {
        client: "C3".to_string(),
        action_with_order: 'b',
        currency: "A".to_string(),
        price: 10,
        amount: 5,
    });

    order_books[&"A".to_string()].lock().unwrap().do_order(Order {
        client: "C4".to_string(),
        action_with_order: 'b',
        currency: "A".to_string(),
        price: 10,
        amount: 5,
    });

    if let OrderBookResults::SuccessfulOrder(successful_order) = order_books[&"A".to_string()].lock().unwrap().do_order(Order {
        client: "C5".to_string(),
        action_with_order: 's',
        currency: "A".to_string(),
        price: 10,
        amount: 5,
    }) {
        transaction(&mut client_list, successful_order);
    } else {panic!("Not expected behavior")}


    assert_eq!(*client_list.get("C1").unwrap().0.get("$").unwrap(), 950);
    assert_eq!(*client_list.get("C1").unwrap().0.get("A").unwrap(), 105);

    assert_eq!(*client_list.get("C2").unwrap().0.get("$").unwrap(), 1000);
    assert_eq!(*client_list.get("C2").unwrap().0.get("A").unwrap(), 100);

    assert_eq!(*client_list.get("C3").unwrap().0.get("$").unwrap(), 1000);
    assert_eq!(*client_list.get("C3").unwrap().0.get("A").unwrap(), 100);

    assert_eq!(*client_list.get("C4").unwrap().0.get("$").unwrap(), 1000);
    assert_eq!(*client_list.get("C4").unwrap().0.get("A").unwrap(), 100);

    assert_eq!(*client_list.get("C5").unwrap().0.get("$").unwrap(), 1050);
    assert_eq!(*client_list.get("C5").unwrap().0.get("A").unwrap(), 95);
    
    //
    let a = OrdersQueue::BuyQueue(VecDeque::from(["C4".to_string(), "C3".to_string(), "C2".to_string()]));
    let b = Arc::new(Mutex::new(HashMap::from([((10, 5), a)])));
    let c = HashMap::from([("A".to_string(), b)]);
   
    assert_eq!(order_books.get("A").unwrap().lock().unwrap().orders.get(&(10, 5)).unwrap(), c.get("A").unwrap().lock().unwrap().get(&(10, 5)).unwrap());
}

#[test]
fn form_orders() {
    let order_books = create_order_books();//Create order books

    let mut client_list: Clients = HashMap::new();    

    let mut balances = HashMap::new();
    balances.insert("$".to_string(), 1000);
    balances.insert("A".to_string(), 100);
    client_list.insert("C1".to_string(), Balances::new(balances));

    let mut balances = HashMap::new();
    balances.insert("$".to_string(), 1000);
    balances.insert("A".to_string(), 100);
    client_list.insert("C2".to_string(), Balances::new(balances));

    let mut balances = HashMap::new();
    balances.insert("$".to_string(), 1000);
    balances.insert("A".to_string(), 100);
    client_list.insert("C3".to_string(), Balances::new(balances));

    let mut balances = HashMap::new();
    balances.insert("$".to_string(), 1000);
    balances.insert("A".to_string(), 100);
    client_list.insert("C4".to_string(), Balances::new(balances));

    let mut balances = HashMap::new();
    balances.insert("$".to_string(), 1000);
    balances.insert("A".to_string(), 100);
    client_list.insert("C5".to_string(), Balances::new(balances));

    order_books[&"A".to_string()].lock().unwrap().do_order(Order {
        client: "C1".to_string(),
        action_with_order: 's',
        currency: "A".to_string(),
        price: 10,
        amount: 5,
    });

    if let OrderBookResults::SuccessfulOrder(successful_order) = order_books[&"A".to_string()].lock().unwrap().do_order(Order {
        client: "C2".to_string(),
        action_with_order: 'b',
        currency: "A".to_string(),
        price: 10,
        amount: 5,
    }) {
        transaction(&mut client_list, successful_order);
    } else {panic!("Not expected behavior")}
    
    order_books[&"A".to_string()].lock().unwrap().do_order(Order {
        client: "C3".to_string(),
        action_with_order: 'b',
        currency: "A".to_string(),
        price: 10,
        amount: 5,
    });

    if let OrderBookResults::SuccessfulOrder(successful_order) = order_books[&"A".to_string()].lock().unwrap().do_order(Order {
        client: "C4".to_string(),
        action_with_order: 's',
        currency: "A".to_string(),
        price: 10,
        amount: 5,
    }) {
        transaction(&mut client_list, successful_order);
    } else {panic!("Not expected behavior")}

    order_books[&"A".to_string()].lock().unwrap().do_order(Order {
        client: "C5".to_string(),
        action_with_order: 'b',
        currency: "A".to_string(),
        price: 10,
        amount: 5,
    });


    assert_eq!(*client_list.get("C1").unwrap().0.get("$").unwrap(), 1050);
    assert_eq!(*client_list.get("C1").unwrap().0.get("A").unwrap(), 95);

    assert_eq!(*client_list.get("C2").unwrap().0.get("$").unwrap(), 950);
    assert_eq!(*client_list.get("C2").unwrap().0.get("A").unwrap(), 105);

    assert_eq!(*client_list.get("C3").unwrap().0.get("$").unwrap(), 950);
    assert_eq!(*client_list.get("C3").unwrap().0.get("A").unwrap(), 105);

    assert_eq!(*client_list.get("C4").unwrap().0.get("$").unwrap(), 1050);
    assert_eq!(*client_list.get("C4").unwrap().0.get("A").unwrap(), 95);

    assert_eq!(*client_list.get("C5").unwrap().0.get("$").unwrap(), 1000);
    assert_eq!(*client_list.get("C5").unwrap().0.get("A").unwrap(), 100);
    
    //
    let a = OrdersQueue::BuyQueue(VecDeque::from(["C5".to_string()]));
    let b = Arc::new(Mutex::new(HashMap::from([((10, 5), a)])));
    let c = HashMap::from([("A".to_string(), b)]);
   
    assert_eq!(order_books.get("A").unwrap().lock().unwrap().orders.get(&(10, 5)).unwrap(), c.get("A").unwrap().lock().unwrap().get(&(10, 5)).unwrap());
}