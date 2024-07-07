use crate::components::component::{ComponentOrElement, ComponentSpecification};
use crate::elements::layout_context::LayoutContext;
use crate::elements::style::Style;
use crate::renderer::renderer::Renderer;
use crate::RenderContext;
use cosmic_text::FontSystem;
use std::any::Any;
use std::fmt::Debug;
use taffy::{NodeId, TaffyTree};

#[derive(Clone, Debug, Default)]
pub(crate) struct CommonElementData {
    pub style: Style,
    pub children: Vec<Box<dyn Element>>,
    pub computed_style: Style,
    pub computed_x: f32,
    pub computed_y: f32,
    pub computed_width: f32,
    pub computed_height: f32,
    pub computed_padding: [f32; 4],
    pub id: Option<String>,
    pub component_id: u64,
}

pub trait Element: Any + StandardElementClone + Debug + Send {
    
    fn common_element_data_mut(&mut self) -> &mut CommonElementData;
    
    fn children(&self) -> Vec<Box<dyn Element>>;
    fn children_as_ref<'a>(&'a self) -> Vec<&'a dyn Element>;

    fn children_mut(&mut self) -> &mut Vec<Box<dyn Element>> {
        &mut self.common_element_data_mut().children
    }

    fn name(&self) -> &'static str;

    fn draw(&mut self, renderer: &mut Box<dyn Renderer + Send>, render_context: &mut RenderContext);

    fn debug_draw(&mut self, render_context: &mut RenderContext);

    fn compute_layout(&mut self, taffy_tree: &mut TaffyTree<LayoutContext>, font_system: &mut FontSystem) -> NodeId;
    fn finalize_layout(&mut self, taffy_tree: &mut TaffyTree<LayoutContext>, root_node: NodeId, x: f32, y: f32);

    fn computed_style(&self) -> Style;
    fn computed_style_mut(&mut self) -> &mut Style;

    fn in_bounds(&self, x: f32, y: f32) -> bool;

    fn id(&self) -> &Option<String>;

    fn set_id(&mut self, id: Option<String>);

    fn component_id(&self) -> u64;
    fn set_component_id(&mut self, id: u64);
}

impl<T: Element> From<T> for Box<dyn Element> {
    fn from(element: T) -> Self {
        Box::new(element)
    }
}

impl<T: Element> From<T> for ComponentOrElement {
    fn from(element: T) -> Self {
        ComponentOrElement::Element(Box::new(element))
    }
}

impl From<Box<dyn Element>> for ComponentOrElement {
    fn from(element: Box<dyn Element>) -> Self {
        ComponentOrElement::Element(element)
    }
}

impl From<ComponentOrElement> for ComponentSpecification {
    fn from(element: ComponentOrElement) -> Self {
        ComponentSpecification {
            component: element,
            key: None,
            props: None,
            children: vec![],
        }
    }
}

impl<T: Element> From<T> for ComponentSpecification {
    fn from(element: T) -> Self {
        ComponentSpecification {
            component: ComponentOrElement::Element(Box::new(element)),
            key: None,
            props: None,
            children: vec![],
        }
    }
}

impl dyn Element {
    pub fn print_tree(&self) {
        let mut elements: Vec<(Box<Self>, usize, bool)> = vec![(self.clone_box(), 0, true)];
        while let Some((element, indent, is_last)) = elements.pop() {
            let mut prefix = String::new();
            for _ in 0..indent {
                prefix.push_str("  ");
            }
            if is_last {
                prefix.push_str("└─");
            } else {
                prefix.push_str("├─");
            }
            println!("{}{}, Parent Component Id: {}", prefix, element.name(), element.component_id());
            let children = element.children();
            for (i, child) in children.iter().enumerate().rev() {
                let is_last = i == children.len() - 1;
                elements.push((child.clone(), indent + 1, is_last));
            }
        }
    }
}

pub trait StandardElementClone {
    fn clone_box(&self) -> Box<dyn Element>;
}

impl<T> StandardElementClone for T
where
    T: Element + Clone,
{
    fn clone_box(&self) -> Box<dyn Element> {
        Box::new(self.clone())
    }
}

// We can now implement Clone manually by forwarding to clone_box.
impl Clone for Box<dyn Element> {
    fn clone(&self) -> Box<dyn Element> {
        self.clone_box()
    }
}
