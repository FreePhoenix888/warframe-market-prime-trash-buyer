use crate::market::Order;

pub fn default_filter_order(order: &Order) -> bool {
    order.order_type == "sell" &&
        order.user.status == "ingame" &&
        order.platinum <= 5 &&
        order.quantity >= 5
}