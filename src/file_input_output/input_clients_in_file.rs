use crate::primitives::Clients;
use std::fs::write;

pub fn write_in_file_balances(client_list: &mut Clients) {
    let mut output = String::new();

    for i in client_list.iter() {
        let mut temp_output = String::new();
        temp_output.push_str(&i.0);
        temp_output.push_str("\t");

        temp_output.push_str(&format!("{}\t",&i.1.get_currency_amount("$".to_string())));
        temp_output.push_str(&format!("{}\t",&i.1.get_currency_amount("A".to_string())));
        temp_output.push_str(&format!("{}\t",&i.1.get_currency_amount("B".to_string())));
        temp_output.push_str(&format!("{}\t",&i.1.get_currency_amount("C".to_string())));
        temp_output.push_str(&format!("{}\n",&i.1.get_currency_amount("D".to_string())));
        output.push_str(&temp_output);
    }
    write("Result.txt", output.trim()).expect("Cann't create and write result.txt");
}