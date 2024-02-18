use crate::elements::color::Color;
use crate::elements::element::Element;
use crate::elements::layout_context::{CosmicTextContent, LayoutContext};
use crate::elements::style::{AlignItems, Display, FlexDirection, JustifyContent, Style, Unit};
use crate::widget_id::create_unique_widget_id;
use crate::{Props, RenderContext};
use cosmic_text::rustybuzz::ttf_parser::Width;
use cosmic_text::FontSystem;
use rand::Rng;
use taffy::style_helpers::length;
use taffy::{NodeId, TaffyTree};
use tiny_skia::{LineCap, LineJoin, Paint, PathBuilder, Rect, StrokeDash, Transform};

#[derive(Clone, Default)]
pub struct Empty {
    id: u64,
    children: Vec<Element>,
}

impl Empty {
    pub fn new() -> Empty {
        Empty {
            id: u64::MAX,
            children: vec![],
        }
    }
}

impl Empty {
    pub fn add_child(mut self, widget: Element) -> Empty {
        panic!("Empty cannot have children");
    }

    pub fn children(&self) -> Vec<Element> {
        self.children.clone()
    }

    pub fn children_mut(&mut self) -> &mut Vec<Element> {
        &mut self.children
    }

    pub const fn name(&self) -> &'static str {
        "Empty"
    }

    pub const fn id(&self) -> u64 {
        self.id
    }

    pub fn id_mut(&mut self) -> &mut u64 {
        &mut self.id
    }

    pub fn draw(&mut self, render_context: &mut RenderContext) {}

    pub fn debug_draw(&mut self, render_context: &mut RenderContext) {}

    pub fn compute_layout(&mut self, taffy_tree: &mut TaffyTree<LayoutContext>, font_system: &mut FontSystem) -> NodeId {
        taffy_tree.new_leaf(Style::default().into()).unwrap()
    }

    pub fn finalize_layout(&mut self, taffy_tree: &mut TaffyTree<LayoutContext>, root_node: NodeId, x: f32, y: f32) {}

    pub fn computed_style(&mut self) -> Style {
        Style::default()
    }

    pub fn computed_style_mut(&mut self) -> &mut Style {
        panic!("Empty cannot have a style");
    }
}
