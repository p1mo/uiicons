use std::collections::HashMap;











#[derive(Debug, serde::Deserialize)]
pub struct SourceInfo {
    #[serde(rename = "name")]
    pub _name: String,
    pub info: SourceDetails,
    pub kind: HashMap<String, String>
}

#[allow(unused)]
#[derive(Debug, serde::Deserialize)]
pub struct SourceDetails {
    pub website: String,
    pub github: Vec<String>,
}







#[derive(Default, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, speedy::Readable, speedy::Writable, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub enum Source {
    #[default]
    #[serde(rename = "bootstrap")]
    Bootstrap,
    #[serde(rename = "devicons")]
    DevIcons,
    #[serde(rename = "feather")]
    Feather,
    #[serde(rename = "google")]
    Google,
    #[serde(rename = "heroicons")]
    HeroIcons,
    #[serde(rename = "iconoir")]
    IconOir,
    #[serde(rename = "lucide")]
    Lucide,
    #[serde(rename = "phosphor")]
    Phosphor,
    #[serde(rename = "remixicon")]
    RemixIcon,
    #[serde(rename = "simpleicons")]
    SimpleIcons,
    #[serde(rename = "tabler")]
    Tabler,
    #[serde(rename = "thesvg")]
    TheSvg
}

impl std::str::FromStr for Source {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "bootstrap" => Ok(Self::Bootstrap),
            "devicons" => Ok(Self::DevIcons),
            "feather" => Ok(Self::Feather),
            "google" => Ok(Self::Google),
            "heroicons" => Ok(Self::HeroIcons),
            "iconoir" => Ok(Self::IconOir),
            "lucide" => Ok(Self::Lucide),
            "phosphor" => Ok(Self::Phosphor),
            "remixicon" => Ok(Self::RemixIcon),
            "simpleicons" => Ok(Self::SimpleIcons),
            "tabler" => Ok(Self::Tabler),
            "thesvg" => Ok(Self::TheSvg),
            _ => Err(format!("Unknown color: {s}")),
        }
    }
}

impl Source {
    
    pub fn get_info(&self) -> Result<SourceInfo, Error> {
        Ok(match self {
            Self::Bootstrap => serde_json::from_slice(include_bytes!("../../json/bootstrap.json"))?,
            Self::DevIcons => serde_json::from_slice(include_bytes!("../../json/devicons.json"))?,
            Self::Feather => serde_json::from_slice(include_bytes!("../../json/feather.json"))?,
            Self::Google => serde_json::from_slice(include_bytes!("../../json/google.json"))?,
            Self::HeroIcons => serde_json::from_slice(include_bytes!("../../json/heroicons.json"))?,
            Self::IconOir => serde_json::from_slice(include_bytes!("../../json/iconoir.json"))?,
            Self::Lucide => serde_json::from_slice(include_bytes!("../../json/lucide.json"))?,
            Self::Phosphor => serde_json::from_slice(include_bytes!("../../json/phosphor.json"))?,
            Self::RemixIcon => serde_json::from_slice(include_bytes!("../../json/remixicon.json"))?,
            Self::SimpleIcons => serde_json::from_slice(include_bytes!("../../json/simple-icons.json"))?,
            Self::Tabler => serde_json::from_slice(include_bytes!("../../json/tabler.json"))?,
            Self::TheSvg => serde_json::from_slice(include_bytes!("../../json/thesvg.json"))?,
        })
    }

}






#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    JsonDeserializeError(#[from] serde_json::Error),

    #[error("embeded file not found: {0}")]
    FileNotFoundError(String),
}