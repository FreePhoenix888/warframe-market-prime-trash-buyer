use crate::order::Order;

pub fn default_get_message(order: &Order, get_sum: &Box<dyn Fn(&Order) -> i32>) -> String {
    format!("/w {} Hi, {}! You have WTS order: {} for {} :platinum: for each on warframe.market. I will buy all {} pieces for {} :platinum: if you are interested :)",
            order.user.ingame_name, order.user.ingame_name, order.item.unwrap().item_name, order.platinum, order.quantity, get_sum(&order))
}