use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    channels: HashMap<String, Channel>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Channel {
    id: u64,
    enabled: bool,
    send_reminder: bool,
}

#[cfg(test)]
mod test_settings {
    use figment::{
        providers::{Format, Toml},
        Figment,
    };

    use super::*;

    #[test]
    fn serialization() {
        let config: Settings = Figment::new()
            .merge(Toml::file("settings/guild_116969616370040841.toml"))
            .extract()
            .unwrap();
    }
}
