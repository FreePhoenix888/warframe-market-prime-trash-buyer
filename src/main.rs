use clap::Parser;
use tokio;

mod market;
mod order;
mod prime_trash_buyer;
mod defaults;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Item names to buy
    #[arg(long)]
    item_names: Vec<String>,

    /// Minimum quantity of items order must have
    #[arg(long, default_value_t = 3)]
    quantity: u8,

    /// Maximum price in platinum
    #[arg(long, default_value_t = 3)]
    maximum_price: u8,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    Ok(())
}