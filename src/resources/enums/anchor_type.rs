use serde::{Deserialize, Serialize};

/// Filters the webhooks in the response by an anchor_id entity type.
#[derive(Copy, Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub enum AnchorType {
    #[serde(rename = "APPLICATION")]
    #[default]
    Application,
    #[serde(rename = "ACCOUNT")]
    Account,
}

impl AnchorType {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Application => "APPLICATION",
            Self::Account => "ACCOUNT",
        }
    }
}

impl AsRef<str> for AnchorType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AnchorType {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(formatter)
    }
}
