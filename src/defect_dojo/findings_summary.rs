pub struct FindingsSummary {
    // The cve identifier
    pub cve: String,
    // The number of projects impacted by the cve
    pub impacted_projects: usize,
    pub severity: String,
}
