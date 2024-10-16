use crate::platform::resource_manager::ResourceIdentifier;

pub enum ResourcEvent {
    Added(ResourceIdentifier),
    Loaded(ResourceIdentifier),
    UnLoaded(ResourceIdentifier),
}