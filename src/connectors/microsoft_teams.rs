use crate::defect_dojo::findings_summary::FindingsSummary;
use crate::Configuration;
use log::debug;

pub async fn post_report_per_finding(findings: &[FindingsSummary]) -> anyhow::Result<()> {
    let config = Configuration::load().unwrap_or_else(|| panic!("Error loading the configuration"));
    let client = reqwest::Client::new();
    let body = serde_json::to_string(&findings)?;
    debug!("Post url: {}", &config.findings_report_teams_url);
    debug!("Post request: {}", body);

    client
        .post(&config.findings_report_teams_url)
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await?;
    Ok(())
}
