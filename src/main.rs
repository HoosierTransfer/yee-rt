use glutin::event::{DeviceEvent, ElementState, Event, VirtualKeyCode, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use std::collections::HashSet;
use std::time::Instant;

mod shader;
mod camera;

fn main() {
    let el = EventLoop::new();
    let wb = WindowBuilder::new().with_title("WOW! so silly :3");

    let windowed_context = ContextBuilder::new()
        .build_windowed(wb, &el)
        .unwrap();

    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    println!("Pixel format of the window's GL context: {:?}", windowed_context.get_pixel_format());

    gl::load_with(|symbol| windowed_context.get_proc_address(symbol) as *const _);

    windowed_context.window().set_cursor_grab(true).unwrap();
    windowed_context.window().set_cursor_visible(false);

    let mut shader = shader::Shader::new("main");

    shader.compile();
    
    let verticies: [f32; 24] = [
        -1.0, 1.0, 0.0, 1.0,
        -1.0, -1.0, 0.0, 0.0,
        1.0, -1.0, 1.0, 0.0,

        -1.0, 1.0, 0.0, 1.0,
        1.0, -1.0, 1.0, 0.0,
        1.0, 1.0, 1.0, 1.0
    ];

    let mut vbo: u32 = 0;
    let mut vao: u32 = 0;

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);

        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER, (verticies.len() * std::mem::size_of::<f32>()) as isize, verticies.as_ptr() as *const _, gl::STATIC_DRAW);

        gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 4 * std::mem::size_of::<f32>() as i32, std::ptr::null());
        gl::EnableVertexAttribArray(0);

        gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, 4 * std::mem::size_of::<f32>() as i32, (2 * std::mem::size_of::<f32>()) as *const _);
        gl::EnableVertexAttribArray(1);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    let mut camera = camera::Camera::new(
        nalgebra::Vector3::new(0.0, 0.0, 3.0),
        nalgebra::Vector3::new(0.0, 1.0, 0.0),
        nalgebra::Vector3::new(-90.0, 0.0, 0.0)
    );
    let (width, height): (u32, u32) = windowed_context.window().inner_size().into();

    let mut pressed_keys: HashSet<VirtualKeyCode> = HashSet::new();

    let mut last_frame = Instant::now();

    let projection = nalgebra::Perspective3::new(width as f32 / height as f32, 45.0, 0.1, 100.0);

    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                },
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(keycode) = input.virtual_keycode {
                        match input.state {
                            ElementState::Pressed => {
                                pressed_keys.insert(keycode);
                            },
                            ElementState::Released => {
                                pressed_keys.remove(&keycode);
                            },
                        }
                    }
                },
                _ => (),
            },
            Event::DeviceEvent { event, .. } => match event {
                DeviceEvent::MouseMotion { delta } => {
                    let (x, y) = delta;
                    camera.process_mouse_movement(x as f32, -y as f32, false);
                },
                _ => (),
            },
            Event::MainEventsCleared => {
                let current_frame = Instant::now();
                let delta_time = current_frame.duration_since(last_frame).as_secs_f32();
                last_frame = current_frame;
                let mut speed = 2.5 * delta_time;
                if pressed_keys.contains(&VirtualKeyCode::LShift) {
                    speed *= 2.0;
                }
                if pressed_keys.contains(&VirtualKeyCode::W) {
                    camera.process_keyboard("FORWARD", speed);
                }
                if pressed_keys.contains(&VirtualKeyCode::S) {
                    camera.process_keyboard("BACKWARD", speed);
                }
                if pressed_keys.contains(&VirtualKeyCode::A) {
                    camera.process_keyboard("LEFT", speed);
                }
                if pressed_keys.contains(&VirtualKeyCode::D) {
                    camera.process_keyboard("RIGHT", speed);
                }
                if pressed_keys.contains(&VirtualKeyCode::Space) {
                    camera.process_keyboard("UP", speed);
                }

                if pressed_keys.contains(&VirtualKeyCode::C) {
                    camera.process_keyboard("DOWN", speed);
                }

                windowed_context.window().request_redraw();
            },
            Event::RedrawRequested(_) => {
                unsafe {
                    gl::ClearColor(0.0, 0.0, 0.0, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT);

                    shader.use_program();

                    shader.set_vec3("cameraPosition", camera.position.into());
                    shader.set_mat4("projectionMatrix", projection.into());
                    shader.set_mat4("viewMatrix", camera.get_view_matrix().into());
                    shader.set_float("time", last_frame.elapsed().as_secs_f32());
                    
                    gl::BindVertexArray(vao);
                    gl::DrawArrays(gl::TRIANGLES, 0, 6);
                }

                windowed_context.swap_buffers().unwrap();
            },
            _ => (),
        }
    });
}