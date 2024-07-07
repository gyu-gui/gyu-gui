use oku::components::component::ComponentOrElement;
use oku::components::component::ComponentSpecification;
use oku::components::component::UpdateFn;
use oku::components::props::Props;
use oku::elements::container::Container;
use oku::elements::style::Weight;
use oku::elements::style::{AlignItems, JustifyContent};
use oku::elements::style::{FlexDirection, Unit};
use oku::elements::text::Text;
use oku::events::Message;
use oku::reactive::reactive::RUNTIME;
use oku::renderer::color::Color;
use oku::RendererType::Wgpu;
use oku::{component, oku_main_with_options, OkuOptions};
use oku_core::elements::image::Image;
use std::any::Any;
use std::path::PathBuf;

pub fn counter(
    _props: Option<Props>,
    _children: Vec<ComponentSpecification>,
    id: u64,
) -> (ComponentSpecification, Option<UpdateFn>) {
    let count = RUNTIME.get_state(id).unwrap_or("start".to_string());
    RUNTIME.set_state(id, count.clone() + "1");

    (
        ComponentSpecification {
            component: Text::new(format!("Counter Count: {}", count.as_str()).as_str())
                .font_size(24.0)
                .font_weight(Weight::SEMIBOLD)
                .into(),
            key: None,
            props: None,
            children: vec![],
        },
        Some(counter_update),
    )
}

pub fn counter_update(_id: u64, _message: Message) {}

pub fn app(
    _props: Option<Props>,
    _children: Vec<ComponentSpecification>,
    _id: u64,
) -> (ComponentSpecification, Option<UpdateFn>) {
    let mut cat_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    cat_path.push("examples");
    cat_path.push("cat.jpg");

    let mut oku_image_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    oku_image_path.push("examples");
    oku_image_path.push("oku.png");

    let cat_path = cat_path.to_str().unwrap();
    let oku_image_path = oku_image_path.to_str().unwrap();
    (
        ComponentSpecification {
            component: Container::new()
                .width(Unit::Percentage(100.0))
                .height(Unit::Percentage(100.0))
                .background(Color::new_from_rgba_u8(255, 255, 255, 255))
                .justify_content(JustifyContent::Center)
                .align_items(AlignItems::Center)
                .flex_direction(FlexDirection::Column)
                .into(),
            key: None,
            props: None,
            children: vec![
                Image::new(cat_path).into(),
                Image::new(oku_image_path).into(),
                ComponentSpecification {
                    component: Container::new()
                        .background(Color::new_from_rgba_u8(200, 200, 200, 255))
                        .padding(10.0, 20.0, 10.0, 20.0)
                        .into(),
                    key: None,
                    props: None,
                    children: vec![component!(counter).into()],
                },
            ],
        },
        None,
    )
}

fn main() {
    oku_main_with_options(
        ComponentSpecification {
            component: component!(app),
            key: None,
            props: None,
            children: vec![],
        },
        Some(OkuOptions { renderer: Wgpu }),
    );
}
