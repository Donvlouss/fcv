use super::Camera;

#[derive(Debug, Clone, Copy)]
pub enum CameraEvent {
    Pan,
    Rot,
    Zoom,
}

#[derive(Debug)]
pub struct CameraController {
    event: Option<CameraEvent>,
    speed: f32,
    position: (f64, f64),
    event_position: (f64, f64),
}

impl CameraController {
    #[inline]
    pub fn set_pos(&mut self, pos: (f64, f64)) {
        self.position = pos;
    }
    pub fn enable_event(&mut self, e: CameraEvent) {
        if self.event.is_none() {
            self.event = Some(e);
            self.event_position = self.position;
        }
    }
    pub fn disable_event(&mut self) {
        self.event = None;
    }
    pub fn process_delta(&mut self, delta: (f64, f64), camera: &mut Camera) {
        if self.event.is_some() {
            match self.event.take().unwrap() {
                CameraEvent::Pan => self.pan(delta, camera),
                CameraEvent::Rot => self.rot(delta, camera),
                CameraEvent::Zoom => self.zoom(delta.1, camera),
            }
        }
    }
    pub fn process_position(&mut self, position: (f64, f64), camera: &mut Camera) {
        let delta = (position.0 - self.event_position.0, position.1 - self.event_position.1);
        self.process_delta(delta, camera);
    }

    fn pan(&self, delta: (f64, f64), camera: &mut Camera) {
        if delta.0 + delta.1 == 0. {
            return;
        }
        let ndc_delta_x = -delta.0 as f32;
        let ndc_delta_y = delta.1 as f32;

        let focus_distance = (camera.target - camera.eye).length();
        let up = camera.up;
        let right = up.cross((camera.eye - camera.target).normalize());

        let world_delta_x = right * ndc_delta_x * focus_distance;
        let world_delta_y = up * ndc_delta_y * focus_distance;

        let v = world_delta_x + world_delta_y;
        camera.eye += v;
        camera.target += v;
    }
    fn rot(&self, delta: (f64, f64), camera: &mut Camera) {
        if delta.0 + delta.1 == 0. {
            return;
        }
        let forward = camera.target - camera.eye;
        let forward_norm = forward.normalize();
        let forward_mag = forward.length();

        let x = forward_norm.cross(camera.up);
        let y = x.cross(forward_norm);

        camera.eye = camera.target
            - (forward - (x * -delta.0 as f32 + y * delta.1 as f32) * self.speed).normalize()
                * forward_mag;

        camera.up = y;
    }
    fn zoom(&self, delta: f64, camera: &mut Camera) {
        if delta == 0. {
            return;
        }
        let forward = camera.target - camera.eye;
        let forward_norm = forward.normalize();
        let forward_mag = forward.length();

        if delta > 0. && forward_mag > self.speed {
            camera.eye += forward_norm * self.speed;
        } else {
            camera.eye -= forward_norm * self.speed;
        }
    }
}