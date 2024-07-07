use crate::elements::element::{CommonElementData, Element};
use crate::elements::layout_context::LayoutContext;
use crate::elements::style::Style;
use crate::renderer::renderer::Renderer;
use crate::RenderContext;
use cosmic_text::FontSystem;
use taffy::{NodeId, TaffyTree};

#[derive(Clone, Default, Debug)]
pub struct Empty {
    common_element_data: CommonElementData
}

impl Empty {
    pub fn new() -> Empty {
        Empty {
            common_element_data: Default::default(),
        }
    }
}

impl Element for Empty {
    fn common_element_data_mut(&mut self) -> &mut CommonElementData {
        &mut self.common_element_data
    }

    fn children(&self) -> Vec<Box<dyn Element>> {
        self.common_element_data.children.clone()
    }

    fn children_as_ref<'a>(&'a self) -> Vec<&'a dyn Element> {
        self.common_element_data.children.iter().map(|x| x.as_ref()).collect()
    }

    fn name(&self) -> &'static str {
        "Empty"
    }

    fn draw(&mut self, _renderer: &mut Box<dyn Renderer + Send>, _render_context: &mut RenderContext) {}

    fn debug_draw(&mut self, _render_context: &mut RenderContext) {}

    fn compute_layout(&mut self, taffy_tree: &mut TaffyTree<LayoutContext>, font_system: &mut FontSystem) -> NodeId {
        let mut child_nodes: Vec<NodeId> = Vec::with_capacity(self.children().len());

        for child in self.common_element_data.children.iter_mut() {
            let child_node = child.compute_layout(taffy_tree, font_system);
            child_nodes.push(child_node);
        }

        let style: taffy::Style = self.common_element_data.style.into();

        taffy_tree.new_with_children(style, &vec![]).unwrap()
    }

    fn finalize_layout(&mut self, _taffy_tree: &mut TaffyTree<LayoutContext>, _root_node: NodeId, _x: f32, _y: f32) {}

    fn computed_style(&self) -> Style {
        Style::default()
    }

    fn computed_style_mut(&mut self) -> &mut Style {
        &mut self.common_element_data.computed_style
    }

    fn in_bounds(&self, _x: f32, _y: f32) -> bool {
        false
    }

    fn id(&self) -> &Option<String> {
        &self.common_element_data.id
    }

    fn set_id(&mut self, id: Option<String>) {
        self.common_element_data.id = id;
    }

    fn component_id(&self) -> u64 {
        self.common_element_data.component_id
    }

    fn set_component_id(&mut self, id: u64) {
        self.common_element_data.component_id = id;
    }
}
