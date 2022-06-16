use std::env;

use log::{debug, error};

use crate::config::{ENV_DEFECT_DOJO_TOKEN, ENV_DEFECT_DOJO_URL, ENV_FINDINGS_REPORT_TEAMS_URL};

pub struct Configuration {
    pub defect_dojo_url: String,
    pub defect_dojo_token: String,
    pub findings_report_teams_url: String,
}

impl Configuration {
    pub fn load() -> Option<Self> {
        match Configuration::from_env() {
            Ok(cfg) => {
                debug!("Configuration loaded:");
                debug!("defect_dojo_url : {} ", cfg.defect_dojo_url);
                debug!("defect_dojo_token : {} ", cfg.defect_dojo_token);
                Some(cfg)
            }
            Err(_) => {
                error!("Failed to load configuration");
                None
            }
        }
    }

    fn from_env() -> anyhow::Result<Configuration> {
        let _defect_dojo_token = env::var(ENV_DEFECT_DOJO_TOKEN)?;
        let _defect_dojo_url = env::var(ENV_DEFECT_DOJO_URL)?;
        let _findings_report_teams_url = env::var(ENV_FINDINGS_REPORT_TEAMS_URL)?;
        Ok(Configuration {
            defect_dojo_url: _defect_dojo_url,
            defect_dojo_token: _defect_dojo_token,
            findings_report_teams_url: _findings_report_teams_url,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_config_should_load_all_config_and_return_some() {
        env::set_var(ENV_DEFECT_DOJO_TOKEN, "123456");
        env::set_var(ENV_DEFECT_DOJO_URL, "https://www.blabla.com");
        env::set_var(ENV_FINDINGS_REPORT_TEAMS_URL, "https://www.blubla.com");
        assert_eq!(Configuration::load().is_some(), true);
        assert_eq!(Configuration::load().unwrap().defect_dojo_token, "123456");
        assert_eq!(
            Configuration::load().unwrap().defect_dojo_url,
            "https://www.blabla.com"
        );
        assert_eq!(
            Configuration::load().unwrap().findings_report_teams_url,
            "https://www.blubla.com"
        );
    }

    #[test]
    fn load_config_should_return_none_when_env_var_for_cfg_is_not_defined() {
        // if environment variable is not set, the remove_var can panic.
        env::set_var(ENV_DEFECT_DOJO_TOKEN, "123456");
        env::set_var(ENV_DEFECT_DOJO_URL, "https://www.blabla.com");
        env::remove_var(ENV_DEFECT_DOJO_TOKEN);
        assert_eq!(Configuration::load().is_none(), true)
    }
}
