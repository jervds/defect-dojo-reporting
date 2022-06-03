use crate::defect_dojo::API_ENGAGEMENTS;
use crate::Configuration;
use log::{debug, info};
use reqwest::header::AUTHORIZATION;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Engagements {
    pub count: u32,
    pub results: Vec<Engagement>,
    pub next: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Engagement {
    pub id: u32,
    pub name: String,
    pub version: String,
    pub product: u32,
}

impl Engagements {
    pub async fn retrieve_all(config: &Configuration) -> anyhow::Result<Engagements> {
        let url = format!("{}{}", config.defect_dojo_url, API_ENGAGEMENTS);

        let mut engagements: Vec<Engagement> = Vec::new();
        let mut partial_engagements = Engagements::retrieve_partial(config, &url, "master").await?;
        engagements.append(&mut partial_engagements.results);

        //TODO handle the ugly clone
        while let Some(next) = partial_engagements.next.clone() {
            partial_engagements = Engagements::retrieve_partial(config, &next, "master").await?;
            engagements.append(&mut partial_engagements.results);
        }

        partial_engagements = Engagements::retrieve_partial(config, &url, "main").await?;
        while let Some(next) = partial_engagements.next.clone() {
            partial_engagements = Engagements::retrieve_partial(config, &next, "main").await?;
            engagements.append(&mut partial_engagements.results);
        }

        Ok(Engagements {
            count: engagements.len() as u32, //TODO move to usize
            results: engagements,
            next: None,
        })
    }

    async fn retrieve_partial(
        config: &Configuration,
        url: &str,
        tag: &str,
    ) -> anyhow::Result<Engagements> {
        info!("Querying {}", url);
        let client = reqwest::Client::new();
        let body = client
            .get(url)
            .header(AUTHORIZATION, format!("Token {}", config.defect_dojo_token))
            .query(&[("limit", "500"), ("version", tag)])
            .send()
            .await?
            .text()
            .await?;
        debug!("Engagements Retrieved: {}", body);
        let engagements = Engagements::parse(body)?;
        Ok(engagements)
    }
}

trait ToEngagements {
    fn parse(raw: String) -> anyhow::Result<Engagements>;
}

impl ToEngagements for Engagements {
    fn parse(raw: String) -> anyhow::Result<Self> {
        let engagements = serde_json::from_str::<Engagements>(&*raw)?;
        Ok(engagements)
    }
}
