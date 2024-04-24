use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use clap::Parser;
use futures::stream::iter;
use log::debug;
use tokio;

use crate::market::{Item, Market, Order, User};

mod defaults;
mod market;
mod prime_trash_buyer;
mod log_with_var;


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
    env_logger::init();
    let args = Args::parse();
    log_var!(args);

    let item_names_to_buy = get_item_names_to_buy(&args.item_names);

    let market = Market::new();
    let items = fetch_items(&market, &item_names_to_buy).await?;
    log_var!(items);

    let orders = filter_orders(&market, &args.min_quantity, &args.max_price_of_order, &items).await;

    process_orders(&args.max_price_to_offer, &args.output_file_path, &orders)?;

    Ok(())
}

fn get_item_names_to_buy(custom_item_names_of_user: &Option<Vec<String>>) -> Vec<String> {
    custom_item_names_of_user.clone().unwrap_or_else(|| {
        defaults::PRIME_TRASH_ITEM_NAMES.iter().map(|&s| s.to_string()).collect()
    })
}

async fn fetch_items(market: &Market, item_names: &[String]) -> anyhow::Result<Vec<Item>> {
    let items = market.fetch_items().await?;
    let filtered_items: Vec<_> = items
        .into_iter()
        .filter(|Item { name, .. }| item_names.contains(name))
        .collect();
    Ok(filtered_items)
}

async fn filter_orders(
    market: &Market,
    min_quantity: &usize,
    max_price_of_order: &usize,
    items: &[Item],
) -> HashMap<String, Vec<(Item, Order)>> {
    let mut orders: HashMap<String, Vec<(Item, Order)>> = HashMap::new();

    for item in items {
        let item_orders = market.fetch_orders_for_item(&item.url_id).await.unwrap_or_default();
        let filtered_orders: Vec<_> = item_orders
            .into_iter()
            .filter(|Order { quantity, platinum, user, r#type, .. }| {
                quantity >= min_quantity
                    && platinum <= max_price_of_order
                    && user.status == "ingame"
                    && r#type == "sell"
            })
            .collect();
        for order in filtered_orders {
            let Order { user, .. } = &order;
            match orders.entry(user.name.clone()) {
                Entry::Occupied(mut entry) => entry.get_mut().push((item.clone(), order)),
                Entry::Vacant(entry) => {
                    entry.insert(vec![(item.clone(), order)]);
                }
            }
        }
    }
    orders
}

fn process_orders(
    max_price_to_offer: &usize,
    output_file_path: &Option<String>,
    orders: &HashMap<String, Vec<(Item, Order)>>,
) -> anyhow::Result<()> {
    for (user, orders) in orders {
        log_var!(user);
        for (item, Order { platinum, quantity, .. }) in orders {
            let sum = quantity * platinum.min(max_price_to_offer);
            let message = format!("/w {user} Hi, {user}!\
                You have WTS order: {item} for {platinum} :platinum: for each on warframe.market. \
                I will buy all {quantity} pieces for {sum} :platinum: if you are interested :)",
                                  sum = sum,
                                  item = item.name,
                                  user = user);
            println!("{}", message);
            if let Some(output_file_path) = output_file_path {
                let mut file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open(output_file_path)?;
                file.write_all(message.as_bytes())?;
            }
        }
    }
    Ok(())
}