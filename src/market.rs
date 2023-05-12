use anyhow::Result;
use clap::arg;
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};

const BASE_URL: Url = Url::parse("https://api.warframe.market/v1").unwrap();

pub struct Market {
    pub client: Client,
}

impl Market {
    pub fn new() -> Self {
        let client = Client::new();
        Market { client }
    }

    pub async fn fetch_items(&self) -> Result<ItemsApiResponse> {
        let url = BASE_URL.join("/items")?;
        let resp = self.client.get(url).send().await?;
        Ok(resp.json())
    }


    pub async fn fetch_orders(&self, item_url: String) -> Result<OrdersApiResponse> {
        let url = BASE_URL.join(&format!("/items/{}/orders", item_url))?;
        let resp = self.client.get(url).send().await?;
        let body = resp.text().await?;
        let json_resp: OrdersApiResponse = serde_json::from_str(&body)?;
        Ok(json_resp)
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
    pub quantity: i32,
    pub user: User,
    pub last_update: String,
    pub platinum: i32,
    pub order_type: String,
    pub platform: String,
    pub id: String,
    pub region: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub reputation: i32,
    pub locale: String,
    pub avatar: Option<String>,
    pub ingame_name: String,
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ItemsItem {
    pub id: String,
    pub url_name: String,
    pub item_name: String,
    pub thumb: String,
    pub vaulted: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct ItemsPayload {
    pub items: Vec<ItemsItem>,
}

pub type ItemsApiResponse = ApiResponse<ItemsPayload>;