use crate::primitives::{
    Clients, CurencyName, 
    Amount, balances::Balances
};
use std::collections::HashMap;
use std::fs::read_to_string;


pub fn parse_clients() -> Clients{
    let mut client_list: Clients = HashMap::new();
    let file = read_to_string("Clients.txt").expect("Without this file we cann't do orders");
    for i in file.split("\n") {
        let mut temp: Vec<String> = Vec::new();
        for j in i.split("\t") {
            temp.push(j.to_string());
        }

        let mut currencies: HashMap<CurencyName, Amount> = HashMap::new();
        currencies.insert("$".to_string(), temp[1].parse::<Amount>().unwrap());
        currencies.insert("A".to_string(), temp[2].parse::<Amount>().unwrap());
        currencies.insert("B".to_string(), temp[3].parse::<Amount>().unwrap());
        currencies.insert("C".to_string(), temp[4].parse::<Amount>().unwrap());
        currencies.insert("D".to_string(), temp[5].parse::<Amount>().unwrap());

        client_list.insert(temp[0].clone(), Balances::new(currencies));
    }
    client_list
}