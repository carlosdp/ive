#[macro_use]
extern crate glium;
extern crate obj;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3]
}

implement_vertex!(Vertex, position, normal);

fn main() {
    use glium::{DisplayBuild, Surface};
    use std::fs::File;
    use std::io::BufReader;
    use std::string::String;
    let filename = match std::env::args().nth(1) {
        Some(f) => f,
        None => String::from("./cube.obj")
    };
    let input = BufReader::new(File::open(filename).unwrap());
    let cube: obj::Obj = obj::load_obj(input).unwrap();
    let cube_vertices = cube.vertices.iter().map(|v| Vertex { position: v.position, normal:
        v.normal
    }).collect::<Vec<Vertex>>();

    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

    let vertex_buffer = glium::VertexBuffer::new(&display, &cube_vertices).unwrap();
    let indices = glium::index::IndexBuffer::new(&display,
                                                 glium::index::PrimitiveType::TrianglesList, &cube.indices).unwrap();

    let vertex_shader_src = r#"
        #version 140

        in vec3 position;
        in vec3 normal;

        uniform mat4 matrix;

        out vec3 v_normal;      // new

        void main() {
            v_normal = normal;
            gl_Position = vec4(position, 1.0);
        }
    "#;

    let frag_shader = r#"
        #version 140

        in vec3 v_normal;
        out vec4 color;
        uniform vec3 u_light;

        void main() {
            float brightness = dot(normalize(v_normal), normalize(u_light));
            vec3 dark_color = vec3(0.6, 0.0, 0.0);
            vec3 regular_color = vec3(1.0, 0.0, 0.0);
            color = vec4(mix(dark_color, regular_color, brightness), 1.0);
        }
    "#;

    let light = [-1.0, 0.4, 0.9f32];

    let matrix = [
        [0.01, 0.0, 0.0, 0.0],
        [0.0, 0.01, 0.0, 0.0],
        [0.0, 0.0, 0.01, 0.0],
        [0.0, 0.0, 0.0, 1.0f32],
    ];

    let program = glium::Program::from_source(&display, vertex_shader_src, frag_shader, None).unwrap();

    loop {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.draw(&vertex_buffer, &indices, &program, &uniform! {matrix: matrix, u_light: light},
                    &Default::default()).unwrap();
        target.finish().unwrap();


        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}
