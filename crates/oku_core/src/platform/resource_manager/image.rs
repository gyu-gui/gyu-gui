use chrono::{DateTime, Utc};
use crate::platform::resource_manager::{Resource, ResourceData};

struct ImageResource {
    common_data: ResourceData,
}

impl Resource for ImageResource {
    fn path(&self) -> Option<String> {
        self.common_data.path.clone()
    }

    fn expiration_time(&self) -> Option<DateTime<Utc>> {
        self.common_data.expiration_time
    }

    fn data(&self) -> Option<&[u8]> {
        self.common_data.data.as_deref()
    }
}