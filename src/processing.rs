use std::sync::mpsc::{Receiver, Sender};
use crate::primitives::{
    Clients, Order,
    OrderBooks,
    OrderBookResults,
    SuccessfulOrder,
};
use std::fs::read_to_string;
use threadpool::ThreadPool;
use num_cpus;
use rayon::prelude::*;
use crate::transactions::transaction;

pub fn parsing_orders(sender: Sender<Order>) {
    let file = read_to_string("Orders.txt").expect("Without this file we cann't do orders");
    let vec_of_slice: Vec<&str> = file.split("\n").collect();
    let vec_of_slice_len = vec_of_slice.len();
    vec_of_slice.par_chunks(vec_of_slice_len).for_each_with(sender, |sender, chunk| {
        chunk.into_iter().for_each(|row| {
            let mut temp: Vec<String> = Vec::new();
            row.split("\t").into_iter().for_each(|item| {
                temp.push(item.to_string());
            });
            sender.send(Order {
                client: temp[0].clone(),
                action_with_order: temp[1].chars().next().expect("Incorrectly entered input data"),
                currency: temp[2].clone(),
                price: temp[3].parse().expect("Incorrectly entered input data"),
                amount: temp[4].parse().expect("Incorrectly entered input data"),
            }).expect("Closed channel");
        });
        
    });
}

pub fn order_processing(receiver_parsing_order: Receiver<Order>, sender_succes_order: Sender<SuccessfulOrder>, order_books: &mut OrderBooks) {
        let pool = ThreadPool::new(num_cpus::get());
        loop {
            match receiver_parsing_order.recv() {
                Ok(order) => {
                    let r = order_books[&order.currency].clone();
                    let sender_succes_order = sender_succes_order.clone();

                    pool.execute(move || {
                        loop{
                            if let Ok(mut r) = r.lock() {
                                if let OrderBookResults::SuccessfulOrder(successful_order) = r.do_order(order) {
                                    sender_succes_order.send(successful_order).expect("Closed channel");
                                }
                                return
                            } else { continue }
                        }
                    })
                },
                Err(_) => {
                    return
                }
            }
        }
}

pub fn client_processing(clients: &mut Clients, receiver_succes_order: Receiver<SuccessfulOrder>) {
    loop {
        if let Ok(successful_order) = receiver_succes_order.recv() {
            transaction(clients, successful_order);
        } else { return }
    }
}
