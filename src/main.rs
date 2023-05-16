use clap::Parser;
use tokio;

use crate::defaults::message;
use crate::market::{ItemsItem, Order};

mod defaults;
mod market;
mod prime_trash_buyer;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Item names to buy
    #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ',')]
    item_names: Vec<String>,

    /// Minimum quantity of items order must have
    #[arg(long, default_value_t = 3)]
    minimal_quantity: usize,

    /// Buy price in platinum that will be used in messages
    #[arg(long, default_value_t = 3)]
    buy_price: usize,

    /// Maximum price in platinum
    #[arg(long, default_value_t = 4)]
    maximum_price: usize,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    println!("{:?}", args);

    let market = market::Market::new();
    let items: Vec<_> = market
        .fetch_items()
        .await?
        .into_iter()
        .filter(|Item { name, .. }| args.items.contains(name))
        .collect();

    let mut all_orders: Vec<Order> = Vec::new();
    for item in items {
        let orders_response = market.fetch_orders(&item.url_id).await?;
        let mut orders: Vec<Order> = orders_response
            .payload
            .orders
            .into_iter()
            .filter(|order| {
                order.quantity >= args.minimal_quantity
                    && order.platinum_price <= args.maximum_price
                    && order.user.status == "ingame"
                    && order.order_type == "sell"
            })
            .map(|order| {
                let mut new_order = Order::from(order);
                new_order.item = Some(item.clone());
                new_order
            })
            .collect();
        all_orders.append(&mut orders)
    }
    for order in all_orders {
        let user_name = &order.user.name;
        let item_name = order.item.clone().unwrap_or_default().name;
        let platinum = order.platinum_price;
        let quantity = order.quantity;
        let sum = order.quantity * order.platinum_price.min(args.maximum_price);
        let message = format!("/w {user_name} Hi, {user_name}! You have WTS order: {item_name} for {platinum} :platinum: for each on warframe.market. I will buy all {quantity} pieces for {sum} :platinum: if you are interested :)");
        println!("{}", message);
    }
    Ok(())
}
