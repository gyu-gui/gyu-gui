#[path = "../../examples/counter/counter.rs"]
mod counter;

#[path = "../../examples/text/text.rs"]
mod text;

#[path = "../../examples/request/request.rs"]
mod request;

#[path = "../../examples/tour/tour.rs"]
mod tour;

use crate::theme::EXAMPLES_SIDEBAR_BACKGROUND_COLOR;
use crate::WebsiteGlobalState;
use craft::components::{Component, ComponentId, ComponentSpecification, UpdateResult};
use craft::elements::{Container, ElementStyles, Text};
use craft::events::Event;
use craft::palette;
use craft::style::Display::Flex;
use craft::style::FlexDirection;

use crate::examples::counter::Counter;
use crate::examples::request::AniList;
use crate::examples::text::TextState;
use crate::examples::tour::Tour;

pub(crate) struct Examples {
    pub(crate) example_to_show: String,
}

impl Default for Examples {
    fn default() -> Self {
        Examples {
            example_to_show: "counter".to_string(),
        }
    }
}

fn create_examples_link(label: &str, example_link: &str) -> Text {
    Text::new(label).id(example_link)
}

fn examples_sidebar() -> ComponentSpecification {
    Container::new()
        .background(EXAMPLES_SIDEBAR_BACKGROUND_COLOR)
        .display(Flex)
        .flex_direction(FlexDirection::Column)
        .gap("15px")
        .width("30%")
        .padding("20px", "20px", "20px", "20px")
        .min_width("300px")
        .max_width("50%")
        .height("100%")
        .push(Text::new("Examples").font_size(24.0).component())
        .push(create_examples_link("Counter", "example_counter"))
        .push(create_examples_link("Tour", "example_tour"))
        .push(create_examples_link("Request", "example_request"))
        .push(create_examples_link("Text", "example_text_state"))
        .component()
}

impl Component<WebsiteGlobalState> for Examples {
    type Props = ();

    fn view(
        state: &Self,
        _global_state: &WebsiteGlobalState,
        _props: &Self::Props,
        _children: Vec<ComponentSpecification>,
        _id: ComponentId,
    ) -> ComponentSpecification {
        let wrapper = Container::new().display(Flex).width("100%").height("100%").push(examples_sidebar()).component();

        wrapper.push(Container::new().width("100%").height("100%").background(palette::css::WHITE).push(
            match state.example_to_show.as_str() {
                "text_state" => TextState::component().key("example_text_state"),
                "tour" => Tour::component().key("example_tour"),
                "request" => AniList::component().key("example_request"),
                "counter" | &_ => Counter::component().key("example_counter"),
            },
        ))
    }

    fn update(
        state: &mut Self,
        _global_state: &mut WebsiteGlobalState,
        _props: &Self::Props,
        event: Event,
    ) -> UpdateResult {
        if event.message.clicked() && event.current_target.is_some() {
            let current_target = event.current_target.as_ref().unwrap();
            if current_target.starts_with("example_") {
                state.example_to_show = current_target.replace("example_", "").to_string();
            }
        }

        UpdateResult::default()
    }
}
