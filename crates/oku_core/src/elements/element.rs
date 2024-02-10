use crate::elements::container::Container;
use crate::elements::layout_context::LayoutContext;
use crate::elements::style::Style;
use crate::elements::text::Text;
use crate::RenderContext;
use cosmic_text::FontSystem;
use taffy::{NodeId, TaffyTree};

#[derive(Clone)]
pub enum Element {
    Container(Container),
    Text(Text),
}

impl Element {
    pub fn draw(&mut self, render_context: &mut RenderContext) {
        match self {
            Element::Container(container) => container.draw(render_context),
            Element::Text(text) => text.draw(render_context),
        }
    }
    pub fn debug_draw(&mut self, render_context: &mut RenderContext) {
        match self {
            Element::Container(container) => container.debug_draw(render_context),
            Element::Text(text) => text.debug_draw(render_context),
        }
    }
    pub fn children(&mut self) -> Vec<Element> {
        match self {
            Element::Container(container) => container.children(),
            Element::Text(text) => text.children(),
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Element::Container(container) => container.name(),
            Element::Text(text) => text.name(),
        }
    }

    pub fn compute_layout(&mut self, taffy_tree: &mut TaffyTree<LayoutContext>, font_system: &mut FontSystem) -> NodeId {
        match self {
            Element::Container(container) => container.compute_layout(taffy_tree, font_system),
            Element::Text(text) => text.compute_layout(taffy_tree, font_system),
        }
    }

    pub fn computed_style(&mut self) -> Style {
        match self {
            Element::Container(container) => container.computed_style(),
            Element::Text(text) => text.computed_style(),
        }
    }

    pub fn computed_style_mut(&mut self) -> &mut Style {
        match self {
            Element::Container(container) => container.computed_style_mut(),
            Element::Text(text) => text.computed_style_mut(),
        }
    }

    pub fn finalize_layout(&mut self, taffy_tree: &mut TaffyTree<LayoutContext>, root_node: NodeId, x: f32, y: f32) {
        match self {
            Element::Container(container) => container.finalize_layout(taffy_tree, root_node, x, y),
            Element::Text(text) => text.finalize_layout(taffy_tree, root_node, x, y),
        }
    }
}
