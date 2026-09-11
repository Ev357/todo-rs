use std::{fmt, fmt::Display};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SearchQuery {
    #[serde(default)]
    pub search: String,
}

impl Display for SearchQuery {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.search.trim().is_empty() {
            return Ok(());
        }

        let encoded = serde_urlencoded::to_string(self).unwrap_or_default();
        write!(f, "{encoded}")
    }
}

impl<'a> From<&'a str> for SearchQuery {
    fn from(s: &'a str) -> Self {
        serde_urlencoded::from_str(s).unwrap_or_default()
    }
}
