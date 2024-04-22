use crate::renderer::renderer::Surface;
use std::rc::Rc;
use winit::window::Window;

struct SoftBufferSurface {
    surface: softbuffer::Surface<Rc<Window>, Rc<Window>>,
    framebuffer: tiny_skia::Pixmap,
}

impl SoftBufferSurface {
    fn new(window: Rc<Window>, width: f32, height: f32) -> Self {
        let context = softbuffer::Context::new(window.clone()).unwrap();
        let surface = softbuffer::Surface::new(&context, window.clone()).unwrap();
        let framebuffer = tiny_skia::Pixmap::new(width as u32, height as u32).unwrap();
        Self {
            surface,
            framebuffer,
        }
    }
}

impl Surface for SoftBufferSurface {
    fn width(&self) -> f32 {
        todo!()
    }

    fn height(&self) -> f32 {
        todo!()
    }

    fn present(&mut self) {
        todo!()
    }

    fn resize(&mut self, width: u32, height: u32) {
        let framebuffer = tiny_skia::Pixmap::new(width as u32, height as u32).unwrap();
        self.framebuffer = framebuffer;
    }
}
