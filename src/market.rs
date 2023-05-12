use {
    anyhow::Result,
    clap::arg,
    reqwest::{Client, Url},
    serde::{Deserialize, Serialize},
};

const URL: &str = "https://api.warframe.market/v1";
const ITEMS: &str = "https://api.warframe.market/v1/items";

pub struct Market {
    pub client: Client,
}

impl Market {
    pub fn new() -> Self {
        let client = Client::new();
        Market { client }
    }

    pub async fn fetch_items(&self) -> Result<ItemsApiResponse> {
        Ok(self.client.get(ITEMS).send().await?.json().await?)
    }

    pub async fn fetch_orders(&self, item_url: String) -> Result<OrdersApiResponse> {
        let url = format!("{ITEMS}/{}/orders", item_url);
        Ok(self.client.get(&url).send().await?.json().await?)
    }

    // todo:
    // pub async fn orders(&self, item: crate::market::ItemsItem) -> Result<Vec<Order>> {
    //     let orders_api_response =
    //         self.warframe_market.fetch_orders(item.url_name.to_string()).await?;
    //     let orders = orders_api_response.payload.orders;
    //     orders
    //         .into_iter()
    //         .filter(&self.filter_order)
    //         .map(|order| {
    //             let mut new_order = Order::from(order);
    //             new_order.item = Some(item.clone());
    //             Ok(new_order)
    //         })
    //         .collect()
    // }
    //
    // pub async fn get_messages(&self, orders: Vec<Order>) -> Vec<String> {
    //     orders
    //         .into_iter()
    //         .map(|order| (self.get_message)(&order, &self.get_profitable_sum))
    //         .collect()
    // }
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
    pub platinum: usize,
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
    #[serde(rename = "ingameName")]
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ItemsItem {
    pub id: String,
    #[serde(rename = "urlName")]
    pub url: String,
    #[serde(rename = "itemName")]
    pub name: String,
    pub thumb: String,
    pub vaulted: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct ItemsPayload {
    pub items: Vec<ItemsItem>,
}

pub type ItemsApiResponse = ApiResponse<ItemsPayload>;
