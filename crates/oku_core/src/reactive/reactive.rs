use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
use crate::widget_id::create_unique_widget_id;


pub struct Runtime {
    pub current_widget_id: u64,
    pub state: RefCell<HashMap<u64, Box<dyn Any>>>,
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}

impl Runtime {
    pub(crate) fn new() -> Self {
        Self {
            current_widget_id: 0,
            state: Default::default(),
        }
    }

    pub(crate) fn get_current_widget_id() -> u64 {
        RUNTIME.with(|runtime| {
            runtime.current_widget_id
        })
    }
    
    pub(crate) fn get_state<T: Clone + 'static>(key: u64) -> Option<T> {
        RUNTIME.with(|runtime| {
            let state = runtime.state.borrow();
            let context = state.get(&key).and_then(|val| val.downcast_ref::<T>()).cloned();
            context
        })
    }

    pub(crate) fn set_state<T: Clone + 'static>(key: u64, value: T) {
        RUNTIME.with(|runtime| {
            let mut state = runtime.state.borrow_mut();
            let entry  = state.entry(key).or_insert_with(|| Box::new(value.clone()));
            *entry = Box::new(value);
        })
    }
    
}

thread_local! {
    pub static RUNTIME: Runtime = Runtime::new();
}

