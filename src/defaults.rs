use std::string::ToString;
use crate::order::Order;

pub fn filter(order: &crate::market::Order) -> bool {
    order.order_type == "sell" &&
        order.user.status == "ingame" &&
        order.platinum <= 5 &&
        order.quantity >= 5
}

pub fn get_message(order: &Order, get_sum: &Box<dyn Fn(&Order) -> i32>) -> String {
    format!("/w {} Hi, {}! You have WTS order: {} for {} :platinum: for each on warframe.market. I will buy all {} pieces for {} :platinum: if you are interested :)",
            order.user.ingame_name, order.user.ingame_name, order.item.unwrap().item_name, order.platinum, order.quantity, get_sum(&order))
}

pub fn get_sum(order: &Order) -> i32 {
    order.quantity * (if order.platinum > 3 { 3 } else { order.platinum })
}

pub const ITEM_NAMES_TO_BUY: Vec<String> = vec![
    "Harrow Prime Blueprint".to_string(),
    "Astilla Prime Stock".to_string(),
    "Braton Prime Receiver".to_string(),
    "Knell Prime Receiver".to_string(),
    "Corvas Prime Blueprint".to_string(),
    "Magnus Prime Receiver".to_string(),
    "Burston Prime Barrel".to_string(),
    "Akbronco Prime Link".to_string(),
    "Pandero Prime Barrel".to_string(),
    "Nagantaka Prime Stock".to_string(),
    "Scourge Prime Handle".to_string(),
    "Tekko Prime Blueprint".to_string(),
    "Orthos Prime Blueprint".to_string(),
    "Zakti Prime Barrel".to_string(),
    "Stradavar Prime Barrel".to_string(),
    "Ninkondi Prime Chain".to_string(),
    "Zakti Prime Barrel".to_string(),
    "Ninkondi Prime Chain".to_string(),
    "Afuris Prime Link".to_string(),
    "Nidus Prime Blueprint".to_string(),
    "Baza Prime Barrel".to_string(),
    "Harrow Prime Neuroptics Blueprint".to_string(),
    "Inaros Prime Chassis Blueprint".to_string(),
    "Gara Prime Neuroptics Blueprint".to_string(),
    "Karyst Prime Handle".to_string(),
    "Tatsu Prime Blade".to_string(),
    "Volnus Prime Head".to_string(),
    "Redeemer Prime Blueprint".to_string(),
    "Dethcube Prime Carapace".to_string(),
    "Titania Prime Neuroptics Blueprint".to_string(),
    "Guandao Prime Blueprint".to_string(),
    "Garuda Prime Chassis Blueprint".to_string(),
    "Panthera Prime Stock".to_string(),
    "Khora Prime Chassis Blueprint".to_string(),
    "Atlas Prime Chassis Blueprint".to_string(),
    "Dual Keres Prime Blueprint".to_string(),
];
