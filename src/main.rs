use glutin::event::{Event, WindowEvent, VirtualKeyCode, ElementState};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;

mod shader;

fn main() {
    let el = EventLoop::new();
    let wb = WindowBuilder::new().with_title("WOW! so silly :3");

    let windowed_context = ContextBuilder::new()
        .build_windowed(wb, &el)
        .unwrap();

    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    println!("Pixel format of the window's GL context: {:?}", windowed_context.get_pixel_format());

    gl::load_with(|symbol| windowed_context.get_proc_address(symbol) as *const _);

    let mut shader = shader::Shader::new("basic");

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

    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                },
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(VirtualKeyCode::Escape) = input.virtual_keycode {
                        if input.state == ElementState::Pressed {
                            *control_flow = ControlFlow::Exit;
                        }
                    }
                },
                _ => (),
            },
            Event::RedrawRequested(_) => {
                unsafe {
                    gl::ClearColor(0.0, 0.0, 0.0, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT);

                    shader.use_program();
                    
                    gl::BindVertexArray(vao);
                    gl::DrawArrays(gl::TRIANGLES, 0, 6);
                }

                windowed_context.swap_buffers().unwrap();
            },
            _ => (),
        }
    });
}
