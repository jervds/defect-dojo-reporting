use crate::defect_dojo::API_PRODUCTS;
use crate::Configuration;
use log::{debug, info};
use reqwest::header::AUTHORIZATION;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Products {
    pub count: u32,
    pub results: Vec<Product>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Product {
    pub id: u32,
    pub name: String,
}

impl Products {
    pub async fn retrieve_all(config: &Configuration) -> anyhow::Result<Products> {
        let url = format!("{}{}", config.defect_dojo_url, API_PRODUCTS);
        info!("Querying {}", url);
        let client = reqwest::Client::new();
        let body = client
            .get(url)
            .header(
                AUTHORIZATION,
                format!("Token {}", &config.defect_dojo_token),
            )
            .query(&[("limit", "1000")])
            .send()
            .await?
            .text()
            .await?;
        debug!("Retrieved: {}", body);
        let products = Products::parse(body)?;
        Ok(products)
    }
}

trait ToProducts {
    fn parse(raw: String) -> anyhow::Result<Products>;
}

impl ToProducts for Products {
    //TODO Go generic
    fn parse(raw: String) -> anyhow::Result<Self> {
        let products = serde_json::from_str::<Products>(&*raw)?;
        Ok(products)
    }
}
