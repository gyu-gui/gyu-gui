use std::num::NonZeroU32;
use crate::renderer::renderer::{Rectangle, RenderCommand, Renderer, Surface};
use std::rc::Rc;
use std::sync::Arc;
use softbuffer::Buffer;
use tiny_skia::{LineCap, LineJoin, Paint, PathBuilder, Pixmap, Rect, Transform};
use winit::window::Window;
use crate::renderer::color::Color;

pub struct SoftBufferRenderer {
    render_commands: Vec<RenderCommand>,

    // Surface
    surface: softbuffer::Surface<Arc<Window>, Arc<Window>>,
    surface_width: f32,
    surface_height: f32,
    framebuffer: tiny_skia::Pixmap,
}

impl SoftBufferRenderer {
    pub(crate) fn new(window: Arc<Window>, width: f32, height: f32) -> Self {
        let context = softbuffer::Context::new(window.clone()).unwrap();
        let mut surface = softbuffer::Surface::new(&context, window.clone()).unwrap();
        surface.resize(NonZeroU32::new(width as u32).unwrap(), NonZeroU32::new(height as u32).unwrap()).expect("TODO: panic message");
        let framebuffer = tiny_skia::Pixmap::new(width as u32, height as u32).unwrap();

        Self {
            render_commands: vec![],
            surface,
            surface_width: width,
            surface_height: height,
            framebuffer,
        }
    }
}

fn draw_rect(canvas: &mut Pixmap, rectangle: Rectangle, fill_color: Color) {
    let mut paint = Paint::default();
    paint.set_color_rgba8(fill_color.r_u8(), fill_color.g_u8(), fill_color.b_u8(), fill_color.a_u8());
    paint.anti_alias = true;

    
    let rect = Rect::from_xywh(rectangle.x, rectangle.y, rectangle.width, rectangle.height).unwrap();
    canvas.fill_rect(rect, &paint, Transform::identity(), None);
}

const fn rgb_to_encoded_u32(r: u32, g: u32, b: u32) -> u32 {
    b | (g << 8) | (r << 16)
}

impl Renderer for SoftBufferRenderer {
    fn surface_width(&self) -> f32 {
        self.surface_width
    }

    fn surface_height(&self) -> f32 {
        self.surface_height
    }

    fn present_surface(&mut self) {
        todo!()
    }

    fn resize_surface(&mut self, width: f32, height: f32) {
        self.surface_width = width;
        self.surface_height = height;
        let framebuffer = tiny_skia::Pixmap::new(width as u32, height as u32).unwrap();
        self.surface.resize(NonZeroU32::new(width as u32).unwrap(), NonZeroU32::new(height as u32).unwrap()).expect("TODO: panic message");
        self.framebuffer = framebuffer;
    }

    fn draw_rect(&mut self, rectangle: Rectangle, fill_color: Color) {
        self.render_commands.push(RenderCommand::DrawRect(rectangle, fill_color));
    }

    fn submit(&mut self) {
        for command in self.render_commands.drain(..) {
            match command {
                RenderCommand::DrawRect(rectangle, fill_color) => {
                    draw_rect(&mut self.framebuffer, rectangle, fill_color);
                }
            }
        }

        println!("{}, {}", self.surface_height, self.surface_height);
        let buffer = self.copy_skia_buffer_to_softbuffer(self.surface_width, self.surface_height);
        buffer.present().unwrap();
    }

}

impl SoftBufferRenderer {
    fn copy_skia_buffer_to_softbuffer(&mut self, width: f32, height: f32) -> Buffer<Arc<Window>, Arc<Window>> {
        let mut buffer = self.surface.buffer_mut().unwrap();
        for y in 0..height as u32 {
            for x in 0..width as u32 {
                let index = y as usize * width as usize + x as usize;
                let current_pixel = self.framebuffer.pixels()[index];

                let red = current_pixel.red() as u32;
                let green = current_pixel.green() as u32;
                let blue = current_pixel.blue() as u32;

                buffer[index] = rgb_to_encoded_u32(red, green, blue);
            }
        }
        buffer
    }
}