# defect-dojo-reporting
## Configuration
Configuration loaded from environment variables:

| Variable                  | Description                                                                      | Example                       |
|---------------------------|----------------------------------------------------------------------------------|:------------------------------|
| DEFECT_DOJO_TOKEN         | The token being used to connect to defect dojo                                   | f1313211234566780b9316546900a |
| DEFECT_DOJO_URL           | The url on which defect dojo API are exposed                                     | https://www.defectdojo.sample |
| RUST_LOG                  | The log level. See [env_log](https://docs.rs/env_logger/0.9.0/env_logger/) crate | info                          |
| FINDINGS_REPORT_TEAMS_URL | The url for webhook of teams. This variable is optional.                         | ...                           |

## Output
Currently, the output is only at log levels. It is structured as following:
1. A first bunch of data is generated, with a view per product:
```shell
product_name;last_scan_date;cve_critical;cve_high
```
2. A second bunch of data is generated, containing a view per CVE
```shell
cve;severity;number_of_occurences
```

## Exporting repport
The finding report can be exported with the following format:
```json
{
    "type": "array",
    "items": {
        "type": "object",
        "properties": {
            "finding": {
                "type": "string"
            },
            "severity": {
                "type": "string"
            },
            "impacted_projects": {
                "type": "integer"
            }
        },
        "required": [
            "finding",
            "impacted_projects",
            "severity"
        ]
    }
}
```