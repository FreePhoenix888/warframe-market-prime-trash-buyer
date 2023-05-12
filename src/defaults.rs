use crate::order::Order;

pub fn default_filter_order(order: &crate::market::Order) -> bool {
    order.order_type == "sell" &&
        order.user.status == "ingame" &&
        order.platinum <= 5 &&
        order.quantity >= 5
}

pub fn default_get_message(order: &Order, get_sum: &Box<dyn Fn(&Order) -> i32>) -> String {
    format!("/w {} Hi, {}! You have WTS order: {} for {} :platinum: for each on warframe.market. I will buy all {} pieces for {} :platinum: if you are interested :)",
            order.user.ingame_name, order.user.ingame_name, order.item.unwrap().item_name, order.platinum, order.quantity, get_sum(&order))
}

pub fn default_get_profitable_sum(order: &Order) -> i32 {
    order.quantity * (if order.platinum > 3 { 3 } else { order.platinum })
}
