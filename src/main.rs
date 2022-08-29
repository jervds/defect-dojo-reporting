extern crate core;

use crate::config::configuration::Configuration;
use crate::connectors::microsoft_teams::post_report_per_finding;
use crate::defect_dojo::engagements::{Engagement, Engagements};
use crate::defect_dojo::mirror::DefectDojo;
use crate::defect_dojo::products::Products;
use crate::defect_dojo::test_imports::{TestImport, TestImports};
use crate::defect_dojo::tests::{Test, Tests};

mod config;
mod connectors;
mod defect_dojo;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let dojo = DefectDojo::load().await?;
    let product_summary = dojo.generate_product_summary();
    let cve_summary = dojo.generate_cve_summary(&product_summary);

    product_summary.clone().into_iter().for_each(|it| {
        println!(
            "{};{};{};{}",
            it.name,
            it.last_scan_date,
            it.cve_critical(),
            it.cve_high()
        );
    });

    cve_summary.clone().into_iter().for_each(|it| {
        println!("{};{};{}", it.finding, it.severity, it.impacted_projects);
    });

    post_report_per_finding(&cve_summary).await?;
    Ok(())
}
