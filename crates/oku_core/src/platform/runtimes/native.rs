use std::io;
use tokio::runtime::Runtime;

pub fn create_native_runtime() -> io::Result<Runtime> {
    tokio::runtime::Builder::new_multi_thread().enable_all().build()
} 