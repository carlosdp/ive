extern crate glium;
extern crate obj;
extern crate ive;

use ive::preview::Preview;

fn main() {
    use glium::{DisplayBuild};
    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();
    let preview = Preview::new(display);
    preview.render();
}
