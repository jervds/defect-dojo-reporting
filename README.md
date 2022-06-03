# defect-dojo-reporting
## Configuration
Configuration loaded from environment variables:

| Variable          | Description                                                                      | Example                       |
|-------------------|----------------------------------------------------------------------------------|:------------------------------|
| DEFECT_DOJO_TOKEN | The token being used to connect to defect dojo                                   | f1313211234566780b9316546900a |
| DEFECT_DOJO_URL   | The url on which defect dojo API are exposed                                     | https://www.defectdojo.sample |
| RUST_LOG          | The log level. See [env_log](https://docs.rs/env_logger/0.9.0/env_logger/) crate | info                          |

## Output
Currently, the output is only at log levels. It is structured as following:
1. A first bunch of data is generated, with a view per product:
```shell
product_name;last_scan_date;total_cve;cve_critical;cve_critical
```
2. A second bunch of data is generated, containing a view per CVE
```shell
cve;severity;number_of_occurences
```