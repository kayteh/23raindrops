use glium;

pub fn start_graphics(texture_size: u32, texture: Vec<u8>) {
    #[allow(unused_imports)]
    use glium::{glutin, Surface};

    let start_time = std::time::Instant::now();

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_inner_size(glutin::dpi::LogicalSize::new(768.0, 768.0));
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
        uv: [f32; 2],
    }

    implement_vertex!(Vertex, position, uv);

    let mut reload_shader_timeout = std::time::Instant::now();
    let (mut vertex_shader_src, mut fragment_shader_src) = load_default_shaders();
    let mut frag_shader_reload_counter = 0;
    let mut vert_shader_reload_counter = 0;

    let matrix = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0f32],
    ];

    let mut program =
        glium::Program::from_source(&display, &vertex_shader_src, &fragment_shader_src, None)
            .unwrap();

    let shape = vec![
        Vertex {
            position: [1.0, -1.0],
            uv: [0.0, 0.0],
        },
        Vertex {
            position: [1.0, 1.0],
            uv: [1.0, 0.0],
        },
        Vertex {
            position: [-1.0, -1.0],
            uv: [0.0, 1.0],
        },
        Vertex {
            position: [-1.0, 1.0],
            uv: [1.0, 1.0],
        },
    ];

    let shape_indices: [u16; 6] = [0, 1, 2, 1, 3, 2];
    let indices = glium::index::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TrianglesList,
        &shape_indices,
    )
    .unwrap();

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();

    let texture = glium::texture::SrgbTexture2d::new(
        &display,
        glium::texture::RawImage2d::from_raw_rgba_reversed(
            &texture.as_slice()[..],
            (texture_size, texture_size),
        ),
    ).unwrap();

    event_loop.run(move |event, _, control_flow| {
        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);

        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        let uniforms =
            &uniform! { 
                matrix: matrix,
                texture: &texture,
            }.add("Time", start_time.elapsed().as_millis() as f32);

        target
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                uniforms,
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();

        if std::time::Instant::now() > reload_shader_timeout {
            reload_shader_timeout = std::time::Instant::now() + std::time::Duration::from_secs(3);
            let (new_vertex_shader_src, new_fragment_shader_src) = load_shaders();
            if new_vertex_shader_src != vertex_shader_src {
                vert_shader_reload_counter += 1;
                println!("Reloading vertex shader");
            }

            if new_fragment_shader_src != fragment_shader_src {
                frag_shader_reload_counter += 1;
                println!("Reloading fragment shader");
            }

            if new_vertex_shader_src != vertex_shader_src
                || new_fragment_shader_src != fragment_shader_src
            {
                vertex_shader_src = new_vertex_shader_src;
                fragment_shader_src = new_fragment_shader_src;

                match glium::Program::from_source(
                    &display,
                    &vertex_shader_src,
                    &fragment_shader_src,
                    None,
                ) {
                    Ok(new_program) => {
                        program = new_program;
                    }

                    Err(e) => println!("Error reloading vertex shader: {}", e),
                }
            }

            let new_title = format!(
                "23raindrops (reloads: v={} f={}) (elapsed: {}s)",
                vert_shader_reload_counter,
                frag_shader_reload_counter,
                start_time.elapsed().as_secs()
            );

            display.gl_window().window().set_title(&new_title);
        }
    });
}

pub fn load_shaders() -> (String, String) {
    let vertex_shader_src = std::fs::read_to_string("src/glsl/vertex.glsl").unwrap();
    let fragment_shader_src = std::fs::read_to_string("src/glsl/fragment.glsl").unwrap();

    return (vertex_shader_src, fragment_shader_src);
}

pub fn load_default_shaders() -> (String, String) {
    let vertex_shader_src = include_str!("glsl/default.vert.glsl");
    let fragment_shader_src = include_str!("glsl/default.frag.glsl");

    return (
        vertex_shader_src.to_string(),
        fragment_shader_src.to_string(),
    );
}
