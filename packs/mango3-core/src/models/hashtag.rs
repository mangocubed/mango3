use std::borrow::Cow;
use std::fmt::{Display, Formatter};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize)]
pub struct Hashtag<'a> {
    pub id: Uuid,
    pub name: Cow<'a, str>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Deserialize, Serialize)]
pub(crate) struct Hashtags<'a>(Vec<Hashtag<'a>>);

impl Display for Hashtag<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl Display for Hashtags<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|hashtag| hashtag.id.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

impl<'a> From<Hashtags<'a>> for Vec<Hashtag<'a>> {
    fn from(hashtags: Hashtags<'a>) -> Self {
        hashtags.0
    }
}

impl<'a> From<Vec<Hashtag<'a>>> for Hashtags<'a> {
    fn from(hashtags: Vec<Hashtag<'a>>) -> Self {
        Hashtags(hashtags)
    }
}
