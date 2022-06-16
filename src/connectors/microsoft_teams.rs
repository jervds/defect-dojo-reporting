use crate::defect_dojo::findings_summary::FindingsSummary;
use crate::Configuration;

pub async fn post_report_per_finding(findings: &[FindingsSummary]) -> anyhow::Result<()> {
    let config = Configuration::load().unwrap_or_else(|| panic!("Error loading the configuration"));
    let client = reqwest::Client::new();
    let mut body = "[".to_owned();
    findings.iter().clone().for_each(|it| {
        let body_tmp = format!(
            "{{ \"finding\":\"{}\", \"severity\":\"{}\", \"count\":\"{}\" }},",
            it.cve, it.severity, it.impacted_projects
        );
        body.push_str(&body_tmp);
    });
    client
        .post(&config.findings_report_teams_url)
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await?;
    Ok(())
}
