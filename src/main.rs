mod primitives;
mod processing;
mod transactions;
mod tests;
mod file_input_output;

use std::time::Instant;


use std::sync::{
    mpsc::{channel, Receiver, Sender},
};
use crate::primitives::{
    Clients, Order,
    create_order_books,
    SuccessfulOrder,
};
use crate::file_input_output::{
    output_clients_from_file::parse_clients, 
    input_clients_in_file::write_in_file_balances
};
use crate::processing::{
    parsing_orders, 
    order_processing, 
    client_processing
};


fn main() {
    let now = Instant::now();
    
    let mut clients: Clients = parse_clients();
    let mut order_books = create_order_books();
    
    let (sender_parsing_order, receiver_parsing_order): (Sender<Order>, Receiver<Order>) = channel();
    std::thread::spawn(move || {
        parsing_orders(sender_parsing_order)
    });

    let (sender_succes_order, receiver_succes_order): (Sender<SuccessfulOrder>, Receiver<SuccessfulOrder>) = channel();
    std::thread::spawn(move || {
        order_processing(receiver_parsing_order, sender_succes_order, &mut order_books)
    });

    client_processing(&mut clients, receiver_succes_order);

    write_in_file_balances(&mut clients);
    
    let elapsed_time = now.elapsed();
    println!("Running took {} millis.", elapsed_time.as_millis());
}
