use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use clap::Parser;
use futures::stream::iter;
use tokio;

use crate::market::{Item, Market, Order, User};

mod defaults;
mod market;
mod prime_trash_buyer;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Item names to buy. If item name contains spaces, wrap it in quotes. Example: "Argon Crystal", "Forma Blueprint". Case sensitive. If is not specified - default item names will be used.
    #[clap(long, value_parser, num_args = 1.., value_delimiter = ',')]
    item_names: Option<Vec<String>>,

    /// Minimum quantity of items order must have. Orders with lower quantity will be ignored.
    #[arg(long, default_value_t = 3)]
    min_quantity: usize,

    /// Maximum price in platinum to offer. If order price is higher, this price will be offered. If order price is lower, order price will be used.
    #[arg(long, default_value_t = 3)]
    max_price_to_offer: usize,

    /// Maximum allowed price of order in platinum. Orders with higher price will be ignored.
    #[arg(long, default_value_t = 4)]
    max_price_of_order: usize,

    /// Path to file where messages will be saved
    #[arg(long)]
    output_file_path: Option<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    println!("{:?}", args);
    let item_names = args.item_names.unwrap_or(defaults::PRIME_TRASH_ITEM_NAMES.iter().map(|&s| s.to_string()).collect::<Vec<String>>());

    let market = Market::new();
    let items: Vec<_> = market
        .fetch_items()
        .await?
        .into_iter()
        .filter(|Item { name, .. }| item_names.contains(name))
        .collect();

    let mut orders = HashMap::<_, Vec<_>>::new();
    for item in &items {
        market
            .fetch_orders(&item.url_id)
            .await?
            .into_iter()
            .filter(| Order { quantity, platinum, user, r#type, .. }| {
                quantity >= &args.min_quantity
                    && platinum <= &args.max_price_of_order
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
            let message = format!("/w {user} Hi, {user}!\
               You have WTS order: {item} for {platinum} :platinum: for each on warframe.market. \
               I will buy all {quantity} pieces for {sum} :platinum: if you are interested :)",
                                  sum = quantity * platinum.min(args.max_price_to_offer),
                                  item = item.name);
            println!("{}", message);
            if let Some(output_file_path) = args.output_file_path.clone() {
                 OpenOptions::new()
                     .write(true)
                     .append(true)
                     .open(output_file_path)
                     .unwrap()
                     .write_all(message.as_bytes())
                     .unwrap();
            }
        }
    }

    Ok(())
}
