use std::collections::HashMap;
use clap::Parser;
use tokio;

use crate::defaults::message;
use crate::market::{Item, Order, User};

mod defaults;
mod market;
mod prime_trash_buyer;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Item names to buy
    #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ',')]
    items: Vec<String>,

    /// Minimum quantity of items order must have
    #[arg(long, default_value_t = 3)]
    quantity: usize,

    /// Buy price in platinum that will be used in messages
    #[arg(long, default_value_t = 3)]
    buy_price: usize,

    /// Maximum price in platinum
    #[arg(long, default_value_t = 4)]
    max_price: usize,
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

    let mut orders: HashMap<&str, Vec<Order>> = HashMap::new();
    for item in &items {
        market
            .fetch_orders(&item.url_id)
            .await?
            .into_iter()
            .filter(|Order { quantity, platinum_price, user, order_type, .. }| {
                quantity >= &args.quantity
                    && platinum_price <= &args.max_price
                    && user.status == "ingame"
                    && order_type == "sell"
            })
            .for_each(|order| {
                #[rustfmt::skip]
                orders.entry(&item.name)
                    .and_modify(|orders| orders.push(order))
                    .or_default();
            });
    }

    for (item, orders) in orders {
        println!("`{item}`:");
        for Order { user: User { name: user, .. }, platinum_price, quantity, .. } in orders {
            println!(
                "  /w {user} Hi, {user}!\
               You have WTS order: {item} for {platinum_price} :platinum: for each on warframe.market. \
               I will buy all {quantity} pieces for {sum} :platinum: if you are interested :)",
                sum = quantity * platinum_price.min(args.max_price),
            );
        }
    }

    Ok(())
}
