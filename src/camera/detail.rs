use glam::{Mat4, Vec3};

use super::{Camera, CameraGraphic, CameraUniform, PerspectiveConfig};

impl Default for Camera {
    fn default() -> Self {
        Self {
            eye: Vec3::ONE,
            target: Vec3::ZERO,
            up: Vec3::Y,
            z_near: 0.01,
            z_far: 100.,
            graphic: CameraGraphic::Perspective(PerspectiveConfig::default()),
        }
    }
}

impl Camera {
    pub fn with_eye(mut self, eye: Vec3) -> Self {
        self.eye = eye;
        self
    }
    pub fn with_target(mut self, target: Vec3) -> Self {
        self.target = target;
        self
    }
    pub fn with_up(mut self, up: Vec3) -> Self {
        self.up = up;
        self
    }
    pub fn with_z_near(mut self, z_near: f32) -> Self {
        self.z_near = z_near;
        self
    }
    pub fn with_z_far(mut self, z_far: f32) -> Self {
        self.z_far = z_far;
        self
    }
    pub fn with_graphic(mut self, graphic: CameraGraphic) -> Self {
        self.graphic = graphic;
        self
    }
}

impl Camera {
    pub fn build_view_projection_matrix(&self) -> Mat4 {
        let view = Mat4::look_at_rh(self.eye, self.target, self.up);
        let proj = match self.graphic {
            CameraGraphic::Perspective(config) => {
                Mat4::perspective_rh(
                    config.fov_y_degree.to_radians(),
                    config.aspect, self.z_near, self.z_far)
            },
            CameraGraphic::Orthogonal(w, h) => {
                Mat4::orthographic_rh(
                    0., w, h, 0., self.z_near, self.z_far
                )
            },
        };
        proj * view
    }
    pub fn on_resize(&mut self, size: (f32, f32)) {
        match &mut self.graphic {
            CameraGraphic::Perspective(perspective_config) => {
                perspective_config.aspect = size.0 / size.1;
            },
            CameraGraphic::Orthogonal(w, h) => {
                (*w, *h) = size;
            },
        }
    }
}

impl From<Camera> for CameraUniform {
    fn from(value: Camera) -> Self {
        Self {
            view_proj: value.build_view_projection_matrix().to_cols_array_2d()
        }
    }
}