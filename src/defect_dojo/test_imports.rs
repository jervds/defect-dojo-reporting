use crate::defect_dojo::API_TEST_IMPORT;
use crate::Configuration;
use log::{debug, info};
use reqwest::header::AUTHORIZATION;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TestImports {
    pub count: usize,
    pub results: Vec<TestImport>,
    pub next: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TestImport {
    pub id: u32,
    pub test: u32,
    pub findings_affected: Vec<u32>,
}

impl TestImports {
    pub async fn retrieve_all(config: &Configuration) -> anyhow::Result<TestImports> {
        let url = format!("{}{}", config.defect_dojo_url, API_TEST_IMPORT);

        let mut tests: Vec<TestImport> = Vec::new();
        let mut partial_tests = TestImports::retrieve_partial(config, &url).await?;
        tests.append(&mut partial_tests.results);

        //TODO handle the ugly clone
        while let Some(next) = partial_tests.next.clone() {
            partial_tests = TestImports::retrieve_partial(config, &next).await?;
            tests.append(&mut partial_tests.results);
        }

        Ok(TestImports {
            count: tests.len(),
            results: tests,
            next: None,
        })
    }

    async fn retrieve_partial(config: &Configuration, url: &str) -> anyhow::Result<TestImports> {
        info!("Querying {}", url);
        let client = reqwest::Client::new();
        let body = client
            .get(url)
            .header(AUTHORIZATION, format!("Token {}", config.defect_dojo_token))
            .query(&[("limit", "500")])
            .send()
            .await?
            .text()
            .await?;
        debug!("TestsImports Retrieved: {}", body);
        let tests = TestImports::parse(body)?;
        Ok(tests)
    }
}

trait ToTestImports {
    fn parse(raw: String) -> anyhow::Result<TestImports>;
}

impl ToTestImports for TestImports {
    //TODO Go generic
    fn parse(raw: String) -> anyhow::Result<Self> {
        let test_imports = serde_json::from_str::<Self>(&*raw)?;
        Ok(test_imports)
    }
}
