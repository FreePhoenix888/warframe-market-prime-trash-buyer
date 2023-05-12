use {
    crate::market::{Order, User},
    std::{cmp, string::ToString},
};

pub fn filter(order: &Order) -> bool {
    order.order_type == "sell"
        && order.user.status == "ingame"
        && order.platinum <= 5
        && order.quantity >= 5
}

pub fn message(order: &Order, sum: impl Fn(&Order) -> usize) -> String {
    let Order { user: User { name, .. }, platinum, quantity, item, .. } = order;
    format!(
        "/w {name} Hi, {name}! \
        You have WTS order: {item} for {platinum} :platinum: for each on warframe.market. \
        I will buy all {quantity} pieces for {sum} :platinum: if you are interested :)",
        item = item.as_ref().expect("TODO").name,
        sum = sum(&order),
    )
}

pub fn sum(order: &Order) -> usize {
    order.quantity * order.platinum.min(3)
}

pub const ITEMS_TO_BUY: [&str; 36] = [
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
