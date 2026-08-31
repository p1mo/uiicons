use std::{collections::HashMap, path::Path};

use crate::utils::config::{Config, Component, Icon};





pub fn hasher(buffer: &[u8]) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(buffer);
    hasher.finalize().to_hex().to_string()
}



pub fn dump_schema(schema_path: impl AsRef<Path>) -> std::io::Result<()> {
    if !schema_path.as_ref().exists() {
        let schema = schemars::schema_for!(Config);
        let json = serde_json::to_string_pretty(&schema)?;
        std::fs::write(schema_path, json)?;
    }
    Ok(())
}



pub fn dump_default(config_path: impl AsRef<Path>) -> std::io::Result<()> {
    if !config_path.as_ref().exists() {
        let schema = Config {
            schema: "gen/uiicons.schema.json".to_string(),
            component: Component {
                name: "ui-icon".into(),
                icon: "home".into(),
                size: "24px".into(),
                kind: "stroke".into(),
            },
            icons: {
                let mut map = HashMap::new();
                map.insert("home".into(), Icon {
                    kind : Some("outline".to_owned()),
                    source: "tabler".parse().unwrap(),
                    icon: "home".into()
                });
                map
            }
        };
        std::fs::write(config_path, serde_json::to_string_pretty(&schema)?)?;
    }
    Ok(())
}