mod image;

use std::sync::Arc;
use chrono::{DateTime, Utc};
use tokio::sync::RwLock;

struct ResourceData  {
    path: Option<String>,
    expiration_time: Option<DateTime<Utc>>,
    data: Option<Vec<u8>>
}

trait Resource {
    
    fn path(&self) -> Option<String>;
    
    fn expiration_time(&self) -> Option<DateTime<Utc>>;
    
    fn data(&self) -> Option<&[u8]>;
}

struct ResourceManager {
    pub resources: Vec<RwLock<Arc<dyn Resource>>>,
}