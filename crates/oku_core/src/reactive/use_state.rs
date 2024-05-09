use crate::reactive::reactive::Runtime;

pub fn use_state<T: Clone + 'static>(value: T) -> (impl Fn() -> T, impl FnMut(T)) {
    let scoped_widget_id = Runtime::get_current_widget_id();
    Runtime::set_state(scoped_widget_id, value);
    
    let state = {
        move || -> T { Runtime::get_state(scoped_widget_id).unwrap() }
    };

    let set_state = move |v: T| {
        Runtime::set_state(scoped_widget_id, v);
    };

    (state, set_state)
}