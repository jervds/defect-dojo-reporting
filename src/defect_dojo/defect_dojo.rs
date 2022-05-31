use crate::defect_dojo::findings::{Finding, Findings};
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
            })
        });

        summary
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
                            Some(import) => {
                                let findings_ids = import
                                    .into_iter()
                                    .flat_map(|it| it.findings_affected)
                                    .flat_map(|id| {
                                        self.findings
                                            .clone()
                                            .into_iter()
                                            .filter(move |it| it.id == id)
                                    })
                                    .collect::<Vec<Finding>>();
                                findings_ids
                            }
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
            Some(engagement) => engagement.version.clone(),
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
