use std::{collections::HashMap, path::Path};

use super::sources::Source;

#[derive(Default, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct Config {
    #[serde(rename = "$schema")]
    pub schema: String,
    /// ### HTML Component for the icon.
    /// This HTML Componen uses `fetch` to retrive the icons
    /// ```js
    /// fetch("/icons/{icon_name}.svg")
    /// ```
    pub component: Component,
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

#[derive(Default, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct Component {
    /// ### Component Name
    /// > Info: all `Custom Component API` components require an `-` in the name
    /// Set your HTML Component name like `ui-icons`
    pub name: String,
    /// ### Default Icon
    /// Set your `icon` if the component HTML attribute is empty. Example `home`
    pub icon: String,
    /// ### Default Size
    /// Set your `size` if the component HTML attribute is empty. Example `24px`
    pub size: String,
    /// ### Default Kind
    ///Set your `kind` if the component HTML attribute is empty. Options are `fill, stroke`
    pub kind: String,
}

pub fn load_config(path: impl AsRef<Path>) -> std::io::Result<Config> {
    let mut cfg: Config = serde_json::from_str(&std::fs::read_to_string(path)?)?;
    cfg.icons.insert(
        "internal-error-404".into(),
        Icon {
            kind: Some("outline".into()),
            source: "tabler".parse().unwrap(),
            icon: "error-404".into(),
        },
    );
    Ok(cfg)
}
