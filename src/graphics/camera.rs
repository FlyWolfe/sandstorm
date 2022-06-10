use cgmath::*;
use winit::event::*;
use winit::dpi::PhysicalPosition;
use instant::Duration;

use crate::input;

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

pub struct Camera {
    pub position: Point3<f32>,
}

impl Camera {
    pub fn new <
        P: Into<Point3<f32>>,
    >(
        position: P,
    ) -> Self {
        Self {
            position: position.into(),
        }
    }
    
    pub fn calc_look_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_to_rh(
            self.position,
            -Vector3::unit_z(),
            Vector3::unit_y(),
        )
    }
}

pub struct Projection {
    aspect: f32,
    fovy: Rad<f32>,
    znear: f32,
    zfar: f32,
}

impl Projection {
    pub fn new<F: Into<Rad<f32>>>(
        width: u32,
        height: u32,
        fovy: F,
        znear: f32,
        zfar: f32,
    ) -> Self {
        Self {
            aspect: width as f32 / height as f32,
            fovy: fovy.into(),
            znear,
            zfar,
        }
    }
    
    pub fn resize(&mut self, width: u32, height: u32) {
        self.aspect = width as f32 / height as f32;
    }
    
    pub fn calc_perspective_matrix(&self) -> Matrix4<f32> {
        OPENGL_TO_WGPU_MATRIX * perspective(self.fovy, self.aspect, self.znear, self.zfar)
    }
}

#[derive(Debug)]
pub struct CameraController {
    velocity: Vector3<f32>,
    speed: f32,
}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self {
            velocity: Vector3::zero(),
            speed,
        }
    }
    
    pub fn process_input(&mut self, input_controller: &input::Input) {
        if input_controller.move_up {
            self.velocity.y = 1f32;
        }
        else if input_controller.move_down {
            self.velocity.y = -1f32;
        }
        else {
            self.velocity.y = 0f32;
        }
        
        if input_controller.move_left {
            self.velocity.x = -1f32;
        }
        else if input_controller.move_right {
            self.velocity.x = 1f32;
        }
        else {
            self.velocity.x = 0f32;
        }
        
        if self.velocity.x == 0f32 && self.velocity.y == 0f32 {
            return;
        }
        
        self.velocity = self.velocity.normalize();
        self.velocity *= self.speed;
    }
    
    pub fn update_camera(&mut self, camera: &mut Camera, dt: Duration) {
        let dt = dt.as_secs_f32();
        
        camera.position += Vector3::unit_x() * (self.velocity.x * dt);
        camera.position += Vector3::unit_y() * (self.velocity.y * dt);
        camera.position += Vector3::unit_z() * (self.velocity.z * dt);
    }
}
