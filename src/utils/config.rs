use std::{collections::HashMap, path::Path};

use super::sources::Source;

#[derive(Default, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct Config {
    #[serde(rename = "$schema")]
    pub schema: String,
    /// ### Icons of your choise
    /// The Map that holds your icons
    pub icons: HashMap<String, Icon>,
}

#[derive(Default, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct Icon {
    /// ### Sources
    /// | IconSet           | Website                           | Github                                            |
    /// |:------------------|:----------------------------------|:--------------------------------------------------|
    /// | tabler            | https://tabler.io                 | https://github.com/tabler/tabler-icons            |
    /// | simpleicons       | https://simpleicons.org           | https://github.com/simple-icons/simple-icons      |
    /// | feather           | https://feathericons.com/         | https://github.com/feathericons/feather           |
    /// | google            | https://fonts.google.com/icons    | https://github.com/google/material-design-icons   |
    /// | lucide            | https://lucide.dev                | https://github.com/lucide-icons/lucide            |
    /// | heroicons         | https://heroicons.com             | https://github.com/tailwindlabs/heroicons         |
    /// | bootstrap         | https://icons.getbootstrap.com    | https://github.com/twbs/icons                     |
    /// | remixicon         | https://remixicon.com             | https://github.com/Remix-Design/RemixIcon         |
    /// | iconoir           | https://iconoir.com               | https://github.com/iconoir-icons/iconoir          |
    /// | phosphor          | https://phosphoricons.com         | https://github.com/phosphor-icons/core            |
    /// | thesvg            | https://thesvg.org                | https://github.com/glincker/thesvg                |
    /// | devicons          | https://devicon.dev               | https://github.com/devicons/devicon               |
    pub source: Source,
    /// ### Icon Name
    /// Info: For icons like google (mdi) you need to specify the category like `action/123`
    pub icon: String,
    /// ### Icon Kind
    /// Set the icon kind. For example tabler has two kinds `filled, outline`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}

pub fn load_config(path: impl AsRef<Path>) -> std::io::Result<Config> {
    Ok(serde_json::from_str(&std::fs::read_to_string(path)?)?)
}
