use crate::config::configuration::Configuration;
use crate::defect_dojo::engagements::{Engagement, Engagements};
use crate::defect_dojo::mirror::DefectDojo;
use crate::defect_dojo::products::Products;
use crate::defect_dojo::test_imports::{TestImport, TestImports};
use crate::defect_dojo::tests::{Test, Tests};

mod config;
mod defect_dojo;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let dojo = DefectDojo::load().await?;
    let product_summary = dojo.generate_product_summary();

    product_summary.iter().cloned().for_each(|it| {
        println!(
            "{};{};{};{};{}",
            it.name,
            it.last_scan_date,
            it.total_cve(),
            it.cve_critical(),
            it.cve_high()
        );
    });

    dojo.generate_cve_summary(&product_summary)
        .into_iter()
        .for_each(|it| {
            println!("{};{};{}", it.cve, it.severity, it.impacted_projects);
        });
    Ok(())
}
