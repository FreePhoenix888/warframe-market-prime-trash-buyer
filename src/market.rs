use {
    anyhow::Result,
    serde::{
        de::{self, MapAccess, Visitor},
        ser::SerializeStruct,
        Deserialize, Deserializer, Serialize, Serializer,
    },
    std::{fmt, marker::PhantomData},
};

const URL: &str = "https://api.warframe.market/v1";
const ITEMS: &str = "https://api.warframe.market/v1/items";

pub struct Market {
    pub client: reqwest::Client,
}

impl Market {
    pub fn new() -> Self {
        Market { client: reqwest::Client::new() }
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

// fixme: use `Payload` manually in place
pub type OrdersApiResponse = Payload<OrdersPayload>;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ItemsItem {
    pub id: String,
    #[serde(rename = "url_name")]
    pub url: String,
    #[serde(rename = "item_name")]
    pub name: String,
    pub thumb: String,
    pub vaulted: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct ItemsPayload {
    pub items: Vec<ItemsItem>,
}

pub type ItemsApiResponse = Payload<ItemsPayload>;

pub struct Payload<R>(R);

impl<R: Serialize> Serialize for Payload<R> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut payload = serializer.serialize_struct("Payload", 1)?;
        payload.serialize_field("payload", &self.0)?;
        payload.end()
    }
}

impl<'de, R: Deserialize<'de>> Deserialize<'de> for Payload<R> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Payload,
        }

        struct PayloadVisitor<R> {
            _marker: PhantomData<R>,
        }

        impl<R> Default for PayloadVisitor<R> {
            fn default() -> Self {
                Self { _marker: PhantomData }
            }
        }

        impl<'de, R: Deserialize<'de>> Visitor<'de> for PayloadVisitor<R> {
            type Value = Payload<R>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Payload")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Payload<R>, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut payload = None;
                while map.next_key::<Field>()?.is_some(/* we have one field */) {
                    if payload.is_some() {
                        return Err(de::Error::duplicate_field("payload"));
                    }
                    payload = Some(map.next_value()?);
                }
                let payload = payload.ok_or_else(|| de::Error::missing_field("payload"))?;
                Ok(Payload(payload))
            }
        }

        deserializer.deserialize_struct("Payload", &["payload"], PayloadVisitor::default())
    }
}

#[test]
fn payload_transparency() {
    let json = serde_json::to_string(&Payload(b"test")).unwrap();
    let Payload::<[u8; 4]>([116, 101, 115, 116]) = serde_json::from_str(&json).unwrap() else {
        panic!()
    };
}
