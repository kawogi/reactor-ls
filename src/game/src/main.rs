#![warn(clippy::pedantic)]
#![allow(clippy::non_ascii_literal)]

use std::io::Cursor;
use std::process;
use std::time::{Duration, Instant};

use glium::{Surface, vertex::VertexBufferAny, glutin::dpi::{PhysicalSize, Size}, program, uniform};
use glium::index::{ NoIndices, PrimitiveType };
use glium::glutin::event::{ Event, WindowEvent, StartCause };
use glium::glutin::event_loop::{ EventLoop, ControlFlow };
use log::{debug, error};
use model::load_stl;

mod display;
mod model;
mod camera;

pub type GliumMatrix = [[f32; 4]; 4];

#[repr(i32)]
pub enum ExitCode {
    //Ok = 0,
    LoadMesh = 1,
    CreateDisplay = 2,
}

impl From<ExitCode> for i32 {
    fn from(code: ExitCode) -> Self {
        code as i32
    }
}

pub enum Action {
    Stop,
    Continue,
}

fn main() {
    pretty_env_logger::init();

    debug!("loading mesh");
    let vertex_data = load_stl(&mut Cursor::new(include_bytes!("../../../res/axis.stl")))
            .unwrap_or_else(|err| {
                error!("Could not parse stl: {}", err);
                process::exit(ExitCode::LoadMesh as i32)
            });


    debug!("creating display");
    let event_loop = EventLoop::new();
    let window_width = 1024;
    let window_height = 768;
    #[allow(clippy::clippy::cast_precision_loss)]
    let aspect_ratio = window_width as f32 / window_height as f32;

    let display = display::create(&event_loop, Size::Physical(PhysicalSize::new(window_width, window_height)))
            .unwrap_or_else(|err| {
                // TODO maybe differentiate between error types
                error!("Could not create display: {}", err);
                process::exit(ExitCode::CreateDisplay as i32)
            });

    display::dump_details(&display);

    // building the vertex and index buffers
    debug!("create vertex buffer from mesh");
    let vertex_buffer: VertexBufferAny = glium::vertex::VertexBuffer::new(&display, &vertex_data).unwrap().into();

    let vertex_shader_str = "
    #version 140

    uniform mat4 persp_matrix;
    uniform mat4 view_matrix;

    in vec3 position;
    in vec3 normal;
    out vec3 v_position;
    out vec3 v_normal;

    void main() {
        v_position = position;
        v_normal = normal;
        gl_Position = persp_matrix * view_matrix * vec4(v_position * 1.00, 1.0);
    }
    ";

    let fragment_shader_stl = "
    #version 140

    in vec3 v_normal;
    out vec4 f_color;

    const vec3 LIGHT = vec3(-0.2, 0.8, 0.1);

    void main() {
        float lum = max(dot(normalize(v_normal), normalize(LIGHT)), 0.0);
        vec3 color = (0.1 + 0.9 * lum * lum * lum) * vec3(1.0, 1.0, 1.0);
        f_color = vec4(color, 1.0);
    }
    ";

    let program = program!(&display,
        140 => { vertex: vertex_shader_str, fragment: fragment_shader_stl, },
    ).unwrap();

    let mut camera = camera::CameraState::new(aspect_ratio);

    debug!("start main loop …");
    start_loop(event_loop, move |events| {
        camera.update_position();

        // building the uniforms
        let persp_matrix: GliumMatrix = camera.get_perspective().into();
        let view_matrix: GliumMatrix = camera.get_view().into();
        let uniforms = uniform! { persp_matrix: persp_matrix, view_matrix: view_matrix, };

        // draw parameters
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                .. glium::Depth::default()
            },
            .. glium::DrawParameters::default()
        };

        // drawing a frame
        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);
        target.draw(&vertex_buffer, &NoIndices(PrimitiveType::TrianglesList), &program, &uniforms, &params).unwrap();
        target.finish().unwrap();

        let mut action = Action::Continue;

        // polling and handling the events received by the window
        for event in events {
            #[allow(clippy::single_match)]
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => action = Action::Stop,
                    #[allow(clippy::clippy::cast_precision_loss)]
                    WindowEvent::Resized(size) => camera.set_aspect_ratio(size.width as f32 / size.height as f32),
                    ev => camera.process_input(&ev),
                },
                _ => (),
            }
        };

        action
    });

}

pub fn start_loop<F>(event_loop: EventLoop<()>, mut callback: F)->!
where
    F: 'static + FnMut(&Vec<Event<'_, ()>>) -> Action
{
    let mut events_buffer = Vec::new();
    let mut next_frame_time = Instant::now();
    event_loop.run(move |event, _, control_flow| {
        let run_callback = match event.to_static() {
            Some(Event::NewEvents(cause)) => {
                match cause {
                    StartCause::ResumeTimeReached { .. } | StartCause::Init => {
                        true
                    },
                    _ => false
                }
            },
            Some(event) => {
                events_buffer.push(event);
                false
            }
            None => {
                // Ignore this event.
                false
            },
        };

        let action = if run_callback {
            let action = callback(&events_buffer);
            next_frame_time = Instant::now() + Duration::from_nanos(16_666_667);
            // TODO: Add back the old accumulator loop in some way

            events_buffer.clear();
            action
        } else {
            Action::Continue
        };

        match action {
            Action::Continue => {
                *control_flow = ControlFlow::WaitUntil(next_frame_time);
            },
            Action::Stop => *control_flow = ControlFlow::Exit
        }
    })
}


