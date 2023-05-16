use anyhow::Result;
use reqwest::Response;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use serde_json::Value;

const URL: &str = "https://api.warframe.market/v1";
const ITEMS: &str = "https://api.warframe.market/v1/items";

pub struct Market {
    pub client: reqwest::Client,
}

async fn payload_bridge<T: DeserializeOwned>(res: Response, bridge: &str) -> Result<T> {
    let bytes = res.bytes().await?;
    let value: Value = serde_json::from_slice(&bytes)?;

    Deserialize::deserialize(&value["payload"][bridge]).map_err(Into::into)
}

impl Market {
    pub fn new() -> Self {
        let client = reqwest::Client::new();
        Market { client }
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

#[derive(Serialize, Deserialize)]
pub struct ApiResponse<TPayload> {
    pub(crate) payload: TPayload,
}

#[derive(Serialize, Deserialize)]
pub struct Order {
    pub visible: bool,
    pub creation_date: String,
    pub quantity: usize,
    pub user: User,
    pub last_update: String,
    #[serde(rename = "platinum")]
    pub platinum_price: usize,
    pub order_type: String,
    pub platform: String,
    pub id: String,
    pub region: String,
    #[serde(skip)]
    pub item: Option<ItemsItem>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub reputation: usize,
    pub locale: String,
    pub avatar: Option<String>,
    #[serde(rename = "ingame_name")]
    pub name: String,
    pub last_seen: String,
    pub id: String,
    pub region: String,
    pub status: String,
}

#[derive(Serialize, Deserialize)]
pub struct OrdersPayload {
    pub orders: Vec<Order>,
}

pub type OrdersApiResponse = ApiResponse<OrdersPayload>;

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct ItemsItem {
    pub id: String,
    #[serde(rename = "url_name")]
    pub url_id: String,
    #[serde(rename = "item_name")]
    pub name: String,
    pub thumb: String,
    pub vaulted: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct ItemsPayload {
    pub items: Vec<ItemsItem>,
}

pub type ItemsApiResponse = ApiResponse<ItemsPayload>;