use crate::defect_dojo::API_FINDINGS;
use crate::Configuration;
use log::{debug, info};
use reqwest::header::AUTHORIZATION;
use serde::Serialize;
use serde::{Deserialize, Deserializer};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Findings {
    pub count: u32,
    pub results: Vec<Finding>,
    pub next: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Finding {
    pub id: u32,
    #[serde(deserialize_with = "deserialize_null_default")]
    pub cve: String,
    pub severity: String,
    pub is_mitigated: bool,
}

fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + Deserialize<'de>,
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}

impl Findings {
    pub async fn retrieve_all(config: &Configuration) -> anyhow::Result<Findings> {
        let url = format!("{}{}", config.defect_dojo_url, API_FINDINGS);
        let mut findings: Vec<Finding> = Vec::new();
        let mut partial_findings = Findings::retrieve_partial(&config, &url).await?;
        findings.append(&mut partial_findings.results);

        //TODO handle the ugly clone
        while let Some(next) = partial_findings.next.clone() {
            partial_findings = Findings::retrieve_partial(&config, &next).await?;
            findings.append(&mut partial_findings.results);
        }

        Ok(Findings {
            count: findings.len() as u32, //TODO move to usize
            results: findings,
            next: None,
        })
    }

    async fn retrieve_partial(config: &Configuration, url: &str) -> anyhow::Result<Findings> {
        info!("Querying {}", url);
        let client = reqwest::Client::new();
        let body = client
            .get(url)
            .header(
                AUTHORIZATION,
                format!("Token {}", &config.defect_dojo_token),
            )
            .query(&[("limit", "500")])
            .send()
            .await?
            .text()
            .await?;
        debug!("Findings Retrieved: {}", body);
        let findings = Findings::parse(body)?;
        Ok(findings)
    }
}

trait ToFindings {
    fn parse(raw: String) -> anyhow::Result<Findings>;
}

impl ToFindings for Findings {
    //TODO Go generic
    fn parse(raw: String) -> anyhow::Result<Self> {
        let findings = serde_json::from_str::<Self>(&*raw)?;
        Ok(findings)
    }
}
