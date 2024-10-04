pub struct Camera {
    pub width: f32,
    pub height: f32,
    pub z_near: f32,
    pub z_far: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub(crate) fn new() -> Self {
        Self {
            view_proj: glam::Mat4::IDENTITY.to_cols_array_2d(),
        }
    }

    pub(crate) fn update_view_proj(&mut self, camera: &Camera) {
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