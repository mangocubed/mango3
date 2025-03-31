use size::Size;

use crate::config::WEBSITE_CONFIG;
use crate::models::Blob;
use crate::CoreContext;

use super::Website;

impl Website {
    pub async fn available_storage(&self, core_context: &CoreContext) -> Size {
        self.max_storage() - self.used_storage(core_context).await
    }

    pub fn max_storage(&self) -> Size {
        WEBSITE_CONFIG.max_storage
    }

    pub async fn used_storage(&self, core_context: &CoreContext) -> Size {
        Blob::website_used_storage(core_context, self)
            .await
            .expect("Could not get used storage")
    }
}
