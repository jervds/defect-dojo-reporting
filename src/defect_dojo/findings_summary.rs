use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FindingsSummary {
    // The cve identifier
    pub finding: String,
    // The number of projects impacted by the cve
    pub impacted_projects: usize,
    pub severity: String,
}
