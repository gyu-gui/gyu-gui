mod image;

use std::sync::Arc;
use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;
use tokio::sync::RwLock;
use crate::platform::resource_manager::image::ImageResource;
use crate::user::reactive::reactive::Runtime;

struct ResourceData  {
    path: Option<String>,
    expiration_time: Option<DateTime<Utc>>,
    data: Option<Vec<u8>>
}

enum Resource {
    Image(ImageResource)
}

impl Resource {
    
    pub fn path(&self) -> Option<String> {
        match self {
            Resource::Image(data) => {
                data.common_data.path.clone()
            }
        }
    }

    pub fn data(&self) -> Option<&[u8]> {
        match self {
            Resource::Image(data) => {
                data.common_data.data.as_deref()
            }
        }
    }
    
}

pub struct ResourceManager {
    pub resources: Vec<RwLock<Arc<Resource>>>,
}

impl ResourceManager {
    
    pub fn new() -> Self {
        ResourceManager {
            resources: vec![],
        }
    }
    
}

pub static RESOURCE_MANAGER: Lazy<ResourceManager> = Lazy::new(ResourceManager::new);