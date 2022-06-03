use crate::defect_dojo::findings::Finding;

pub struct ProductSummary {
    pub name: String,
    pub version: String,
    pub last_scan_date: String,
    pub findings: Vec<Finding>,
}

impl ProductSummary {
    pub fn cve_high(&self) -> usize {
        self.cve_without_duplicates()
            .into_iter()
            .filter(|it| it.severity == "High")
            .filter(|it| !it.is_mitigated)
            .count()
    }

    pub fn cve_critical(&self) -> usize {
        self.cve_without_duplicates()
            .into_iter()
            .filter(|it| it.severity == "Critical")
            .filter(|it| !it.is_mitigated)
            .count()
    }

    pub fn total_cve(&self) -> usize {
        self.findings.len()
    }

    pub fn cve_without_duplicates(&self) -> Vec<Finding> {
        let mut list_cve = self
            .findings
            .clone()
            .into_iter()
            .map(|it| it.cve)
            .collect::<Vec<String>>();
        list_cve.sort();
        list_cve.dedup();
        let mut findings: Vec<Finding> = Vec::new();
        list_cve.into_iter().for_each(|it| {
            let finding = self
                .findings
                .clone()
                .into_iter()
                .filter(|f| f.cve == it)
                .collect::<Vec<Finding>>();
            findings.push(Finding {
                id: finding[0].id,
                cve: finding[0].cve.clone(),
                severity: finding[0].severity.clone(),
                is_mitigated: finding[0].is_mitigated,
            })
        });
        findings
    }
}
