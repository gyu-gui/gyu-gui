use crate::reactive::reactive::OnClickType;
use crate::reactive::reactive::Runtime;
use std::rc::Rc;

pub fn use_state<T: Clone + 'static>(value: T) -> (impl Fn() -> T, impl FnMut(T)) {
    let scoped_widget_id = Runtime::get_current_widget_id();

    if !Runtime::has_state(scoped_widget_id) {
        Runtime::set_state(scoped_widget_id, value);
    }

    let state = { move || -> T { Runtime::get_state(scoped_widget_id).unwrap() } };

    let set_state = move |v: T| {
        Runtime::set_state(scoped_widget_id, v);
    };

    (state, set_state)
}

pub fn use_click(handler: OnClickType) {
    let scoped_widget_id = 0;
    Runtime::set_click_handler(scoped_widget_id, handler);
}
