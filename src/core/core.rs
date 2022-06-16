use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::graphics::{renderer, camera};
use crate::input;

pub struct GameState {
    pub render_state: renderer::RenderState,
    pub camera: camera::Camera,
}

pub fn start() {
    pollster::block_on(run());
}

pub async fn run() {
    env_logger::init();
    
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    
    let render_state = renderer::RenderState::new(&window).await;
    let camera = camera::Camera::new((0.0, 0.0, 10.0));
    
    let mut game_state = GameState { render_state, camera };
    
    let mut last_render_time = instant::Instant::now();
    
    let mut input_controller = input::Input::new();
    
    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } => {
                let found_key = input_controller.process_input_event(event);
                if found_key {
                    game_state.render_state.camera_controller.process_input(&input_controller);
                }
                if window_id == window.id() {
                    match event {
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        } => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            game_state.render_state.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            game_state.render_state.resize(**new_inner_size);
                        }
                        _ => {}
                    }
                }
            }
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                let now = instant::Instant::now();
                let dt = now - last_render_time;
                last_render_time = now;
                game_state.render_state.update(dt);
                match game_state.render_state.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => game_state.render_state.resize(game_state.render_state.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => {}
        }
    });
}
