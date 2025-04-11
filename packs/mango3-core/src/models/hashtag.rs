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
pub(crate) struct Hashtags(Vec<Hashtag>);

impl Display for Hashtags {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|hashtag| hashtag.id.to_string())
                .collect::<Vec<String>>()
                .join(',')
        )
    }
}

impl From<Hashtags> for Vec<Hashtag> {
    fn from(hashtags: Hashtags) -> Self {
        hashtags.0
    }
}

impl From<Vec<Hashtag>> for Hashtags {
    fn from(hashtags: Vec<Hashtag>) -> Self {
        Hashtags(hashtags)
    }
}
