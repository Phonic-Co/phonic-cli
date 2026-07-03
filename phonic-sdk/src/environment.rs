use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultUrls {
    pub base: String,
    pub production: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Environment {
    Default(DefaultUrls),
}
impl Environment {
    pub fn url(&self) -> &str {
        match self {
            Self::Default(urls) => &urls.base,
        }
    }

    pub fn base_url(&self) -> &str {
        match self {
            Self::Default(urls) => &urls.base,
        }
    }

    pub fn production_url(&self) -> &str {
        match self {
            Self::Default(urls) => &urls.production,
        }
    }
}
impl Default for Environment {
    fn default() -> Self {
        Self::Default(DefaultUrls {
            base: "https://api.phonic.ai/v1".to_string(),
            production: "wss://api.phonic.ai".to_string(),
        })
    }
}
