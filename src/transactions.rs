use crate::{Clients, primitives::SuccessfulOrder};

pub fn transaction(clients: &mut Clients, successful_order: SuccessfulOrder) {
    let seller = clients.get_mut(&successful_order.seller).expect("Client do order, but don't exist in client list");
    if let Err(_) = seller.reduce_currency_amount(&successful_order.currency, successful_order.amount) {return}
    if let Err(_) = seller.increase_currency_amount(&"$".to_string(), successful_order.amount * successful_order.price) {
        seller.increase_currency_amount(&successful_order.currency, successful_order.amount).expect("Cann't do reverse action");
        return;
    }
    let buyer = clients.get_mut(&successful_order.buyer).expect("Client do order, but don't exist in client list");
    if let Err(_) = buyer.reduce_currency_amount(&"$".to_string(), successful_order.amount * successful_order.price) {
        let seller = clients.get_mut(&successful_order.seller).expect("Client do order, but don't exist in client list");
        seller.increase_currency_amount(&successful_order.currency, successful_order.amount).expect("Cann't do reverse action");
        seller.reduce_currency_amount(&"$".to_string(), successful_order.amount * successful_order.price).expect("Cann't do reverse action");
        return
    }
    if let Err(_) = buyer.increase_currency_amount(&successful_order.currency, successful_order.amount) {
        buyer.increase_currency_amount(&"$".to_string(), successful_order.amount * successful_order.price).expect("Cann't do reverse action");
        let seller = clients.get_mut(&successful_order.seller).expect("Client do order, but don't exist in client list");
        seller.increase_currency_amount(&successful_order.currency, successful_order.amount).expect("Cann't do reverse action");
        seller.reduce_currency_amount(&"$".to_string(), successful_order.amount * successful_order.price).expect("Cann't do reverse action");
        return
    }
}