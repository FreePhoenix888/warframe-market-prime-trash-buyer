use {
    anyhow::Result,
    reqwest::Response,
    serde::{de, Deserialize},
    serde_json::Value,
};

const URL: &str = "https://api.warframe.market/v1";
const ITEMS: &str = "https://api.warframe.market/v1/items";

pub struct Market {
    pub client: reqwest::Client,
}

async fn payload_bridge<T: de::DeserializeOwned>(res: Response, bridge: &str) -> Result<T> {
    let bytes = res.bytes().await?;
    let value: Value = serde_json::from_slice(&bytes)?;

    Deserialize::deserialize(&value["payload"][bridge]).map_err(Into::into)
}

impl Market {
    pub fn new() -> Self {
        Market { client: reqwest::Client::new() }
    }

    pub async fn fetch_items(&self) -> Result<Vec<Item>> {
        let res = self.client.get(ITEMS).send().await?;
        payload_bridge(res, "items").await
    }

    pub async fn fetch_orders(&self, item_url: &str) -> Result<Vec<Order>> {
        let url = format!("{ITEMS}/{item_url}/orders");
        let res = self.client.get(&url).send().await?;
        payload_bridge(res, "orders").await
    }
}

#[derive(Debug, Deserialize)]
pub struct Order {
    pub quantity: usize,
    pub user: User,
    pub platinum: usize,
    #[serde(rename = "order_type")]
    pub ty: String,
}

#[derive(Debug, Deserialize)]
pub struct User {
    #[serde(rename = "ingame_name")]
    pub name: String,
    pub status: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Item {
    pub id: String,
    #[serde(rename = "url_name")]
    pub url_id: String,
    #[serde(rename = "item_name")]
    pub name: String,
}
