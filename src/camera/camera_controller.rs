use super::Camera;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CameraEvent {
    Pan,
    Rot,
    Zoom,
}

#[derive(Debug, Default)]
pub struct CameraController {
    event: Option<CameraEvent>,
    speed: f32,
    position: (f64, f64),
    event_position: (f64, f64),
    delta: (f32, f32),
    window_size: (u32, u32),
}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self {
            speed, ..Default::default()
        }
    }
    pub fn resize(&mut self, size: (u32, u32)) {
        self.window_size = size;
    }
    pub fn enabled(&self) -> bool {
        match self.event {
            Some(e) => e != CameraEvent::Zoom,
            None => false,
        }
    }
    #[inline]
    pub fn set_pos(&mut self, pos: (f64, f64)) {
        self.position = pos;
    }
    pub fn enable_event(&mut self, e: CameraEvent) {
        if self.event.is_none() {
            self.event = Some(e);
            self.event_position = self.position;
            self.delta = (0., 0.);
        }
    }
    pub fn disable_event(&mut self) {
        self.event = None;
    }
    pub fn process_delta(&mut self, camera: &mut Camera) -> bool {
        if self.event.is_some() {
            match self.event.unwrap() {
                CameraEvent::Pan => self.pan(camera),
                CameraEvent::Rot => self.rot(camera),
                CameraEvent::Zoom => {
                    self.zoom(camera);
                    self.event = None;
                },
            }
            self.delta = (0., 0.);
            true
        } else {
            false
        }
    }
    pub fn set_delta(&mut self, xy: (f64, f64)) {
        self.delta = ((xy.0 - self.event_position.0) as f32, (xy.1 - self.event_position.1) as f32);
        self.event_position = xy;
    }
    pub fn set_zoom_delta(&mut self, xy: (f32, f32)) {
        self.delta = xy;
    }

    fn pan(&self, camera: &mut Camera) {
        if self.delta.0 + self.delta.1 == 0. {
            return;
        }
        let ndc_delta_x = -self.delta.0 as f32 / self.window_size.0 as f32;
        let ndc_delta_y = self.delta.1 as f32 / self.window_size.1 as f32;

        let z = camera.eye - camera.target;
        let focus_distance = z.length();
        let up = camera.up;
        let right = up.cross(z.normalize());
        let up = z.normalize().cross(right);

        let world_delta_x = right * ndc_delta_x * focus_distance;
        let world_delta_y = up * ndc_delta_y * focus_distance;

        let v = world_delta_x + world_delta_y;
        camera.eye += v;
        camera.target += v;
        camera.up = up.normalize();
    }
    fn rot(&self, camera: &mut Camera) {
        if self.delta.0 + self.delta.1 == 0. {
            return;
        }
        let forward = camera.target - camera.eye;
        let forward_norm = forward.normalize();
        let forward_mag = forward.length();

        let x = forward_norm.cross(camera.up);
        let y = x.cross(forward_norm);

        let dx = -self.delta.0 as f32;
        let dy = self.delta.1 as f32;
        let d = x * dx + y * dy;

        let factor = self.speed * forward_mag.max(0.1).min(1.0);

        camera.eye = camera.target
            - (forward - d * factor).normalize()
                * forward_mag;

        camera.up = y.normalize();
    }
    fn zoom(&self, camera: &mut Camera) {
        if self.delta.1 == 0. {
            return;
        }
        let forward = camera.target - camera.eye;
        let forward_norm = forward.normalize();
        let forward_mag = forward.length();

        if self.delta.1 > 0. && forward_mag > self.speed {
            camera.eye += forward_norm * self.speed;
        } else if self.delta.1 > 0. && forward_mag * 0.1 > self.speed * 0.1 {
            camera.eye += forward_norm * forward_mag * 0.1;
        } else {
            camera.eye -= forward_norm * self.speed;
        }
    }
}