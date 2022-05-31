# defect-dojo-reporting

## Target
Retrieve and aglomerate a list of CVE per product from defect dojo and play easily with the data.
1. Total number of CVE for master/main branch for all products retrieved
2. Total number of CVE per product on master/main branch without duplicates ?
3. CVE hit rate (number of CVE)

All those reports will be first written in a csv file.

## Configuration
Configuration loaded from environment variables:

| Variable          | Description                                                                      | Example                       |
|-------------------|----------------------------------------------------------------------------------|:------------------------------|
| DEFECT_DOJO_TOKEN | The token being used to connect to defect dojo                                   | f1313211234566780b9316546900a |
| DEFECT_DOJO_URL   | The url on which defect dojo API are exposed                                     | https://www.defectdojo.sample |
| RUST_LOG          | The log level. See [env_log](https://docs.rs/env_logger/0.9.0/env_logger/) crate | info                          |
