use crate::elements::element::Element;
use crate::Props;

pub trait Component {
    fn view(&self, props: &Props) -> Element;
}
