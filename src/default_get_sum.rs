use crate::order::Order;

pub fn default_get_profitable_sum(order: &Order) -> i32 {
    order.quantity *  (if order.platinum > 3 {3} else {order.platinum})
}