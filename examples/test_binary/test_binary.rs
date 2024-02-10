use oku_core::components::component::Component;
use oku_core::elements::container::Container;
use oku_core::elements::element::Element;
use oku_core::elements::text::Text;
use oku_core::Props;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

fn use_state<T: Clone>(value: T) -> (impl Fn() -> T, impl FnMut(T)) {
    let val = Rc::new(RefCell::new(value));

    let state = {
        let val = val.clone();
        move || -> T { val.borrow().clone() }
    };

    let set_state = move |v: T| {
        val.replace(v);
    };

    (state, set_state)
}

struct Hello {}

impl Component for Hello {
    fn view(&self, props: &Props) -> Element {
        let (data, mut set_data) = use_state(String::from("foo"));

        println!("data: {}", data());
        set_data(String::from("bar"));
        println!("data: {}", data());

        let my_data = props.get_data::<u32>().unwrap();
        let container = Container::new().add_child(Element::Text(Text::new(format!("Hello, world! {}", my_data))));
        Element::Container(container)
    }
}

fn main() {
    let hello = Hello {};
    let hello_props = Props {
        data: Box::new(12_u32),
    };

    oku_dylib::oku_core::main(hello.view(&hello_props));
}
