use anyhow::Result;
use futures_util::{StreamExt, TryFutureExt};

use crate::default_filter_order::default_filter_order;
use crate::default_get_message::default_get_message;
use crate::default_get_sum::default_get_profitable_sum;
use crate::defaults::{filter, get_message, get_sum};
use crate::market;
use crate::market::Market;
use crate::order::Order;

pub struct PrimeTrashBuyer<'a> {
    warframe_market: &'a market::Market,
    filter_order: Box<dyn Fn(&crate::market::Order) -> bool>,
    get_profitable_sum: Box<dyn Fn(&Order) -> i32>,
    get_message: Box<dyn Fn(&Order, &Box<dyn Fn(&Order) -> i32>) -> String>,
}

impl PrimeTrashBuyer<'_> {
    pub fn new(
        warframe_market: &Market,
        filter_order: Option<Box<dyn Fn(&crate::market::Order) -> bool>>,
        get_profitable_sum: Option<Box<dyn Fn(&Order) -> i32>>,
        get_message: Box<dyn Fn(&Order, &Box<dyn Fn(&Order) -> i32>) -> String>,
    ) -> PrimeTrashBuyer {
        PrimeTrashBuyer {
            warframe_market,
            filter_order: filter_order.unwrap_or_else(|| Box::new(filter)),
            get_profitable_sum: get_profitable_sum.unwrap_or_else(|| Box::new(get_profitable_sum)),
            get_message: get_message.unwrap_or_else(|| Box::new(get_message)),
        }
    }

    pub async fn get_orders(&self, item: crate::market::ItemsItem) -> Result<Vec<Order>> {
        let orders_api_response = self.warframe_market.fetch_orders(item.url_name.to_string()).await?;
        let orders = orders_api_response.payload.orders;
        orders.into_iter()
            .filter(&self.filter_order)
            .map(|order| {
                let mut new_order = Order::from(order);
                new_order.item = Some(item.clone());
                Ok(new_order)
            })
            .collect()
    }

    pub async fn get_messages(&self, orders: Vec<Order>) -> Vec<String> {
        orders.into_iter().map(|order| {
            (self.get_message)(&order, &self.get_profitable_sum)
        }).collect()
    }
}