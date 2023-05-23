use crate::market::Order;
use anyhow::Result;

pub fn filter(order: &crate::market::Order) -> bool {
    order.r#type == "sell" &&
        order.user.status == "ingame" &&
        order.platinum_price <= 5 &&
        order.quantity >= 5
}

// pub fn message(order: &Order, get_sum: &Box<dyn Fn(&Order) -> i32>) -> Result<String> {
//     let user_name = &order.user.name;
//     let item_name = order.item.clone().unwrap_or_default().name;
//     let platinum = order.platinum_price;
//     let quantity = order.quantity;
//     let sum = get_sum(&order);
//     Ok(
//         format!("/w {user_name} Hi, {user_name}! You have WTS order: {item_name} for {platinum} :platinum: for each on warframe.market. I will buy all {quantity} pieces for {sum} :platinum: if you are interested :)")
//     )
// }

pub fn sum(order: &Order) -> usize {
    order.quantity * order.platinum_price.min(3)
}

pub const ITEM_NAMES_TO_BUY: [&str; 36] = [
    "Harrow Prime Blueprint",
    "Astilla Prime Stock",
    "Braton Prime Receiver",
    "Knell Prime Receiver",
    "Corvas Prime Blueprint",
    "Magnus Prime Receiver",
    "Burston Prime Barrel",
    "Akbronco Prime Link",
    "Pandero Prime Barrel",
    "Nagantaka Prime Stock",
    "Scourge Prime Handle",
    "Tekko Prime Blueprint",
    "Orthos Prime Blueprint",
    "Zakti Prime Barrel",
    "Stradavar Prime Barrel",
    "Ninkondi Prime Chain",
    "Zakti Prime Barrel",
    "Ninkondi Prime Chain",
    "Afuris Prime Link",
    "Nidus Prime Blueprint",
    "Baza Prime Barrel",
    "Harrow Prime Neuroptics Blueprint",
    "Inaros Prime Chassis Blueprint",
    "Gara Prime Neuroptics Blueprint",
    "Karyst Prime Handle",
    "Tatsu Prime Blade",
    "Volnus Prime Head",
    "Redeemer Prime Blueprint",
    "Dethcube Prime Carapace",
    "Titania Prime Neuroptics Blueprint",
    "Guandao Prime Blueprint",
    "Garuda Prime Chassis Blueprint",
    "Panthera Prime Stock",
    "Khora Prime Chassis Blueprint",
    "Atlas Prime Chassis Blueprint",
    "Dual Keres Prime Blueprint",
];
