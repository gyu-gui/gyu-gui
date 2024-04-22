pub mod surface;

use crate::renderer::renderer::{Rectangle, RenderCommand, Renderer, Surface};
use std::rc::Rc;
use winit::window::Window;

struct SoftBufferRenderer {
    context: softbuffer::Context<Rc<Window>>,
    render_commands: Vec<RenderCommand>,
}

impl SoftBufferRenderer {
    fn new(window: Rc<Window>) -> Self {
        let context = softbuffer::Context::new(window.clone()).unwrap();
        Self {
            context,
            render_commands: vec![],
        }
    }
}

impl Renderer for SoftBufferRenderer {
    fn draw_rect(&mut self, rectangle: Rectangle) {
        self.render_commands.push(RenderCommand::DrawRect(rectangle));
    }

    fn submit(&mut self, surface: Box<dyn Surface>) {
        todo!()
    }
}
