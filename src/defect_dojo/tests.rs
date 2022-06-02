use crate::defect_dojo::API_TESTS;
use crate::Configuration;
use log::{debug, info};
use reqwest::header::AUTHORIZATION;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tests {
    pub count: u32,
    pub results: Vec<Test>,
    pub next: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Test {
    pub id: u32,
    pub engagement: u32,
    pub test_type_name: String,
    pub updated: String,
}

impl Tests {
    pub async fn retrieve_all(config: &Configuration) -> anyhow::Result<Tests> {
        let url = format!("{}{}", config.defect_dojo_url, API_TESTS);

        let mut tests: Vec<Test> = Vec::new();
        let mut partial_tests = Tests::retrieve_partial(&config, &url).await?;
        tests.append(&mut partial_tests.results);

        //TODO handle the ugly clone
        while let Some(next) = partial_tests.next.clone() {
            partial_tests = Tests::retrieve_partial(&config, &next).await?;
            tests.append(&mut partial_tests.results);
        }

        Ok(Tests {
            count: tests.len() as u32, //TODO move to usize
            results: tests,
            next: None,
        })
    }

    async fn retrieve_partial(config: &Configuration, url: &str) -> anyhow::Result<Tests> {
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
        debug!("Tests Retrieved: {}", body);
        let tests = Tests::parse(body)?;
        Ok(tests)
    }
}

trait ToTests {
    fn parse(raw: String) -> anyhow::Result<Tests>;
}

impl ToTests for Tests {
    fn parse(raw: String) -> anyhow::Result<Self> {
        let tests = serde_json::from_str::<Tests>(&*raw)?;
        Ok(tests)
    }
}
