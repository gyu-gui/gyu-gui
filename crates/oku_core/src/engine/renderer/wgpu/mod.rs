mod texture;
mod context;
mod uniform;
mod pipeline_2d;

use crate::engine::renderer::color::Color;
use crate::engine::renderer::renderer::{Rectangle, Renderer};
use glam;
use image::{GenericImageView, ImageEncoder};
use std::collections::HashMap;
use std::sync::Arc;
use wgpu::util::DeviceExt;
use winit::window::Window;
use crate::engine::renderer::wgpu::context::{create_surface_config, request_adapter, request_device_and_queue, Context};
use crate::engine::renderer::wgpu::pipeline_2d::Pipeline2D;
use crate::engine::renderer::wgpu::texture::Texture;

pub struct WgpuRenderer<'a> {
    context: Context<'a>,
    pipeline2d: Pipeline2D,
}

struct RectangleBatch {
    texture_path: Option<String>,
    rectangle_vertices: Vec<Vertex>,
    rectangle_indices: Vec<u32>,
}

fn bind_group_from_2d_texture(
    device: &wgpu::Device,
    texture_bind_group_layout: &wgpu::BindGroupLayout,
    texture: &Texture,
) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: texture_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&texture.view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&texture.sampler),
            },
        ],
        label: Some("oku_bind_group"),
    })
}

impl<'a> WgpuRenderer<'a> {
    pub(crate) async fn new(window: Arc<dyn Window>) -> WgpuRenderer<'a> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::DX12 | wgpu::Backends::GL,
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone()).unwrap();
        let adapter = request_adapter(instance, &surface).await;
        let (device, queue) = request_device_and_queue(&adapter).await;

        let surface_size = window.surface_size();
        let surface_config =
            create_surface_config(&surface, surface_size.width, surface_size.height, &device, &adapter);
        surface.configure(&device, &surface_config);

        let default_texture = Texture::generate_default_white_texture(&device, &queue);

        let context = Context {
            device,
            queue,
            default_texture,
            surface,
            surface_config,
            surface_clear_color: Color::new_from_rgba_u8(255, 255, 255, 255),
        };

        let pipeline2d = Pipeline2D::new(&context);
        
        WgpuRenderer {
            pipeline2d,
            context,
        }
    }
}

impl Renderer for WgpuRenderer<'_> {
    fn surface_width(&self) -> f32 {
        self.context.surface_config.width as f32
    }

    fn surface_height(&self) -> f32 {
        self.context.surface_config.height as f32
    }

    fn present_surface(&mut self) {
        todo!()
    }

    fn resize_surface(&mut self, width: f32, height: f32) {
        self.context.surface_config.width = width as u32;
        self.context.surface_config.height = height as u32;
        self.context.surface.configure(&self.context.device, &self.context.surface_config);
        self.pipeline2d.camera = Camera {
            width,
            height,
            z_near: 0.0,
            z_far: 100.0,
        };

        self.pipeline2d.camera_uniform.update_view_proj(&self.pipeline2d.camera);
        self.context.queue.write_buffer(&self.pipeline2d.camera_buffer, 0, bytemuck::cast_slice(&[self.pipeline2d.camera_uniform.view_proj]));
    }

    fn surface_set_clear_color(&mut self, color: Color) {
        self.context.surface_clear_color = color;
    }

    fn draw_rect(&mut self, rectangle: Rectangle, fill_color: Color) {
       self.pipeline2d.draw_rect(rectangle, fill_color);
    }

    fn draw_image(&mut self, rectangle: Rectangle, path: &str) {
        self.pipeline2d.draw_image(rectangle, path)
    }

    fn submit(&mut self) {
        self.pipeline2d.submit(&mut self.context)
    }     
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 4],
    tex_coords: [f32; 2],
}

struct Camera {
    width: f32,
    height: f32,
    z_near: f32,
    z_far: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    fn new() -> Self {
        Self {
            view_proj: glam::Mat4::IDENTITY.to_cols_array_2d(),
        }
    }

    fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().to_cols_array_2d();
    }
}

impl Camera {
    fn build_view_projection_matrix(&self) -> glam::Mat4 {
        let view = glam::Mat4::IDENTITY;
        let proj = glam::Mat4::orthographic_lh(0.0, self.width, self.height, 0.0, self.z_near, self.z_far);
        proj * view
    }
}

impl Vertex {
    fn description<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 7]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}
