use crate::defect_dojo::findings_summary::FindingsSummary;
use crate::Configuration;
use log::debug;

pub async fn post_report_per_finding(findings: &[FindingsSummary]) -> anyhow::Result<()> {
    let config = Configuration::load().unwrap_or_else(|| panic!("Error loading the configuration"));
    if config.findings_report_teams_url.is_none() {
        Ok(())
    } else {
        let client = reqwest::Client::new();
        let body = serde_json::to_string(&findings)?;
        let teams_post_url = &config.findings_report_teams_url.unwrap();
        debug!("Post url: {}", &teams_post_url);
        debug!("Post request: {}", body);

        client
            .post(teams_post_url)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await?;
        Ok(())
    }
}
