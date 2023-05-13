use anyhow::Result;
use clap::arg;
use serde::{Deserialize, Serialize};
use crate::order::Order;

const BASE_URL: reqwest::Url = reqwest::Url::parse("https://api.warframe.market/v1").unwrap();

pub struct Market {
    pub client: reqwest::Client,
}

impl Market {
    pub fn new() -> Self {
        let client = reqwest::Client::new();
        Market { client }
    }

    pub async fn fetch_items(&self) -> Result<ItemsApiResponse> {
        Ok(self.client.get(ITEMS).send().await?.json().await?)
    }


    pub async fn fetch_orders(&self, item_url: &str) -> Result<OrdersApiResponse> {
        let url = format!("{ITEMS}/{item_url}/orders");
        Ok(self.client.get(&url).send().await?.json().await?)
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