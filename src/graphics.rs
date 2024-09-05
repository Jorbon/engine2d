use glium::{program::ProgramCreationInput, texture::{RawImage2d, SrgbTexture2d}, Display, Frame, Surface, Texture2d};

use crate::*;


pub fn load_texture(display: &Display, path: &str) -> SrgbTexture2d {
    let image = image::open(&format!("assets/textures/{path}.png")).unwrap();
    use image::GenericImageView;
    SrgbTexture2d::new(display, RawImage2d::from_raw_rgba(image.to_rgba8().into_raw(), image.dimensions())).unwrap()
}

pub fn blit_texture(target: &mut Frame, texture: &Texture2d) {
    texture.as_surface().blit_whole_color_to(
        target,
        &glium::BlitTarget {
            left: 0,
            bottom: 0,
            width: target.get_dimensions().0 as i32,
            height: target.get_dimensions().1 as i32,
        },
        glium::uniforms::MagnifySamplerFilter::Nearest
    );
}

pub fn load_shader_program(display: &Display, vert_shader: &str, frag_shader: &str) -> glium::Program {
    glium::Program::new(display, ProgramCreationInput::SourceCode {
        vertex_shader: &std::fs::read_to_string(format!("src/shaders/{vert_shader}.vert")).unwrap(),
        tessellation_control_shader: None,
        tessellation_evaluation_shader: None,
        geometry_shader: None,
        fragment_shader: &std::fs::read_to_string(format!("src/shaders/{frag_shader}.frag")).unwrap(),
        transform_feedback_varyings: None,
        outputs_srgb: false,
        uses_point_size: false,
    }).unwrap()
}

#[derive(Copy, Clone)]
pub struct ModelVertex {
    pub position: Vec3<f32>,
    pub normal: Vec3<f32>,
    pub uv: Vec2<f32>,
}
glium::implement_vertex!(ModelVertex, position, normal, uv);

pub type ModelIndex = u16;

