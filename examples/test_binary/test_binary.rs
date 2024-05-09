use oku::application::Props;
use oku::components::component::Component;
use oku::elements::container::Container;
use oku::elements::element::Element;
use oku::elements::text::Text;
use oku_core::OkuOptions;
use oku_core::RendererType::{Software, Wgpu};
use std::cell::RefCell;
use std::rc::Rc;
use oku_core::reactive::reactive::Runtime;
use oku_core::reactive::use_state::{use_state};

/*fn use_state<T: Clone>(value: T) -> (impl Fn() -> T, impl FnMut(T)) {
    let val = Rc::new(RefCell::new(value));

    let state = {
        let val = val.clone();
        move || -> T { val.borrow().clone() }
    };

    let set_state = move |v: T| {
        val.replace(v);
    };

    (state, set_state)
}*/

struct Test1 {}

impl Component for Test1 {
    fn view(&self, _props: Option<&Props>, _children: Vec<Element>) -> Element {
        Element::Text(Text::new(String::from("Hello")))
    }
}

struct Hello {}

impl Component for Hello {
    fn view(&self, props: Option<&Props>, children: Vec<Element>) -> Element {

        let (number, mut set_number) = use_state(123);

        println!("{:?}", number());
        set_number(45);
        println!("{:?}", number());

        // let my_data = props.unwrap().get_data::<u32>().unwrap();
        let mut container = Container::new().add_child(Element::Text(Text::new(format!("Hello, world! {}", number()))));

        for child in children {
            container = container.add_child(child);
        }

        Element::Container(container)
    }
}

struct App {}

impl oku_core::application::Application for App {
    fn view(&self) -> Element {
        let hello = Hello {};
        let hello_props = Props {
            data: Box::new(12_u32),
        };

        let test1 = Test1 {};

        hello.view(Some(&hello_props), vec![test1.view(None, vec![])])
    }
}

fn main() {
    let application = App {};
    oku_core::oku_main_with_options(Box::new(application), Some(OkuOptions { renderer: Wgpu }));
}
