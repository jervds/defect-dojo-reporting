use crate::defect_dojo::findings::{Finding, Findings};
use crate::defect_dojo::findings_summary::FindingsSummary;
use crate::defect_dojo::product_summary::ProductSummary;
use crate::defect_dojo::products::Product;
use crate::{
    Configuration, Engagement, Engagements, Products, Test, TestImport, TestImports, Tests,
};
use log::{debug, info};

pub struct DefectDojo {
    pub products: Vec<Product>,
    pub engagements: Vec<Engagement>,
    pub tests: Vec<Test>,
    pub test_imports: Vec<TestImport>,
    pub findings: Vec<Finding>,
}

impl DefectDojo {
    pub async fn load() -> anyhow::Result<DefectDojo> {
        let config =
            Configuration::load().unwrap_or_else(|| panic!("Error loading the configuration"));

        let products = Products::retrieve_all(&config).await?;
        let engagements = Engagements::retrieve_all(&config).await?;
        let tests = Tests::retrieve_all(&config).await?;
        let test_imports = TestImports::retrieve_all(&config).await?;
        let findings = Findings::retrieve_all(&config).await?;

        Ok(DefectDojo {
            products: products.results,
            engagements: engagements.results,
            tests: tests.results,
            test_imports: test_imports.results,
            findings: findings.results,
        })
    }

    pub fn generate_product_summary(&self) -> Vec<ProductSummary> {
        let mut summary: Vec<ProductSummary> = Vec::new();
        self.products.iter().for_each(|it| {
            summary.push(ProductSummary {
                name: it.name.clone(),
                version: self.retrieve_version_for(it.id),
                findings: self.retrieve_findings_for(it.id),
                last_scan_date: self.retrieve_last_scan_date_for(it.id),
            })
        });

        summary
    }

    fn findings_from_tag(products: &[ProductSummary], tag: &str) -> Vec<Finding> {
        products
            .iter()
            .cloned()
            .flat_map(|it| it.findings)
            .filter(|it| it.severity == *tag)
            .collect::<Vec<Finding>>()
    }

    fn remove_duplicates(all_cve: &[Finding]) -> Vec<String> {
        let mut cve_without_duplicate = all_cve
            .iter()
            .cloned()
            .map(|it| it.cve)
            .collect::<Vec<String>>();
        cve_without_duplicate.sort();
        cve_without_duplicate.dedup();
        cve_without_duplicate
    }

    fn count_in_products(products: &[ProductSummary], cve: &str) -> usize {
        products
            .iter()
            .cloned()
            .filter(|it| it.has_cve(cve))
            .for_each(|it| info!("CVE {} found in {}", cve, it.name));

        products
            .iter()
            .cloned()
            .filter(|it| it.has_cve(cve))
            .count()
    }

    pub fn generate_cve_summary(&self, product_summary: &[ProductSummary]) -> Vec<FindingsSummary> {
        let mut all_cve: Vec<Finding> = Vec::new();
        all_cve.append(&mut DefectDojo::findings_from_tag(
            product_summary,
            "Critical",
        ));
        all_cve.append(&mut DefectDojo::findings_from_tag(product_summary, "High"));

        let cve_without_duplicate = DefectDojo::remove_duplicates(&all_cve);

        let mut findings_summary: Vec<FindingsSummary> = Vec::new();
        cve_without_duplicate.into_iter().for_each(|it| {
            findings_summary.push(FindingsSummary {
                finding: it.clone(),
                impacted_projects: DefectDojo::count_in_products(product_summary, &it),
                severity: DefectDojo::retrieve_severity(&all_cve, &it),
            })
        });
        findings_summary
    }

    fn retrieve_severity(findings: &[Finding], cve: &str) -> String {
        findings
            .iter()
            .cloned()
            .filter(|finding| finding.cve == cve)
            .collect::<Vec<Finding>>()
            .first()
            .unwrap()
            .severity
            .clone()
    }

    fn retrieve_last_scan_date_for(&self, product_id: u32) -> String {
        //TODO move to &str
        let maybe_engagement = self.retrieve_engagement_for(product_id);
        match maybe_engagement {
            None => "No engagement".to_string(),
            Some(engagement) => {
                let maybe_test = self.retrieve_test_for(engagement.id);
                match maybe_test {
                    None => "No test".to_string(),
                    Some(test) => test.updated,
                }
            }
        }
    }

    fn retrieve_findings_for(&self, product_id: u32) -> Vec<Finding> {
        let maybe_engagement = self.retrieve_engagement_for(product_id);
        match maybe_engagement {
            None => Vec::new(),
            Some(engagement) => {
                let maybe_test = self.retrieve_test_for(engagement.id);
                match maybe_test {
                    None => Vec::new(),
                    Some(test) => {
                        let maybe_imports = self.retrieve_test_import_for(test.id);
                        match maybe_imports {
                            None => Vec::new(),
                            Some(import) => import
                                .into_iter()
                                .flat_map(|it| it.findings_affected)
                                .flat_map(|id| {
                                    debug!("Product id: {} - engagment: {} - test {} - finding id: {} ",product_id, engagement.id, test.id, id);
                                    self.findings
                                        .clone()
                                        .into_iter()
                                        .filter(move |it| it.id == id)
                                })
                                .collect::<Vec<Finding>>()
                        }
                    }
                }
            }
        }
    }

    fn retrieve_test_import_for(&self, test_id: u32) -> Option<Vec<TestImport>> {
        let x = self
            .test_imports
            .clone()
            .into_iter()
            .filter(|it| it.test == test_id)
            .collect::<Vec<TestImport>>();
        if x.is_empty() {
            None
        } else {
            Some(x)
        }
    }

    fn retrieve_engagement_for(&self, product_id: u32) -> Option<Engagement> {
        let x = self
            .engagements
            .clone()
            .into_iter()
            .filter(|it| it.product == product_id)
            .collect::<Vec<Engagement>>();
        if x.is_empty() {
            None
        } else {
            Some(x[0].clone())
        }
    }

    fn retrieve_version_for(&self, product_id: u32) -> String {
        let maybe_engagement = self.retrieve_engagement_for(product_id);
        match maybe_engagement {
            None => "None".to_string(),
            Some(engagement) => engagement.version,
        }
    }

    fn retrieve_test_for(&self, engagement_id: u32) -> Option<Test> {
        let x = self
            .tests
            .clone()
            .into_iter()
            .filter(|it| it.engagement == engagement_id)
            .collect::<Vec<Test>>();
        if x.is_empty() {
            None
        } else {
            Some(x[0].clone())
        }
    }
}
