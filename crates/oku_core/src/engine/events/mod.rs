mod pointer_button;
mod pointer_moved;

pub mod update_queue_entry;
pub(crate) mod internal;
pub(crate) mod resource_event;

pub use pointer_button::PointerButton;
pub use pointer_moved::PointerMoved;
pub use winit::event::ElementState;
pub use winit::event::ButtonSource;
pub use winit::event::MouseButton;

use std::any::Any;

pub enum EventResult {
    Stop,
    Continue,
}

pub enum OkuEvent {
    PointerButtonEvent(PointerButton),
    PointerMovedEvent(PointerMoved),
}

pub enum Message {
    OkuMessage(OkuEvent),
    UserMessage(Box<dyn Any>),
}
