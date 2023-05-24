use std::collections::hash_map::Entry;
use std::collections::HashMap;
use clap::Parser;
use tokio;

use crate::market::{Item, Market, Order, User};

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
    price: usize,

    /// Maximum price in platinum
    #[arg(long, default_value_t = 4)]
    max_price: usize,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    println!("{:?}", args);

    let market = Market::new();
    let items: Vec<_> = market
        .fetch_items()
        .await?
        .into_iter()
        .filter(|Item { name, .. }| args.items.contains(name))
        .collect();

    let mut orders = HashMap::<_, Vec<_>>::new();
    for item in &items {
        market
            .fetch_orders(&item.url_id)
            .await?
            .into_iter()
            .filter(| Order { quantity, platinum, user, r#type, .. }| {
                quantity >= &args.quantity
                    && platinum <= &args.max_price
                    && user.status == "ingame"
                    && r#type == "sell"
            })
            .for_each(|order| {
                let Order { user, .. } = &order; // isn't working in param match
                match orders.entry(user.name.clone()) {
                    Entry::Occupied(mut entry) => entry.get_mut().push((item, order)),
                    Entry::Vacant(entry) => {
                        entry.insert(Vec::from([(item, order)]));
                    }
                }
            });
    }

    for (user, orders) in orders {
        // fixme: should be configured by verbosity
        println!("Orders of `{user}`:");
        for (item, Order { platinum, quantity, .. }) in orders {
            println!(
                "  /w {user} Hi, {user}!\
               You have WTS order: {item} for {platinum} :platinum: for each on warframe.market. \
               I will buy all {quantity} pieces for {sum} :platinum: if you are interested :)",
                sum = quantity * platinum.min(args.max_price),
                item = item.name,
            );
        }
    }

    Ok(())
}
