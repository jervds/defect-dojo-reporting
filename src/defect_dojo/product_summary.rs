use crate::defect_dojo::findings::Finding;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone)]
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
            .count()
    }

    pub fn cve_critical(&self) -> usize {
        self.cve_without_duplicates()
            .into_iter()
            .filter(|it| it.severity == "Critical")
            .count()
    }

    pub fn has_cve(&self, cve: &str) -> bool {
        self.findings
            .clone()
            .into_iter()
            .filter(|it| it.cve == cve)
            .count()
            > 0
    }

    pub fn total_cve(&self) -> usize {
        self.findings.len()
    }

    pub fn cve_without_duplicates(&self) -> Vec<Finding> {
        let mut list_cve = self
            .findings
            .iter()
            .cloned()
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
            })
        });
        findings
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cve_high_should_return_correct_number_of_cve() {
        assert_eq!(default_product_summary().cve_high(), 3)
    }

    #[test]
    fn cve_critical_should_return_correct_number_of_cve() {
        assert_eq!(default_product_summary().cve_critical(), 2)
    }

    #[test]
    fn has_cve_should_return_false_when_cve_is_not_in_list_of_findings() {
        assert_eq!(default_product_summary().has_cve("UNKNOWN"), false)
    }

    #[test]
    fn has_cve_should_return_true_when_cve_is_in_list_of_findings() {
        assert_eq!(default_product_summary().has_cve("CVE-SAMPLE-0001"), true)
    }

    #[test]
    fn cve_without_duplicate_should_remove_duplicates() {
        let findings_without_duplicate = default_product_summary().cve_without_duplicates();
        let cve_duplicated_count = findings_without_duplicate
            .iter()
            .map(|it| it.cve.clone())
            .filter(|it| it == "CVE-SAMPLE-0001")
            .count();
        assert_eq!(cve_duplicated_count, 1)
    }

    fn default_product_summary() -> ProductSummary {
        ProductSummary {
            name: String::from("sample product"),
            version: String::from("master"),
            last_scan_date: String::from("01-01-2019"),
            findings: default_findings_list(),
        }
    }

    fn default_findings_list() -> Vec<Finding> {
        vec![
            Finding {
                id: 0,
                cve: String::from("CVE-SAMPLE-0001"),
                severity: String::from("High"),
            },
            Finding {
                id: 1,
                cve: String::from("CVE-SAMPLE-0002"),
                severity: String::from("Critical"),
            },
            Finding {
                id: 2,
                cve: String::from("CVE-SAMPLE-0003"),
                severity: String::from("High"),
            },
            Finding {
                id: 3,
                cve: String::from("CVE-SAMPLE-0004"),
                severity: String::from("High"),
            },
            Finding {
                id: 4,
                cve: String::from("CVE-SAMPLE-0005"),
                severity: String::from("Critical"),
            },
            Finding {
                id: 5,
                cve: String::from("CVE-SAMPLE-0001"),
                severity: String::from("High"),
            },
        ]
    }
}
